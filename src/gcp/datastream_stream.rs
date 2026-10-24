use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DatastreamStreamData {
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
    customer_managed_encryption_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_state: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    stream_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backfill_all: std::option::Option<std::vec::Vec<DatastreamStreamBackfillAllEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backfill_none: std::option::Option<std::vec::Vec<DatastreamStreamBackfillNoneEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_config: std::option::Option<std::vec::Vec<DatastreamStreamDestinationConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_config: std::option::Option<std::vec::Vec<DatastreamStreamSourceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DatastreamStreamTimeoutsEl>,
    dynamic: DatastreamStreamDynamic,
}

struct DatastreamStream_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatastreamStreamData>,
}

#[derive(Clone)]
pub struct DatastreamStream(std::rc::Rc<DatastreamStream_>);

impl DatastreamStream {
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
        "Set the field `customer_managed_encryption_key`.\nA reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data\nwill be encrypted using an internal Stream-specific encryption key provisioned through KMS."]
    pub fn set_customer_managed_encryption_key(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().customer_managed_encryption_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `desired_state`.\nDesired state of the Stream. Set this field to 'RUNNING' to start the stream, and 'PAUSED' to pause the stream."]
    pub fn set_desired_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().desired_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `backfill_all`.\n"]
    pub fn set_backfill_all(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStreamBackfillAllEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backfill_all = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backfill_all = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `backfill_none`.\n"]
    pub fn set_backfill_none(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStreamBackfillNoneEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().backfill_none = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.backfill_none = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_config`.\n"]
    pub fn set_destination_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStreamDestinationConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_config`.\n"]
    pub fn set_source_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStreamSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DatastreamStreamTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `customer_managed_encryption_key` after provisioning.\nA reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data\nwill be encrypted using an internal Stream-specific encryption key provisioned through KMS."]
    pub fn customer_managed_encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption_key", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Stream. Set this field to 'RUNNING' to start the stream, and 'PAUSED' to pause the stream."]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this stream is located in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe stream's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the stream."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_id` after provisioning.\nThe stream identifier."]
    pub fn stream_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backfill_all` after provisioning.\n"]
    pub fn backfill_all(&self) -> terrars::ListRef<DatastreamStreamBackfillAllElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backfill_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backfill_none` after provisioning.\n"]
    pub fn backfill_none(&self) -> terrars::ListRef<DatastreamStreamBackfillNoneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backfill_none", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<DatastreamStreamDestinationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_config` after provisioning.\n"]
    pub fn source_config(&self) -> terrars::ListRef<DatastreamStreamSourceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamStreamTimeoutsElRef {
        DatastreamStreamTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DatastreamStream {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatastreamStream { }

impl terrars::ToListMappable for DatastreamStream {
    type O = terrars::ListRef<DatastreamStreamRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatastreamStream_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_datastream_stream".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatastreamStream {
    pub tf_id: std::string::String,
    #[doc = "Display name."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the location this stream is located in."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The stream identifier."]
    pub stream_id: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream {
    pub fn build(self, stack: &mut terrars::Stack) -> DatastreamStream {
        let out = DatastreamStream(std::rc::Rc::new(DatastreamStream_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatastreamStreamData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                customer_managed_encryption_key: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                stream_id: self.stream_id,
                backfill_all: core::default::Default::default(),
                backfill_none: core::default::Default::default(),
                destination_config: core::default::Default::default(),
                source_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatastreamStreamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatastreamStreamRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `customer_managed_encryption_key` after provisioning.\nA reference to a KMS encryption key. If provided, it will be used to encrypt the data. If left blank, data\nwill be encrypted using an internal Stream-specific encryption key provisioned through KMS."]
    pub fn customer_managed_encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_managed_encryption_key", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Stream. Set this field to 'RUNNING' to start the stream, and 'PAUSED' to pause the stream."]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `labels` after provisioning.\nLabels.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this stream is located in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe stream's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe state of the stream."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_id` after provisioning.\nThe stream identifier."]
    pub fn stream_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backfill_all` after provisioning.\n"]
    pub fn backfill_all(&self) -> terrars::ListRef<DatastreamStreamBackfillAllElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backfill_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backfill_none` after provisioning.\n"]
    pub fn backfill_none(&self) -> terrars::ListRef<DatastreamStreamBackfillNoneElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backfill_none", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<DatastreamStreamDestinationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_config` after provisioning.\n"]
    pub fn source_config(&self) -> terrars::ListRef<DatastreamStreamSourceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamStreamTimeoutsElRef {
        DatastreamStreamTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ordinal_position: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_key: std::option::Option<terrars::PrimField<bool>>,
}

impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    #[doc = "Set the field `collation`.\nColumn collation."]
    pub fn set_collation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_type`.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."]
    pub fn set_ordinal_position(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ordinal_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."]
    pub fn set_primary_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.primary_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {}

impl BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
            collation: core::default::Default::default(),
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
            nullable: core::default::Default::default(),
            ordinal_position: core::default::Default::default(),
            primary_key: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collation` after provisioning.\nColumn collation."]
    pub fn collation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."]
    pub fn primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElDynamic {
    mysql_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_columns: std
    ::option
    ::Option<
        std::vec::Vec<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl>,
    >,
    dynamic: DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElDynamic,
}

impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
    #[doc = "Set the field `mysql_columns`.\n"]
    pub fn set_mysql_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
    type O =
        terrars::BlockAssignable<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl {
            table: self.table,
            mysql_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_columns` after provisioning.\n"]
    pub fn mysql_columns(
        &self,
    ) -> terrars
    ::ListRef<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElDynamic {
    mysql_tables: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_tables: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl>>,
    dynamic: DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElDynamic,
}

impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
    #[doc = "Set the field `mysql_tables`.\n"]
    pub fn set_mysql_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
    type O = terrars::BlockAssignable<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
    #[doc = "Database name."]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl {
            database: self.database,
            mysql_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_tables` after provisioning.\n"]
    pub fn mysql_tables(
        &self,
    ) -> terrars::ListRef<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElMysqlTablesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElDynamic {
    mysql_databases: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl>>,
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_databases: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl>>,
    dynamic: DatastreamStreamBackfillAllElMysqlExcludedObjectsElDynamic,
}

impl DatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
    #[doc = "Set the field `mysql_databases`.\n"]
    pub fn set_mysql_databases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_databases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_databases = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
    type O = terrars::BlockAssignable<DatastreamStreamBackfillAllElMysqlExcludedObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsEl {}

impl BuildDatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsEl {
            mysql_databases: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef {
        DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mysql_databases` after provisioning.\n"]
    pub fn mysql_databases(
        &self,
    ) -> terrars::ListRef<DatastreamStreamBackfillAllElMysqlExcludedObjectsElMysqlDatabasesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_databases", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_type`.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {}

impl BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `encoding` after provisioning.\nColumn encoding."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }

    #[doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."]
    pub fn precision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."]
    pub fn primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElDynamic {
    oracle_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_columns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl>,
    >,
    dynamic: DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElDynamic,
}

impl DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
    #[doc = "Set the field `oracle_columns`.\n"]
    pub fn set_oracle_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
    type O =
        terrars::BlockAssignable<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl {
            table: self.table,
            oracle_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_columns` after provisioning.\n"]
    pub fn oracle_columns(
        &self,
    ) -> terrars
    ::ListRef<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElDynamic {
    oracle_tables: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_tables: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl>>,
    dynamic: DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElDynamic,
}

impl DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
    #[doc = "Set the field `oracle_tables`.\n"]
    pub fn set_oracle_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
    type O = terrars::BlockAssignable<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
    #[doc = "Schema name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl {
            schema: self.schema,
            oracle_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_tables` after provisioning.\n"]
    pub fn oracle_tables(
        &self,
    ) -> terrars::ListRef<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElOracleTablesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElOracleExcludedObjectsElDynamic {
    oracle_schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl>>,
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_schemas: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl>>,
    dynamic: DatastreamStreamBackfillAllElOracleExcludedObjectsElDynamic,
}

impl DatastreamStreamBackfillAllElOracleExcludedObjectsEl {
    #[doc = "Set the field `oracle_schemas`.\n"]
    pub fn set_oracle_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamBackfillAllElOracleExcludedObjectsEl {
    type O = terrars::BlockAssignable<DatastreamStreamBackfillAllElOracleExcludedObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElOracleExcludedObjectsEl {}

impl BuildDatastreamStreamBackfillAllElOracleExcludedObjectsEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElOracleExcludedObjectsEl {
        DatastreamStreamBackfillAllElOracleExcludedObjectsEl {
            oracle_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElOracleExcludedObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamBackfillAllElOracleExcludedObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElOracleExcludedObjectsElRef {
        DatastreamStreamBackfillAllElOracleExcludedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElOracleExcludedObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oracle_schemas` after provisioning.\n"]
    pub fn oracle_schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStreamBackfillAllElOracleExcludedObjectsElOracleSchemasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_schemas", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ordinal_position: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_key: std::option::Option<terrars::PrimField<bool>>,
}

impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_type`.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."]
    pub fn set_ordinal_position(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ordinal_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."]
    pub fn set_primary_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.primary_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {}

impl BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
            nullable: core::default::Default::default(),
            ordinal_position: core::default::Default::default(),
            primary_key: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }

    #[doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."]
    pub fn precision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."]
    pub fn primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic {
    postgresql_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_columns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
        >,
    >,
    dynamic: DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic,
}

impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    #[doc = "Set the field `postgresql_columns`.\n"]
    pub fn set_postgresql_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl {
            table: self.table,
            postgresql_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_columns` after provisioning.\n"]
    pub fn postgresql_columns(
        &self,
    ) -> terrars
    ::ListRef<
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElDynamic {
    postgresql_tables: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_tables: std
    ::option
    ::Option<
        std::vec::Vec<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl>,
    >,
    dynamic: DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElDynamic,
}

impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
    #[doc = "Set the field `postgresql_tables`.\n"]
    pub fn set_postgresql_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
    type O = terrars::BlockAssignable<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
    #[doc = "Database name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl {
            schema: self.schema,
            postgresql_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nDatabase name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_tables` after provisioning.\n"]
    pub fn postgresql_tables(
        &self,
    ) -> terrars
    ::ListRef<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElPostgresqlTablesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElDynamic {
    postgresql_schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl>>,
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_schemas: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl>>,
    dynamic: DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElDynamic,
}

impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
    #[doc = "Set the field `postgresql_schemas`.\n"]
    pub fn set_postgresql_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
    type O = terrars::BlockAssignable<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {}

impl BuildDatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
    pub fn build(self) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl {
            postgresql_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef {
        DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `postgresql_schemas` after provisioning.\n"]
    pub fn postgresql_schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElPostgresqlSchemasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_schemas", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamBackfillAllElDynamic {
    mysql_excluded_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamBackfillAllElMysqlExcludedObjectsEl>>,
    oracle_excluded_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamBackfillAllElOracleExcludedObjectsEl>>,
    postgresql_excluded_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl>>,
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillAllEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_excluded_objects: std::option::Option<std::vec::Vec<DatastreamStreamBackfillAllElMysqlExcludedObjectsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_excluded_objects: std::option::Option<std::vec::Vec<DatastreamStreamBackfillAllElOracleExcludedObjectsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_excluded_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl>>,
    dynamic: DatastreamStreamBackfillAllElDynamic,
}

impl DatastreamStreamBackfillAllEl {
    #[doc = "Set the field `mysql_excluded_objects`.\n"]
    pub fn set_mysql_excluded_objects(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStreamBackfillAllElMysqlExcludedObjectsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_excluded_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_excluded_objects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oracle_excluded_objects`.\n"]
    pub fn set_oracle_excluded_objects(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStreamBackfillAllElOracleExcludedObjectsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_excluded_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_excluded_objects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postgresql_excluded_objects`.\n"]
    pub fn set_postgresql_excluded_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_excluded_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_excluded_objects = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamBackfillAllEl {
    type O = terrars::BlockAssignable<DatastreamStreamBackfillAllEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillAllEl {}

impl BuildDatastreamStreamBackfillAllEl {
    pub fn build(self) -> DatastreamStreamBackfillAllEl {
        DatastreamStreamBackfillAllEl {
            mysql_excluded_objects: core::default::Default::default(),
            oracle_excluded_objects: core::default::Default::default(),
            postgresql_excluded_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamBackfillAllElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamBackfillAllElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStreamBackfillAllElRef {
        DatastreamStreamBackfillAllElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillAllElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mysql_excluded_objects` after provisioning.\n"]
    pub fn mysql_excluded_objects(&self) -> terrars::ListRef<DatastreamStreamBackfillAllElMysqlExcludedObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_excluded_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_excluded_objects` after provisioning.\n"]
    pub fn oracle_excluded_objects(&self) -> terrars::ListRef<DatastreamStreamBackfillAllElOracleExcludedObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_excluded_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_excluded_objects` after provisioning.\n"]
    pub fn postgresql_excluded_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStreamBackfillAllElPostgresqlExcludedObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_excluded_objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamBackfillNoneEl {}

impl DatastreamStreamBackfillNoneEl { }

impl terrars::ToListMappable for DatastreamStreamBackfillNoneEl {
    type O = terrars::BlockAssignable<DatastreamStreamBackfillNoneEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamBackfillNoneEl {}

impl BuildDatastreamStreamBackfillNoneEl {
    pub fn build(self) -> DatastreamStreamBackfillNoneEl {
        DatastreamStreamBackfillNoneEl {}
    }
}

pub struct DatastreamStreamBackfillNoneElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamBackfillNoneElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStreamBackfillNoneElRef {
        DatastreamStreamBackfillNoneElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamBackfillNoneElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl {
    dataset_id: terrars::PrimField<std::string::String>,
}

impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl { }

impl terrars::ToListMappable for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl {
    type O =
        terrars
        ::BlockAssignable<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl {
    #[doc = "Dataset ID in the format projects/{project}/datasets/{dataset_id} or\n{project}:{dataset_id}"]
    pub dataset_id: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl {
    pub fn build(self) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl {
            dataset_id: self.dataset_id,
        }
    }
}

pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nDataset ID in the format projects/{project}/datasets/{dataset_id} or\n{project}:{dataset_id}"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
}

impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl {
    #[doc =
        "Set the field `dataset_id_prefix`.\nIf supplied, every created dataset will have its name prefixed by the provided value.\nThe prefix and name will be separated by an underscore. i.e. _."]
    pub fn set_dataset_id_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_name`.\nDescribes the Cloud KMS encryption key that will be used to protect destination BigQuery\ntable. The BigQuery Service Account associated with your project requires access to this\nencryption key. i.e. projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{cryptoKey}.\nSee https://cloud.google.com/bigquery/docs/customer-managed-encryption for more information."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl {
    #[doc =
        "The geographic location where the dataset should reside.\nSee https://cloud.google.com/bigquery/docs/locations for supported locations."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl {
    pub fn build(
        self,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl {
            dataset_id_prefix: core::default::Default::default(),
            kms_key_name: core::default::Default::default(),
            location: self.location,
        }
    }
}

pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id_prefix` after provisioning.\nIf supplied, every created dataset will have its name prefixed by the provided value.\nThe prefix and name will be separated by an underscore. i.e. _."]
    pub fn dataset_id_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nDescribes the Cloud KMS encryption key that will be used to protect destination BigQuery\ntable. The BigQuery Service Account associated with your project requires access to this\nencryption key. i.e. projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{cryptoKey}.\nSee https://cloud.google.com/bigquery/docs/customer-managed-encryption for more information."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the dataset should reside.\nSee https://cloud.google.com/bigquery/docs/locations for supported locations."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDynamic {
    dataset_template: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_template: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl,
        >,
    >,
    dynamic: DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDynamic,
}

impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl {
    #[doc = "Set the field `dataset_template`.\n"]
    pub fn set_dataset_template(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataset_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset_template = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl {
    type O =
        terrars
        ::BlockAssignable<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl {}

impl BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl {
    pub fn build(self) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl {
            dataset_template: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_template` after provisioning.\n"]
    pub fn dataset_template(
        &self,
    ) -> terrars
    ::ListRef<
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElDatasetTemplateElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset_template", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElDynamic {
    single_target_dataset: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl>,
    >,
    source_hierarchy_datasets: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_freshness: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_target_dataset: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_hierarchy_datasets: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl>>,
    dynamic: DatastreamStreamDestinationConfigElBigqueryDestinationConfigElDynamic,
}

impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
    #[doc =
        "Set the field `data_freshness`.\nThe guaranteed data freshness (in seconds) when querying tables created by the stream.\nEditing this field will only affect new tables created in the future, but existing tables\nwill not be impacted. Lower values mean that queries will return fresher data, but may result in higher cost.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn set_data_freshness(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_freshness = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `single_target_dataset`.\n"]
    pub fn set_single_target_dataset(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.single_target_dataset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.single_target_dataset = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_hierarchy_datasets`.\n"]
    pub fn set_source_hierarchy_datasets(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_hierarchy_datasets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_hierarchy_datasets = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
    type O = terrars::BlockAssignable<DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {}

impl BuildDatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
    pub fn build(self) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl {
            data_freshness: core::default::Default::default(),
            single_target_dataset: core::default::Default::default(),
            source_hierarchy_datasets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef {
        DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_freshness` after provisioning.\nThe guaranteed data freshness (in seconds) when querying tables created by the stream.\nEditing this field will only affect new tables created in the future, but existing tables\nwill not be impacted. Lower values mean that queries will return fresher data, but may result in higher cost.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn data_freshness(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_freshness", self.base))
    }

    #[doc = "Get a reference to the value of field `single_target_dataset` after provisioning.\n"]
    pub fn single_target_dataset(
        &self,
    ) -> terrars::ListRef<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSingleTargetDatasetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_target_dataset", self.base))
    }

    #[doc = "Get a reference to the value of field `source_hierarchy_datasets` after provisioning.\n"]
    pub fn source_hierarchy_datasets(
        &self,
    ) -> terrars::ListRef<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElSourceHierarchyDatasetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_hierarchy_datasets", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {}

impl DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl { }

impl terrars::ToListMappable for DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {
    type O = terrars::BlockAssignable<DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {}

impl BuildDatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {
    pub fn build(self) -> DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {
        DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl {}
    }
}

pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef {
        DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_file_format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
    #[doc =
        "Set the field `compression`.\nCompression of the loaded JSON file. Possible values: [\"NO_COMPRESSION\", \"GZIP\"]"]
    pub fn set_compression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schema_file_format`.\nThe schema file format along JSON data files. Possible values: [\"NO_SCHEMA_FILE\", \"AVRO_SCHEMA_FILE\"]"]
    pub fn set_schema_file_format(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.schema_file_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
    type O = terrars::BlockAssignable<DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {}

impl BuildDatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
    pub fn build(self) -> DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
        DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl {
            compression: core::default::Default::default(),
            schema_file_format: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef {
        DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compression` after provisioning.\nCompression of the loaded JSON file. Possible values: [\"NO_COMPRESSION\", \"GZIP\"]"]
    pub fn compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema_file_format` after provisioning.\nThe schema file format along JSON data files. Possible values: [\"NO_SCHEMA_FILE\", \"AVRO_SCHEMA_FILE\"]"]
    pub fn schema_file_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_file_format", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamDestinationConfigElGcsDestinationConfigElDynamic {
    avro_file_format: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl>>,
    json_file_format: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl>>,
}

#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_rotation_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_rotation_mb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro_file_format: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_file_format: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl>>,
    dynamic: DatastreamStreamDestinationConfigElGcsDestinationConfigElDynamic,
}

impl DatastreamStreamDestinationConfigElGcsDestinationConfigEl {
    #[doc =
        "Set the field `file_rotation_interval`.\nThe maximum duration for which new events are added before a file is closed and a new file is created.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn set_file_rotation_interval(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.file_rotation_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_rotation_mb`.\nThe maximum file size to be saved in the bucket."]
    pub fn set_file_rotation_mb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.file_rotation_mb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\nPath inside the Cloud Storage bucket to write data to."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `avro_file_format`.\n"]
    pub fn set_avro_file_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.avro_file_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.avro_file_format = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json_file_format`.\n"]
    pub fn set_json_file_format(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json_file_format = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json_file_format = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamDestinationConfigElGcsDestinationConfigEl {
    type O = terrars::BlockAssignable<DatastreamStreamDestinationConfigElGcsDestinationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamDestinationConfigElGcsDestinationConfigEl {}

impl BuildDatastreamStreamDestinationConfigElGcsDestinationConfigEl {
    pub fn build(self) -> DatastreamStreamDestinationConfigElGcsDestinationConfigEl {
        DatastreamStreamDestinationConfigElGcsDestinationConfigEl {
            file_rotation_interval: core::default::Default::default(),
            file_rotation_mb: core::default::Default::default(),
            path: core::default::Default::default(),
            avro_file_format: core::default::Default::default(),
            json_file_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamDestinationConfigElGcsDestinationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamDestinationConfigElGcsDestinationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamDestinationConfigElGcsDestinationConfigElRef {
        DatastreamStreamDestinationConfigElGcsDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamDestinationConfigElGcsDestinationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `file_rotation_interval` after provisioning.\nThe maximum duration for which new events are added before a file is closed and a new file is created.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn file_rotation_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_rotation_interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_rotation_mb` after provisioning.\nThe maximum file size to be saved in the bucket."]
    pub fn file_rotation_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_rotation_mb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath inside the Cloud Storage bucket to write data to."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `avro_file_format` after provisioning.\n"]
    pub fn avro_file_format(
        &self,
    ) -> terrars::ListRef<DatastreamStreamDestinationConfigElGcsDestinationConfigElAvroFileFormatElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro_file_format", self.base))
    }

    #[doc = "Get a reference to the value of field `json_file_format` after provisioning.\n"]
    pub fn json_file_format(
        &self,
    ) -> terrars::ListRef<DatastreamStreamDestinationConfigElGcsDestinationConfigElJsonFileFormatElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_file_format", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamDestinationConfigElDynamic {
    bigquery_destination_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl>>,
    gcs_destination_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamDestinationConfigElGcsDestinationConfigEl>>,
}

#[derive(Serialize)]
pub struct DatastreamStreamDestinationConfigEl {
    destination_connection_profile: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_destination_config: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_destination_config: std::option::Option<std::vec::Vec<DatastreamStreamDestinationConfigElGcsDestinationConfigEl>>,
    dynamic: DatastreamStreamDestinationConfigElDynamic,
}

impl DatastreamStreamDestinationConfigEl {
    #[doc = "Set the field `bigquery_destination_config`.\n"]
    pub fn set_bigquery_destination_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStreamDestinationConfigElBigqueryDestinationConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_destination_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_destination_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_destination_config`.\n"]
    pub fn set_gcs_destination_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStreamDestinationConfigElGcsDestinationConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_destination_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_destination_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamDestinationConfigEl {
    type O = terrars::BlockAssignable<DatastreamStreamDestinationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamDestinationConfigEl {
    #[doc =
        "Destination connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}"]
    pub destination_connection_profile: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamDestinationConfigEl {
    pub fn build(self) -> DatastreamStreamDestinationConfigEl {
        DatastreamStreamDestinationConfigEl {
            destination_connection_profile: self.destination_connection_profile,
            bigquery_destination_config: core::default::Default::default(),
            gcs_destination_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamDestinationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamDestinationConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStreamDestinationConfigElRef {
        DatastreamStreamDestinationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamDestinationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `destination_connection_profile` after provisioning.\nDestination connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}"]
    pub fn destination_connection_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_connection_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `bigquery_destination_config` after provisioning.\n"]
    pub fn bigquery_destination_config(
        &self,
    ) -> terrars::ListRef<DatastreamStreamDestinationConfigElBigqueryDestinationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_destination_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_destination_config` after provisioning.\n"]
    pub fn gcs_destination_config(
        &self,
    ) -> terrars::ListRef<DatastreamStreamDestinationConfigElGcsDestinationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_destination_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ordinal_position: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_key: std::option::Option<terrars::PrimField<bool>>,
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    #[doc = "Set the field `collation`.\nColumn collation."]
    pub fn set_collation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_type`.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."]
    pub fn set_ordinal_position(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ordinal_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."]
    pub fn set_primary_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.primary_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {}

impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
            collation: core::default::Default::default(),
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
            nullable: core::default::Default::default(),
            ordinal_position: core::default::Default::default(),
            primary_key: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collation` after provisioning.\nColumn collation."]
    pub fn collation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."]
    pub fn primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElDynamic {
    mysql_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_columns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl,
        >,
    >,
    dynamic: DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElDynamic,
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl {
    #[doc = "Set the field `mysql_columns`.\n"]
    pub fn set_mysql_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl {
            table: self.table,
            mysql_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_columns` after provisioning.\n"]
    pub fn mysql_columns(
        &self,
    ) -> terrars
    ::ListRef<
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElDynamic {
    mysql_tables: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_tables: std
    ::option
    ::Option<
        std::vec::Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElDynamic,
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
    #[doc = "Set the field `mysql_tables`.\n"]
    pub fn set_mysql_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
    type O =
        terrars::BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
    #[doc = "Database name."]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl {
            database: self.database,
            mysql_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_tables` after provisioning.\n"]
    pub fn mysql_tables(
        &self,
    ) -> terrars
    ::ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElMysqlTablesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElDynamic {
    mysql_databases: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_databases: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl>>,
    dynamic: DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElDynamic,
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
    #[doc = "Set the field `mysql_databases`.\n"]
    pub fn set_mysql_databases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_databases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_databases = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {}

impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl {
            mysql_databases: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mysql_databases` after provisioning.\n"]
    pub fn mysql_databases(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElMysqlDatabasesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_databases", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ordinal_position: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_key: std::option::Option<terrars::PrimField<bool>>,
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    #[doc = "Set the field `collation`.\nColumn collation."]
    pub fn set_collation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_type`.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."]
    pub fn set_ordinal_position(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ordinal_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."]
    pub fn set_primary_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.primary_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {}

impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl {
            collation: core::default::Default::default(),
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
            nullable: core::default::Default::default(),
            ordinal_position: core::default::Default::default(),
            primary_key: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collation` after provisioning.\nColumn collation."]
    pub fn collation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collation", self.base))
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe MySQL data type. Full data types list can be found here:\nhttps://dev.mysql.com/doc/refman/8.0/en/data-types.html"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."]
    pub fn primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElDynamic {
    mysql_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_columns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl,
        >,
    >,
    dynamic: DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElDynamic,
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl {
    #[doc = "Set the field `mysql_columns`.\n"]
    pub fn set_mysql_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl {
            table: self.table,
            mysql_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_columns` after provisioning.\n"]
    pub fn mysql_columns(
        &self,
    ) -> terrars
    ::ListRef<
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElMysqlColumnsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElDynamic {
    mysql_tables: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_tables: std
    ::option
    ::Option<
        std::vec::Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElDynamic,
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
    #[doc = "Set the field `mysql_tables`.\n"]
    pub fn set_mysql_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
    type O =
        terrars::BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
    #[doc = "Database name."]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl {
            database: self.database,
            mysql_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_tables` after provisioning.\n"]
    pub fn mysql_tables(
        &self,
    ) -> terrars
    ::ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElMysqlTablesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElDynamic {
    mysql_databases: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_databases: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl>>,
    dynamic: DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElDynamic,
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
    #[doc = "Set the field `mysql_databases`.\n"]
    pub fn set_mysql_databases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_databases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_databases = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {}

impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl {
            mysql_databases: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mysql_databases` after provisioning.\n"]
    pub fn mysql_databases(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElMysqlDatabasesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_databases", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElMysqlSourceConfigElDynamic {
    exclude_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl>>,
    include_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl>>,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElMysqlSourceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_backfill_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_cdc_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl>>,
    dynamic: DatastreamStreamSourceConfigElMysqlSourceConfigElDynamic,
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigEl {
    #[doc =
        "Set the field `max_concurrent_backfill_tasks`.\nMaximum number of concurrent backfill tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn set_max_concurrent_backfill_tasks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_backfill_tasks = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_concurrent_cdc_tasks`.\nMaximum number of concurrent CDC tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn set_max_concurrent_cdc_tasks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_cdc_tasks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_objects`.\n"]
    pub fn set_exclude_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_objects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include_objects`.\n"]
    pub fn set_include_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_objects = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElMysqlSourceConfigEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElMysqlSourceConfigEl {}

impl BuildDatastreamStreamSourceConfigElMysqlSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElMysqlSourceConfigEl {
        DatastreamStreamSourceConfigElMysqlSourceConfigEl {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            max_concurrent_cdc_tasks: core::default::Default::default(),
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElMysqlSourceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElMysqlSourceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElMysqlSourceConfigElRef {
        DatastreamStreamSourceConfigElMysqlSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElMysqlSourceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nMaximum number of concurrent backfill tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn max_concurrent_backfill_tasks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_backfill_tasks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_cdc_tasks` after provisioning.\nMaximum number of concurrent CDC tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn max_concurrent_cdc_tasks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_cdc_tasks", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElExcludeObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElIncludeObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {}

impl DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl { }

impl terrars::ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {}

impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl {}
    }
}

pub struct DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_type`.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {}

impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `encoding` after provisioning.\nColumn encoding."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }

    #[doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."]
    pub fn precision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."]
    pub fn primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElDynamic {
    oracle_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_columns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl,
        >,
    >,
    dynamic: DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElDynamic,
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl {
    #[doc = "Set the field `oracle_columns`.\n"]
    pub fn set_oracle_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl {
            table: self.table,
            oracle_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_columns` after provisioning.\n"]
    pub fn oracle_columns(
        &self,
    ) -> terrars
    ::ListRef<
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElDynamic {
    oracle_tables: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_tables: std
    ::option
    ::Option<
        std::vec::Vec<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElDynamic,
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
    #[doc = "Set the field `oracle_tables`.\n"]
    pub fn set_oracle_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
    type O =
        terrars::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
    #[doc = "Schema name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl {
            schema: self.schema,
            oracle_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_tables` after provisioning.\n"]
    pub fn oracle_tables(
        &self,
    ) -> terrars
    ::ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElOracleTablesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElDynamic {
    oracle_schemas: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_schemas: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl>>,
    dynamic: DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElDynamic,
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
    #[doc = "Set the field `oracle_schemas`.\n"]
    pub fn set_oracle_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {}

impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl {
            oracle_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oracle_schemas` after provisioning.\n"]
    pub fn oracle_schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElOracleSchemasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_schemas", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_type`.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {}

impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl {
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe Oracle data type. Full data types list can be found here:\nhttps://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Data-Types.html"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `encoding` after provisioning.\nColumn encoding."]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }

    #[doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."]
    pub fn precision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."]
    pub fn primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElDynamic {
    oracle_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_columns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl,
        >,
    >,
    dynamic: DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElDynamic,
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl {
    #[doc = "Set the field `oracle_columns`.\n"]
    pub fn set_oracle_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl {
            table: self.table,
            oracle_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_columns` after provisioning.\n"]
    pub fn oracle_columns(
        &self,
    ) -> terrars
    ::ListRef<
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElOracleColumnsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElDynamic {
    oracle_tables: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_tables: std
    ::option
    ::Option<
        std::vec::Vec<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElDynamic,
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
    #[doc = "Set the field `oracle_tables`.\n"]
    pub fn set_oracle_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
    type O =
        terrars::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
    #[doc = "Schema name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl {
            schema: self.schema,
            oracle_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_tables` after provisioning.\n"]
    pub fn oracle_tables(
        &self,
    ) -> terrars
    ::ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElOracleTablesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElDynamic {
    oracle_schemas: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_schemas: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl>>,
    dynamic: DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElDynamic,
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
    #[doc = "Set the field `oracle_schemas`.\n"]
    pub fn set_oracle_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {}

impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl {
            oracle_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oracle_schemas` after provisioning.\n"]
    pub fn oracle_schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElOracleSchemasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_schemas", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {}

impl DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl { }

impl terrars::ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {}

impl BuildDatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {
        DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl {}
    }
}

pub struct DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElOracleSourceConfigElDynamic {
    drop_large_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl>>,
    exclude_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl>>,
    include_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl>>,
    stream_large_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl>>,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElOracleSourceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_backfill_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_cdc_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop_large_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_large_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl>>,
    dynamic: DatastreamStreamSourceConfigElOracleSourceConfigElDynamic,
}

impl DatastreamStreamSourceConfigElOracleSourceConfigEl {
    #[doc =
        "Set the field `max_concurrent_backfill_tasks`.\nMaximum number of concurrent backfill tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn set_max_concurrent_backfill_tasks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_backfill_tasks = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_concurrent_cdc_tasks`.\nMaximum number of concurrent CDC tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn set_max_concurrent_cdc_tasks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_cdc_tasks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drop_large_objects`.\n"]
    pub fn set_drop_large_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.drop_large_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.drop_large_objects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude_objects`.\n"]
    pub fn set_exclude_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_objects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include_objects`.\n"]
    pub fn set_include_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_objects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stream_large_objects`.\n"]
    pub fn set_stream_large_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stream_large_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stream_large_objects = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElOracleSourceConfigEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElOracleSourceConfigEl {}

impl BuildDatastreamStreamSourceConfigElOracleSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElOracleSourceConfigEl {
        DatastreamStreamSourceConfigElOracleSourceConfigEl {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            max_concurrent_cdc_tasks: core::default::Default::default(),
            drop_large_objects: core::default::Default::default(),
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            stream_large_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElOracleSourceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElOracleSourceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElOracleSourceConfigElRef {
        DatastreamStreamSourceConfigElOracleSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElOracleSourceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nMaximum number of concurrent backfill tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn max_concurrent_backfill_tasks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_backfill_tasks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_cdc_tasks` after provisioning.\nMaximum number of concurrent CDC tasks. The number should be non negative.\nIf not set (or set to 0), the system's default value will be used."]
    pub fn max_concurrent_cdc_tasks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_cdc_tasks", self.base))
    }

    #[doc = "Get a reference to the value of field `drop_large_objects` after provisioning.\n"]
    pub fn drop_large_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElDropLargeObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.drop_large_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElExcludeObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElIncludeObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_large_objects` after provisioning.\n"]
    pub fn stream_large_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElStreamLargeObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stream_large_objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ordinal_position: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_key: std::option::Option<terrars::PrimField<bool>>,
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_type`.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."]
    pub fn set_ordinal_position(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ordinal_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."]
    pub fn set_primary_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.primary_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {}

impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
            nullable: core::default::Default::default(),
            ordinal_position: core::default::Default::default(),
            primary_key: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }

    #[doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."]
    pub fn precision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."]
    pub fn primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic {
    postgresql_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_columns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
        >,
    >,
    dynamic: DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic,
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    #[doc = "Set the field `postgresql_columns`.\n"]
    pub fn set_postgresql_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
            table: self.table,
            postgresql_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_columns` after provisioning.\n"]
    pub fn postgresql_columns(
        &self,
    ) -> terrars
    ::ListRef<
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElDynamic {
    postgresql_tables: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_tables: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl,
        >,
    >,
    dynamic: DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElDynamic,
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl {
    #[doc = "Set the field `postgresql_tables`.\n"]
    pub fn set_postgresql_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl {
    type O =
        terrars
        ::BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl {
    #[doc = "Database name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl {
            schema: self.schema,
            postgresql_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nDatabase name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_tables` after provisioning.\n"]
    pub fn postgresql_tables(
        &self,
    ) -> terrars
    ::ListRef<
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElDynamic {
    postgresql_schemas: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_schemas: std
    ::option
    ::Option<
        std::vec::Vec<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElDynamic,
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {
    #[doc = "Set the field `postgresql_schemas`.\n"]
    pub fn set_postgresql_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {}

impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl {
            postgresql_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `postgresql_schemas` after provisioning.\n"]
    pub fn postgresql_schemas(
        &self,
    ) -> terrars
    ::ListRef<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElPostgresqlSchemasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_schemas", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ordinal_position: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_key: std::option::Option<terrars::PrimField<bool>>,
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_type`.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\nWhether or not the column can accept a null value."]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ordinal_position`.\nThe ordinal position of the column in the table."]
    pub fn set_ordinal_position(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ordinal_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_key`.\nWhether or not the column represents a primary key."]
    pub fn set_primary_key(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.primary_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {}

impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl {
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
            nullable: core::default::Default::default(),
            ordinal_position: core::default::Default::default(),
            primary_key: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe PostgreSQL data type. Full data types list can be found here:\nhttps://www.postgresql.org/docs/current/datatype.html"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc = "Get a reference to the value of field `length` after provisioning.\nColumn length."]
    pub fn length(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.length", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nullable` after provisioning.\nWhether or not the column can accept a null value."]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }

    #[doc = "Get a reference to the value of field `precision` after provisioning.\nColumn precision."]
    pub fn precision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_key` after provisioning.\nWhether or not the column represents a primary key."]
    pub fn primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_key", self.base))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\nColumn scale."]
    pub fn scale(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic {
    postgresql_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_columns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
        >,
    >,
    dynamic: DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElDynamic,
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    #[doc = "Set the field `postgresql_columns`.\n"]
    pub fn set_postgresql_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
    pub fn build(
        self,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl {
            table: self.table,
            postgresql_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_columns` after provisioning.\n"]
    pub fn postgresql_columns(
        &self,
    ) -> terrars
    ::ListRef<
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElPostgresqlColumnsElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElDynamic {
    postgresql_tables: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_tables: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl,
        >,
    >,
    dynamic: DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElDynamic,
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl {
    #[doc = "Set the field `postgresql_tables`.\n"]
    pub fn set_postgresql_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl {
    type O =
        terrars
        ::BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl {
    #[doc = "Database name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl {
            schema: self.schema,
            postgresql_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nDatabase name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_tables` after provisioning.\n"]
    pub fn postgresql_tables(
        &self,
    ) -> terrars
    ::ListRef<
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElPostgresqlTablesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElDynamic {
    postgresql_schemas: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_schemas: std
    ::option
    ::Option<
        std::vec::Vec<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl>,
    >,
    dynamic: DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElDynamic,
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {
    #[doc = "Set the field `postgresql_schemas`.\n"]
    pub fn set_postgresql_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {}

impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl {
            postgresql_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `postgresql_schemas` after provisioning.\n"]
    pub fn postgresql_schemas(
        &self,
    ) -> terrars
    ::ListRef<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElPostgresqlSchemasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_schemas", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElDynamic {
    exclude_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl>>,
    include_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl>>,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_backfill_tasks: std::option::Option<terrars::PrimField<f64>>,
    publication: terrars::PrimField<std::string::String>,
    replication_slot: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl>>,
    dynamic: DatastreamStreamSourceConfigElPostgresqlSourceConfigElDynamic,
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
    #[doc =
        "Set the field `max_concurrent_backfill_tasks`.\nMaximum number of concurrent backfill tasks. The number should be non\nnegative. If not set (or set to 0), the system's default value will be used."]
    pub fn set_max_concurrent_backfill_tasks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_backfill_tasks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_objects`.\n"]
    pub fn set_exclude_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_objects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include_objects`.\n"]
    pub fn set_include_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_objects = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
    #[doc =
        "The name of the publication that includes the set of all tables\nthat are defined in the stream's include_objects."]
    pub publication: terrars::PrimField<std::string::String>,
    #[doc = "The name of the logical replication slot that's configured with\nthe pgoutput plugin."]
    pub replication_slot: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigEl {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            publication: self.publication,
            replication_slot: self.replication_slot,
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef {
        DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nMaximum number of concurrent backfill tasks. The number should be non\nnegative. If not set (or set to 0), the system's default value will be used."]
    pub fn max_concurrent_backfill_tasks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_backfill_tasks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `publication` after provisioning.\nThe name of the publication that includes the set of all tables\nthat are defined in the stream's include_objects."]
    pub fn publication(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publication", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replication_slot` after provisioning.\nThe name of the logical replication slot that's configured with\nthe pgoutput plugin."]
    pub fn replication_slot(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_slot", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElPostgresqlSourceConfigElExcludeObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElPostgresqlSourceConfigElIncludeObjectsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_objects", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamSourceConfigElDynamic {
    mysql_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigElMysqlSourceConfigEl>>,
    oracle_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigElOracleSourceConfigEl>>,
    postgresql_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigElPostgresqlSourceConfigEl>>,
}

#[derive(Serialize)]
pub struct DatastreamStreamSourceConfigEl {
    source_connection_profile: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_source_config: std::option::Option<std::vec::Vec<DatastreamStreamSourceConfigElMysqlSourceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_source_config: std::option::Option<std::vec::Vec<DatastreamStreamSourceConfigElOracleSourceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_source_config: std::option::Option<std::vec::Vec<DatastreamStreamSourceConfigElPostgresqlSourceConfigEl>>,
    dynamic: DatastreamStreamSourceConfigElDynamic,
}

impl DatastreamStreamSourceConfigEl {
    #[doc = "Set the field `mysql_source_config`.\n"]
    pub fn set_mysql_source_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStreamSourceConfigElMysqlSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_source_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oracle_source_config`.\n"]
    pub fn set_oracle_source_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStreamSourceConfigElOracleSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_source_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postgresql_source_config`.\n"]
    pub fn set_postgresql_source_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStreamSourceConfigElPostgresqlSourceConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_source_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStreamSourceConfigEl {
    type O = terrars::BlockAssignable<DatastreamStreamSourceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamSourceConfigEl {
    #[doc =
        "Source connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}"]
    pub source_connection_profile: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStreamSourceConfigEl {
    pub fn build(self) -> DatastreamStreamSourceConfigEl {
        DatastreamStreamSourceConfigEl {
            source_connection_profile: self.source_connection_profile,
            mysql_source_config: core::default::Default::default(),
            oracle_source_config: core::default::Default::default(),
            postgresql_source_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamSourceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamSourceConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStreamSourceConfigElRef {
        DatastreamStreamSourceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamSourceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `source_connection_profile` after provisioning.\nSource connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}"]
    pub fn source_connection_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_connection_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_source_config` after provisioning.\n"]
    pub fn mysql_source_config(&self) -> terrars::ListRef<DatastreamStreamSourceConfigElMysqlSourceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_source_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_source_config` after provisioning.\n"]
    pub fn oracle_source_config(&self) -> terrars::ListRef<DatastreamStreamSourceConfigElOracleSourceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_source_config", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_source_config` after provisioning.\n"]
    pub fn postgresql_source_config(
        &self,
    ) -> terrars::ListRef<DatastreamStreamSourceConfigElPostgresqlSourceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_source_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStreamTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStreamTimeoutsEl {
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

impl terrars::ToListMappable for DatastreamStreamTimeoutsEl {
    type O = terrars::BlockAssignable<DatastreamStreamTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStreamTimeoutsEl {}

impl BuildDatastreamStreamTimeoutsEl {
    pub fn build(self) -> DatastreamStreamTimeoutsEl {
        DatastreamStreamTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DatastreamStreamTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStreamTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStreamTimeoutsElRef {
        DatastreamStreamTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStreamTimeoutsElRef {
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
struct DatastreamStreamDynamic {
    backfill_all: std::option::Option<terrars::DynamicBlock<DatastreamStreamBackfillAllEl>>,
    backfill_none: std::option::Option<terrars::DynamicBlock<DatastreamStreamBackfillNoneEl>>,
    destination_config: std::option::Option<terrars::DynamicBlock<DatastreamStreamDestinationConfigEl>>,
    source_config: std::option::Option<terrars::DynamicBlock<DatastreamStreamSourceConfigEl>>,
}
