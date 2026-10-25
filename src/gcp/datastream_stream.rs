#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DatastreamStreamData_ {
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
    create_without_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_encryption_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
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
    backfill_all: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backfill_none: std::option::Option<std::vec::Vec<DatastreamStream_BackfillNone>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_config: std::option::Option<std::vec::Vec<DatastreamStream_DestinationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_sets: std::option::Option<std::vec::Vec<DatastreamStream_RuleSets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_config: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DatastreamStream_Timeouts>,
    dynamic: DatastreamStreamDynamic,
}

struct DatastreamStream_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatastreamStreamData_>,
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

    #[doc = "Set the field `create_without_validation`.\nCreate the stream without validating it."]
    pub fn set_create_without_validation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_without_validation = std::option::Option::Some(v.into());
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `desired_state`.\nDesired state of the Stream. Set this field to 'RUNNING' to start the stream,\n'NOT_STARTED' to create the stream without starting and 'PAUSED' to pause\nthe stream from a 'RUNNING' state.\nPossible values: NOT_STARTED, RUNNING, PAUSED. Default: NOT_STARTED"]
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_BackfillNone>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_DestinationConfig>>,
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

    #[doc = "Set the field `rule_sets`.\n"]
    pub fn set_rule_sets(self, v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_RuleSets>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_sets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_sets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_config`.\n"]
    pub fn set_source_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DatastreamStream_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_without_validation` after provisioning.\nCreate the stream without validating it."]
    pub fn create_without_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_without_validation", self.extract_ref()))
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Stream. Set this field to 'RUNNING' to start the stream,\n'NOT_STARTED' to create the stream without starting and 'PAUSED' to pause\nthe stream from a 'RUNNING' state.\nPossible values: NOT_STARTED, RUNNING, PAUSED. Default: NOT_STARTED"]
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
    pub fn backfill_all(&self) -> terrars::ListRef<DatastreamStream_BackfillAllRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backfill_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backfill_none` after provisioning.\n"]
    pub fn backfill_none(&self) -> terrars::ListRef<DatastreamStream_BackfillNoneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backfill_none", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<DatastreamStream_DestinationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_sets` after provisioning.\n"]
    pub fn rule_sets(&self) -> terrars::ListRef<DatastreamStream_RuleSetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_sets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_config` after provisioning.\n"]
    pub fn source_config(&self) -> terrars::ListRef<DatastreamStream_SourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamStream_TimeoutsRef {
        DatastreamStream_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DatastreamStreamData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                create_without_validation: core::default::Default::default(),
                customer_managed_encryption_key: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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
                rule_sets: core::default::Default::default(),
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
        "Get a reference to the value of field `create_without_validation` after provisioning.\nCreate the stream without validating it."]
    pub fn create_without_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_without_validation", self.extract_ref()))
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Stream. Set this field to 'RUNNING' to start the stream,\n'NOT_STARTED' to create the stream without starting and 'PAUSED' to pause\nthe stream from a 'RUNNING' state.\nPossible values: NOT_STARTED, RUNNING, PAUSED. Default: NOT_STARTED"]
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
    pub fn backfill_all(&self) -> terrars::ListRef<DatastreamStream_BackfillAllRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backfill_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `backfill_none` after provisioning.\n"]
    pub fn backfill_none(&self) -> terrars::ListRef<DatastreamStream_BackfillNoneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backfill_none", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_config` after provisioning.\n"]
    pub fn destination_config(&self) -> terrars::ListRef<DatastreamStream_DestinationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_sets` after provisioning.\n"]
    pub fn rule_sets(&self) -> terrars::ListRef<DatastreamStream_RuleSetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_sets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_config` after provisioning.\n"]
    pub fn source_config(&self) -> terrars::ListRef<DatastreamStream_SourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatastreamStream_TimeoutsRef {
        DatastreamStream_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStreamDynamic {
    backfill_all: std::option::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll>>,
    backfill_none: std::option::Option<terrars::DynamicBlock<DatastreamStream_BackfillNone>>,
    destination_config: std::option::Option<terrars::DynamicBlock<DatastreamStream_DestinationConfig>>,
    rule_sets: std::option::Option<terrars::DynamicBlock<DatastreamStream_RuleSets>>,
    source_config: std::option::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields {
    #[doc = "Set the field `field`.\nField name."]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields {
    type O =
        terrars::BlockAssignable<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields {}

impl BuildDatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields {
    pub fn build(self) -> DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields {
        DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields {
            field: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_FieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_FieldsRef {
        DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_FieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\nField name."]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_CollectionsDynamic {
    fields: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections {
    collection: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields>>,
    dynamic: DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_CollectionsDynamic,
}

impl DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections {
    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections {
    #[doc = "Collection name."]
    pub collection: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections {
    pub fn build(self) -> DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections {
        DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections {
            collection: self.collection,
            fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_CollectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_CollectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_CollectionsRef {
        DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_CollectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_CollectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection` after provisioning.\nCollection name."]
    pub fn collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection", self.base))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_MongodbExcludedObjects_DatabasesDynamic {
    collections: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases {
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collections: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections>>,
    dynamic: DatastreamStream_BackfillAll_MongodbExcludedObjects_DatabasesDynamic,
}

impl DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases {
    #[doc = "Set the field `collections`.\n"]
    pub fn set_collections(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.collections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.collections = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_MongodbExcludedObjects_Databases {
    #[doc = "Database name."]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_MongodbExcludedObjects_Databases {
    pub fn build(self) -> DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases {
        DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases {
            database: self.database,
            collections: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_MongodbExcludedObjects_DatabasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_MongodbExcludedObjects_DatabasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_MongodbExcludedObjects_DatabasesRef {
        DatastreamStream_BackfillAll_MongodbExcludedObjects_DatabasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_MongodbExcludedObjects_DatabasesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `collections` after provisioning.\n"]
    pub fn collections(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_CollectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collections", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_MongodbExcludedObjectsDynamic {
    databases: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_MongodbExcludedObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    databases: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases>>,
    dynamic: DatastreamStream_BackfillAll_MongodbExcludedObjectsDynamic,
}

impl DatastreamStream_BackfillAll_MongodbExcludedObjects {
    #[doc = "Set the field `databases`.\n"]
    pub fn set_databases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.databases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.databases = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_MongodbExcludedObjects {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_MongodbExcludedObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_MongodbExcludedObjects {}

impl BuildDatastreamStream_BackfillAll_MongodbExcludedObjects {
    pub fn build(self) -> DatastreamStream_BackfillAll_MongodbExcludedObjects {
        DatastreamStream_BackfillAll_MongodbExcludedObjects {
            databases: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_MongodbExcludedObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_MongodbExcludedObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_MongodbExcludedObjectsRef {
        DatastreamStream_BackfillAll_MongodbExcludedObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_MongodbExcludedObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `databases` after provisioning.\n"]
    pub fn databases(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_MongodbExcludedObjects_DatabasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.databases", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumns {
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

impl DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumns {
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
::ToListMappable for DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumns {
    type O =
        terrars
        ::BlockAssignable<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumns {}

impl BuildDatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumns {
    pub fn build(self) -> DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumns {
        DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumns {
            collation: core::default::Default::default(),
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
            nullable: core::default::Default::default(),
            ordinal_position: core::default::Default::default(),
            primary_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumnsRef {
        DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumnsRef {
    #[allow(dead_code)]
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
struct DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTablesDynamic {
    mysql_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumns>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_columns: std
    ::option
    ::Option<
        std::vec::Vec<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumns>,
    >,
    dynamic: DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTablesDynamic,
}

impl DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables {
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
                            DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumns,
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

impl terrars::ToListMappable for DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables {
    pub fn build(self) -> DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables {
        DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables {
            table: self.table,
            mysql_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTablesRef {
        DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTablesRef {
    #[allow(dead_code)]
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
    ::ListRef<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables_MysqlColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabasesDynamic {
    mysql_tables: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases {
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_tables: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables>>,
    dynamic: DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabasesDynamic,
}

impl DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases {
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
                            DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTables,
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

impl terrars::ToListMappable for DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases {
    #[doc = "Database name."]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases {
    pub fn build(self) -> DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases {
        DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases {
            database: self.database,
            mysql_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabasesRef {
        DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabasesRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases_MysqlTablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_MysqlExcludedObjectsDynamic {
    mysql_databases: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_MysqlExcludedObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_databases: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases>>,
    dynamic: DatastreamStream_BackfillAll_MysqlExcludedObjectsDynamic,
}

impl DatastreamStream_BackfillAll_MysqlExcludedObjects {
    #[doc = "Set the field `mysql_databases`.\n"]
    pub fn set_mysql_databases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases>,
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

impl terrars::ToListMappable for DatastreamStream_BackfillAll_MysqlExcludedObjects {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_MysqlExcludedObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_MysqlExcludedObjects {}

impl BuildDatastreamStream_BackfillAll_MysqlExcludedObjects {
    pub fn build(self) -> DatastreamStream_BackfillAll_MysqlExcludedObjects {
        DatastreamStream_BackfillAll_MysqlExcludedObjects {
            mysql_databases: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_MysqlExcludedObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_MysqlExcludedObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_MysqlExcludedObjectsRef {
        DatastreamStream_BackfillAll_MysqlExcludedObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_MysqlExcludedObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mysql_databases` after provisioning.\n"]
    pub fn mysql_databases(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_databases", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumns {
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
::ToListMappable for DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumns {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumns,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumns {}

impl BuildDatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumns {
    pub fn build(
        self,
    ) -> DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumns {
        DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumns {
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumnsRef {
        DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumnsRef {
    #[allow(dead_code)]
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
struct DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTablesDynamic {
    oracle_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumns>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_columns: std
    ::option
    ::Option<
        std::vec::Vec<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumns>,
    >,
    dynamic: DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTablesDynamic,
}

impl DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables {
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
                            DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumns,
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

impl terrars::ToListMappable for DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables {
    pub fn build(self) -> DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables {
        DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables {
            table: self.table,
            oracle_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTablesRef {
        DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTablesRef {
    #[allow(dead_code)]
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
    ::ListRef<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables_OracleColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemasDynamic {
    oracle_tables: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_tables: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables>>,
    dynamic: DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemasDynamic,
}

impl DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas {
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
                            DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTables,
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

impl terrars::ToListMappable for DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas {
    #[doc = "Schema name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas {
    pub fn build(self) -> DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas {
        DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas {
            schema: self.schema,
            oracle_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemasRef {
        DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemasRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas_OracleTablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_OracleExcludedObjectsDynamic {
    oracle_schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_OracleExcludedObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_schemas: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas>>,
    dynamic: DatastreamStream_BackfillAll_OracleExcludedObjectsDynamic,
}

impl DatastreamStream_BackfillAll_OracleExcludedObjects {
    #[doc = "Set the field `oracle_schemas`.\n"]
    pub fn set_oracle_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas>,
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

impl terrars::ToListMappable for DatastreamStream_BackfillAll_OracleExcludedObjects {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_OracleExcludedObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_OracleExcludedObjects {}

impl BuildDatastreamStream_BackfillAll_OracleExcludedObjects {
    pub fn build(self) -> DatastreamStream_BackfillAll_OracleExcludedObjects {
        DatastreamStream_BackfillAll_OracleExcludedObjects {
            oracle_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_OracleExcludedObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_OracleExcludedObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_OracleExcludedObjectsRef {
        DatastreamStream_BackfillAll_OracleExcludedObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_OracleExcludedObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oracle_schemas` after provisioning.\n"]
    pub fn oracle_schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_schemas", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumns {
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

impl DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumns {
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
::ToListMappable for DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumns {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumns,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumns {}

impl BuildDatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumns {
    pub fn build(
        self,
    ) -> DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumns {
        DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumns {
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
            nullable: core::default::Default::default(),
            ordinal_position: core::default::Default::default(),
            primary_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumnsRef {
        DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumnsRef {
    #[allow(dead_code)]
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
struct DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTablesDynamic {
    postgresql_columns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumns,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_columns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumns,
        >,
    >,
    dynamic: DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTablesDynamic,
}

impl DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables {
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
                            DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumns,
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
::ToListMappable for DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables {
    type O =
        terrars
        ::BlockAssignable<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables {
    pub fn build(self) -> DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables {
        DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables {
            table: self.table,
            postgresql_columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTablesRef {
        DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTablesRef {
    #[allow(dead_code)]
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
        DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables_PostgresqlColumnsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemasDynamic {
    postgresql_tables: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_tables: std
    ::option
    ::Option<
        std::vec::Vec<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables>,
    >,
    dynamic: DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemasDynamic,
}

impl DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas {
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
                            DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTables,
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

impl terrars::ToListMappable for DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas {
    #[doc = "Database name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas {
    pub fn build(self) -> DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas {
        DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas {
            schema: self.schema,
            postgresql_tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemasRef {
        DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemasRef {
    #[allow(dead_code)]
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
    ::ListRef<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas_PostgresqlTablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_PostgresqlExcludedObjectsDynamic {
    postgresql_schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_PostgresqlExcludedObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_schemas: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas>>,
    dynamic: DatastreamStream_BackfillAll_PostgresqlExcludedObjectsDynamic,
}

impl DatastreamStream_BackfillAll_PostgresqlExcludedObjects {
    #[doc = "Set the field `postgresql_schemas`.\n"]
    pub fn set_postgresql_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas>,
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

impl terrars::ToListMappable for DatastreamStream_BackfillAll_PostgresqlExcludedObjects {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_PostgresqlExcludedObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_PostgresqlExcludedObjects {}

impl BuildDatastreamStream_BackfillAll_PostgresqlExcludedObjects {
    pub fn build(self) -> DatastreamStream_BackfillAll_PostgresqlExcludedObjects {
        DatastreamStream_BackfillAll_PostgresqlExcludedObjects {
            postgresql_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_PostgresqlExcludedObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_PostgresqlExcludedObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_PostgresqlExcludedObjectsRef {
        DatastreamStream_BackfillAll_PostgresqlExcludedObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_PostgresqlExcludedObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `postgresql_schemas` after provisioning.\n"]
    pub fn postgresql_schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_schemas", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_Fields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_Fields {
    #[doc = "Set the field `name`.\nField name."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_Fields {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_Fields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_Fields {}

impl BuildDatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_Fields {
    pub fn build(self) -> DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_Fields {
        DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_Fields {
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_FieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_FieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_FieldsRef {
        DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_FieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_FieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nField name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_SalesforceExcludedObjects_ObjectsDynamic {
    fields: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_Fields>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_Fields>>,
    dynamic: DatastreamStream_BackfillAll_SalesforceExcludedObjects_ObjectsDynamic,
}

impl DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects {
    #[doc = "Set the field `object_name`.\nName of object in Salesforce Org."]
    pub fn set_object_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_Fields>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects {}

impl BuildDatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects {
    pub fn build(self) -> DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects {
        DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects {
            object_name: core::default::Default::default(),
            fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_SalesforceExcludedObjects_ObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_SalesforceExcludedObjects_ObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_SalesforceExcludedObjects_ObjectsRef {
        DatastreamStream_BackfillAll_SalesforceExcludedObjects_ObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_SalesforceExcludedObjects_ObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `object_name` after provisioning.\nName of object in Salesforce Org."]
    pub fn object_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_name", self.base))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_SalesforceExcludedObjectsDynamic {
    objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_SalesforceExcludedObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    objects: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects>>,
    dynamic: DatastreamStream_BackfillAll_SalesforceExcludedObjectsDynamic,
}

impl DatastreamStream_BackfillAll_SalesforceExcludedObjects {
    #[doc = "Set the field `objects`.\n"]
    pub fn set_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.objects = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_SalesforceExcludedObjects {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_SalesforceExcludedObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_SalesforceExcludedObjects {}

impl BuildDatastreamStream_BackfillAll_SalesforceExcludedObjects {
    pub fn build(self) -> DatastreamStream_BackfillAll_SalesforceExcludedObjects {
        DatastreamStream_BackfillAll_SalesforceExcludedObjects {
            objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_SalesforceExcludedObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_SalesforceExcludedObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_SalesforceExcludedObjectsRef {
        DatastreamStream_BackfillAll_SalesforceExcludedObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_SalesforceExcludedObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `objects` after provisioning.\n"]
    pub fn objects(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_SalesforceExcludedObjects_ObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_Columns {
    column: terrars::PrimField<std::string::String>,
}

impl DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_Columns { }

impl terrars::ToListMappable for DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_Columns {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_Columns {
    #[doc = "Column name."]
    pub column: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_Columns {
    pub fn build(self) -> DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_Columns {
        DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_Columns { column: self.column }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_ColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_ColumnsRef {
        DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_ColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe Spanner data type. Full data types list can be found here:\nhttps://docs.cloud.google.com/spanner/docs/reference/standard-sql/data-types"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_primary_key` after provisioning.\nWhether the column is a primary key."]
    pub fn is_primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_primary_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_TablesDynamic {
    columns: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_Columns>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_Columns>>,
    dynamic: DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_TablesDynamic,
}

impl DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_Columns>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables {
    pub fn build(self) -> DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables {
        DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables {
            table: self.table,
            columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_TablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_TablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_TablesRef {
        DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_TablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_TablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_SpannerExcludedObjects_SchemasDynamic {
    tables: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tables: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables>>,
    dynamic: DatastreamStream_BackfillAll_SpannerExcludedObjects_SchemasDynamic,
}

impl DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas {
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_Tables>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas {
    #[doc = "Schema name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas {
    pub fn build(self) -> DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas {
        DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas {
            schema: self.schema,
            tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_SpannerExcludedObjects_SchemasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_SpannerExcludedObjects_SchemasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_SpannerExcludedObjects_SchemasRef {
        DatastreamStream_BackfillAll_SpannerExcludedObjects_SchemasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_SpannerExcludedObjects_SchemasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas_TablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_SpannerExcludedObjectsDynamic {
    schemas: std::option::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_SpannerExcludedObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schemas: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas>>,
    dynamic: DatastreamStream_BackfillAll_SpannerExcludedObjectsDynamic,
}

impl DatastreamStream_BackfillAll_SpannerExcludedObjects {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_SpannerExcludedObjects_Schemas>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_SpannerExcludedObjects {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_SpannerExcludedObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_SpannerExcludedObjects {}

impl BuildDatastreamStream_BackfillAll_SpannerExcludedObjects {
    pub fn build(self) -> DatastreamStream_BackfillAll_SpannerExcludedObjects {
        DatastreamStream_BackfillAll_SpannerExcludedObjects {
            schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_SpannerExcludedObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_SpannerExcludedObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_SpannerExcludedObjectsRef {
        DatastreamStream_BackfillAll_SpannerExcludedObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_SpannerExcludedObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_SpannerExcludedObjects_SchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_Columns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_Columns {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_type`.\nThe SQL Server data type. Full data types list can be found here:\nhttps://learn.microsoft.com/en-us/sql/t-sql/data-types/data-types-transact-sql?view=sql-server-ver16"]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_Columns {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_Columns {}

impl BuildDatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_Columns {
    pub fn build(self) -> DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_Columns {
        DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_Columns {
            column: core::default::Default::default(),
            data_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_ColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_ColumnsRef {
        DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_ColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe SQL Server data type. Full data types list can be found here:\nhttps://learn.microsoft.com/en-us/sql/t-sql/data-types/data-types-transact-sql?view=sql-server-ver16"]
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
struct DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_TablesDynamic {
    columns: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_Columns>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_Columns>>,
    dynamic: DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_TablesDynamic,
}

impl DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_Columns,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables {
    pub fn build(self) -> DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables {
        DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables {
            table: self.table,
            columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_TablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_TablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_TablesRef {
        DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_TablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_TablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_SqlServerExcludedObjects_SchemasDynamic {
    tables: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tables: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables>>,
    dynamic: DatastreamStream_BackfillAll_SqlServerExcludedObjects_SchemasDynamic,
}

impl DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas {
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_Tables>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas {
    #[doc = "Schema name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas {
    pub fn build(self) -> DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas {
        DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas {
            schema: self.schema,
            tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_SqlServerExcludedObjects_SchemasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_SqlServerExcludedObjects_SchemasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_SqlServerExcludedObjects_SchemasRef {
        DatastreamStream_BackfillAll_SqlServerExcludedObjects_SchemasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_SqlServerExcludedObjects_SchemasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas_TablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAll_SqlServerExcludedObjectsDynamic {
    schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll_SqlServerExcludedObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schemas: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas>>,
    dynamic: DatastreamStream_BackfillAll_SqlServerExcludedObjectsDynamic,
}

impl DatastreamStream_BackfillAll_SqlServerExcludedObjects {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll_SqlServerExcludedObjects {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll_SqlServerExcludedObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll_SqlServerExcludedObjects {}

impl BuildDatastreamStream_BackfillAll_SqlServerExcludedObjects {
    pub fn build(self) -> DatastreamStream_BackfillAll_SqlServerExcludedObjects {
        DatastreamStream_BackfillAll_SqlServerExcludedObjects {
            schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAll_SqlServerExcludedObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAll_SqlServerExcludedObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_BackfillAll_SqlServerExcludedObjectsRef {
        DatastreamStream_BackfillAll_SqlServerExcludedObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAll_SqlServerExcludedObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_SqlServerExcludedObjects_SchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_BackfillAllDynamic {
    mongodb_excluded_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_MongodbExcludedObjects>>,
    mysql_excluded_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_MysqlExcludedObjects>>,
    oracle_excluded_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_OracleExcludedObjects>>,
    postgresql_excluded_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_PostgresqlExcludedObjects>>,
    salesforce_excluded_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SalesforceExcludedObjects>>,
    spanner_excluded_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SpannerExcludedObjects>>,
    sql_server_excluded_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SqlServerExcludedObjects>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillAll {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mongodb_excluded_objects: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_MongodbExcludedObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_excluded_objects: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_MysqlExcludedObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_excluded_objects: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_OracleExcludedObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_excluded_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_PostgresqlExcludedObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce_excluded_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_SalesforceExcludedObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spanner_excluded_objects: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_SpannerExcludedObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_server_excluded_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_SqlServerExcludedObjects>>,
    dynamic: DatastreamStream_BackfillAllDynamic,
}

impl DatastreamStream_BackfillAll {
    #[doc = "Set the field `mongodb_excluded_objects`.\n"]
    pub fn set_mongodb_excluded_objects(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_MongodbExcludedObjects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mongodb_excluded_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mongodb_excluded_objects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mysql_excluded_objects`.\n"]
    pub fn set_mysql_excluded_objects(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_MysqlExcludedObjects>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_OracleExcludedObjects>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_PostgresqlExcludedObjects>>,
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

    #[doc = "Set the field `salesforce_excluded_objects`.\n"]
    pub fn set_salesforce_excluded_objects(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_SalesforceExcludedObjects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce_excluded_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce_excluded_objects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spanner_excluded_objects`.\n"]
    pub fn set_spanner_excluded_objects(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_SpannerExcludedObjects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spanner_excluded_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spanner_excluded_objects = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sql_server_excluded_objects`.\n"]
    pub fn set_sql_server_excluded_objects(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_SqlServerExcludedObjects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sql_server_excluded_objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_server_excluded_objects = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_BackfillAll {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillAll>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillAll {}

impl BuildDatastreamStream_BackfillAll {
    pub fn build(self) -> DatastreamStream_BackfillAll {
        DatastreamStream_BackfillAll {
            mongodb_excluded_objects: core::default::Default::default(),
            mysql_excluded_objects: core::default::Default::default(),
            oracle_excluded_objects: core::default::Default::default(),
            postgresql_excluded_objects: core::default::Default::default(),
            salesforce_excluded_objects: core::default::Default::default(),
            spanner_excluded_objects: core::default::Default::default(),
            sql_server_excluded_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillAllRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillAllRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStream_BackfillAllRef {
        DatastreamStream_BackfillAllRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillAllRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mongodb_excluded_objects` after provisioning.\n"]
    pub fn mongodb_excluded_objects(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_MongodbExcludedObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_excluded_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_excluded_objects` after provisioning.\n"]
    pub fn mysql_excluded_objects(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_MysqlExcludedObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_excluded_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_excluded_objects` after provisioning.\n"]
    pub fn oracle_excluded_objects(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_OracleExcludedObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_excluded_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_excluded_objects` after provisioning.\n"]
    pub fn postgresql_excluded_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_PostgresqlExcludedObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_excluded_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce_excluded_objects` after provisioning.\n"]
    pub fn salesforce_excluded_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_SalesforceExcludedObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce_excluded_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `spanner_excluded_objects` after provisioning.\n"]
    pub fn spanner_excluded_objects(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_SpannerExcludedObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spanner_excluded_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_server_excluded_objects` after provisioning.\n"]
    pub fn sql_server_excluded_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_SqlServerExcludedObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server_excluded_objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_BackfillNone {}

impl DatastreamStream_BackfillNone { }

impl terrars::ToListMappable for DatastreamStream_BackfillNone {
    type O = terrars::BlockAssignable<DatastreamStream_BackfillNone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_BackfillNone {}

impl BuildDatastreamStream_BackfillNone {
    pub fn build(self) -> DatastreamStream_BackfillNone {
        DatastreamStream_BackfillNone {}
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_BackfillNoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_BackfillNoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStream_BackfillNoneRef {
        DatastreamStream_BackfillNoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_BackfillNoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnly {}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnly { }

impl terrars::ToListMappable for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnly {
    type O = terrars::BlockAssignable<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnly>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnly {}

impl BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnly {
    pub fn build(self) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnly {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnly {}
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnlyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnlyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnlyRef {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnlyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnlyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfig {
    bucket: terrars::PrimField<std::string::String>,
    connection_name: terrars::PrimField<std::string::String>,
    file_format: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_path: std::option::Option<terrars::PrimField<std::string::String>>,
    table_format: terrars::PrimField<std::string::String>,
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfig {
    #[doc = "Set the field `root_path`.\nThe root path inside the Cloud Storage bucket."]
    pub fn set_root_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfig {
    type O = terrars::BlockAssignable<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfig {
    #[doc = "The Cloud Storage bucket name."]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = "The bigquery connection. Format: '{project}.{location}.{name}'"]
    pub connection_name: terrars::PrimField<std::string::String>,
    #[doc = "The file format."]
    pub file_format: terrars::PrimField<std::string::String>,
    #[doc = "The table format."]
    pub table_format: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfig {
    pub fn build(self) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfig {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfig {
            bucket: self.bucket,
            connection_name: self.connection_name,
            file_format: self.file_format,
            root_path: core::default::Default::default(),
            table_format: self.table_format,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfigRef {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\nThe Cloud Storage bucket name."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection_name` after provisioning.\nThe bigquery connection. Format: '{project}.{location}.{name}'"]
    pub fn connection_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_name", self.base))
    }

    #[doc = "Get a reference to the value of field `file_format` after provisioning.\nThe file format."]
    pub fn file_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `root_path` after provisioning.\nThe root path inside the Cloud Storage bucket."]
    pub fn root_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_path", self.base))
    }

    #[doc = "Get a reference to the value of field `table_format` after provisioning.\nThe table format."]
    pub fn table_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_format", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_Merge {}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_Merge { }

impl terrars::ToListMappable for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_Merge {
    type O = terrars::BlockAssignable<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_Merge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_Merge {}

impl BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_Merge {
    pub fn build(self) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_Merge {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_Merge {}
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_MergeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_MergeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_MergeRef {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_MergeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_MergeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDataset {
    dataset_id: terrars::PrimField<std::string::String>,
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDataset { }

impl terrars::ToListMappable for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDataset {
    type O =
        terrars::BlockAssignable<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDataset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDataset {
    #[doc = "Dataset ID in the format projects/{project}/datasets/{dataset_id} or\n{project}:{dataset_id}"]
    pub dataset_id: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDataset {
    pub fn build(self) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDataset {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDataset {
            dataset_id: self.dataset_id,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDatasetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDatasetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDatasetRef {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDatasetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDatasetRef {
    #[allow(dead_code)]
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
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplate {
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
::ToListMappable for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplate {
    type O =
        terrars
        ::BlockAssignable<
            DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplate,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplate {
    #[doc =
        "The geographic location where the dataset should reside.\nSee https://cloud.google.com/bigquery/docs/locations for supported locations."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplate {
    pub fn build(
        self,
    ) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplate {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplate {
            dataset_id_prefix: core::default::Default::default(),
            kms_key_name: core::default::Default::default(),
            location: self.location,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplateRef {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplateRef {
    #[allow(dead_code)]
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
struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasetsDynamic {
    dataset_template: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplate,
        >,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_template: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplate>,
    >,
    dynamic: DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasetsDynamic,
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets {
    #[doc =
        "Set the field `project_id`.\nOptional. The project id of the BigQuery dataset. If not specified, the project will be inferred from the stream resource."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

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
                            DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplate,
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

impl terrars::ToListMappable for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets {
    type O =
        terrars
        ::BlockAssignable<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets {}

impl BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets {
    pub fn build(self) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets {
            project_id: core::default::Default::default(),
            dataset_template: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasetsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasetsRef {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nOptional. The project id of the BigQuery dataset. If not specified, the project will be inferred from the stream resource."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `dataset_template` after provisioning.\n"]
    pub fn dataset_template(
        &self,
    ) -> terrars
    ::ListRef<
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets_DatasetTemplateRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset_template", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_DestinationConfig_BigqueryDestinationConfigDynamic {
    append_only: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnly>>,
    blmt_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfig>>,
    merge: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_Merge>>,
    single_target_dataset: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDataset>>,
    source_hierarchy_datasets: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_freshness: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    append_only: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnly>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blmt_config: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    merge: std::option::Option<std::vec::Vec<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_Merge>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_target_dataset: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDataset>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_hierarchy_datasets: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets>>,
    dynamic: DatastreamStream_DestinationConfig_BigqueryDestinationConfigDynamic,
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfig {
    #[doc =
        "Set the field `data_freshness`.\nThe guaranteed data freshness (in seconds) when querying tables created by the stream.\nEditing this field will only affect new tables created in the future, but existing tables\nwill not be impacted. Lower values mean that queries will return fresher data, but may result in higher cost.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn set_data_freshness(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_freshness = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `append_only`.\n"]
    pub fn set_append_only(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnly>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.append_only = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.append_only = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `blmt_config`.\n"]
    pub fn set_blmt_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.blmt_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.blmt_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `merge`.\n"]
    pub fn set_merge(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_Merge>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.merge = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.merge = std::option::Option::Some(d);
            },
        }
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
                            DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDataset,
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
                            DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasets,
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

impl terrars::ToListMappable for DatastreamStream_DestinationConfig_BigqueryDestinationConfig {
    type O = terrars::BlockAssignable<DatastreamStream_DestinationConfig_BigqueryDestinationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig {}

impl BuildDatastreamStream_DestinationConfig_BigqueryDestinationConfig {
    pub fn build(self) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfig {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfig {
            data_freshness: core::default::Default::default(),
            append_only: core::default::Default::default(),
            blmt_config: core::default::Default::default(),
            merge: core::default::Default::default(),
            single_target_dataset: core::default::Default::default(),
            source_hierarchy_datasets: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_DestinationConfig_BigqueryDestinationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_DestinationConfig_BigqueryDestinationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_DestinationConfig_BigqueryDestinationConfigRef {
        DatastreamStream_DestinationConfig_BigqueryDestinationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_DestinationConfig_BigqueryDestinationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_freshness` after provisioning.\nThe guaranteed data freshness (in seconds) when querying tables created by the stream.\nEditing this field will only affect new tables created in the future, but existing tables\nwill not be impacted. Lower values mean that queries will return fresher data, but may result in higher cost.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\". Defaults to 900s."]
    pub fn data_freshness(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_freshness", self.base))
    }

    #[doc = "Get a reference to the value of field `append_only` after provisioning.\n"]
    pub fn append_only(
        &self,
    ) -> terrars::ListRef<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_AppendOnlyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.append_only", self.base))
    }

    #[doc = "Get a reference to the value of field `blmt_config` after provisioning.\n"]
    pub fn blmt_config(
        &self,
    ) -> terrars::ListRef<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_BlmtConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blmt_config", self.base))
    }

    #[doc = "Get a reference to the value of field `merge` after provisioning.\n"]
    pub fn merge(&self) -> terrars::ListRef<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_MergeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.merge", self.base))
    }

    #[doc = "Get a reference to the value of field `single_target_dataset` after provisioning.\n"]
    pub fn single_target_dataset(
        &self,
    ) -> terrars::ListRef<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SingleTargetDatasetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_target_dataset", self.base))
    }

    #[doc = "Get a reference to the value of field `source_hierarchy_datasets` after provisioning.\n"]
    pub fn source_hierarchy_datasets(
        &self,
    ) -> terrars::ListRef<DatastreamStream_DestinationConfig_BigqueryDestinationConfig_SourceHierarchyDatasetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_hierarchy_datasets", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormat {}

impl DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormat { }

impl terrars::ToListMappable for DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormat {
    type O = terrars::BlockAssignable<DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormat {}

impl BuildDatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormat {
    pub fn build(self) -> DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormat {
        DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormat {}
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormatRef {
        DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormatRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormat {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_file_format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormat {
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

impl terrars::ToListMappable for DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormat {
    type O = terrars::BlockAssignable<DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormat>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormat {}

impl BuildDatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormat {
    pub fn build(self) -> DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormat {
        DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormat {
            compression: core::default::Default::default(),
            schema_file_format: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormatRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormatRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormatRef {
        DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormatRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormatRef {
    #[allow(dead_code)]
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
struct DatastreamStream_DestinationConfig_GcsDestinationConfigDynamic {
    avro_file_format: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormat>>,
    json_file_format: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormat>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_DestinationConfig_GcsDestinationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_rotation_interval: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_rotation_mb: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro_file_format: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormat>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_file_format: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormat>>,
    dynamic: DatastreamStream_DestinationConfig_GcsDestinationConfigDynamic,
}

impl DatastreamStream_DestinationConfig_GcsDestinationConfig {
    #[doc =
        "Set the field `file_rotation_interval`.\nThe maximum duration for which new events are added before a file is closed and a new file is created.\nValues within the range of 15-60 seconds are allowed.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
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
                        ::BlockAssignable<DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormat>,
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
                        ::BlockAssignable<DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormat>,
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

impl terrars::ToListMappable for DatastreamStream_DestinationConfig_GcsDestinationConfig {
    type O = terrars::BlockAssignable<DatastreamStream_DestinationConfig_GcsDestinationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_DestinationConfig_GcsDestinationConfig {}

impl BuildDatastreamStream_DestinationConfig_GcsDestinationConfig {
    pub fn build(self) -> DatastreamStream_DestinationConfig_GcsDestinationConfig {
        DatastreamStream_DestinationConfig_GcsDestinationConfig {
            file_rotation_interval: core::default::Default::default(),
            file_rotation_mb: core::default::Default::default(),
            path: core::default::Default::default(),
            avro_file_format: core::default::Default::default(),
            json_file_format: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_DestinationConfig_GcsDestinationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_DestinationConfig_GcsDestinationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_DestinationConfig_GcsDestinationConfigRef {
        DatastreamStream_DestinationConfig_GcsDestinationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_DestinationConfig_GcsDestinationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `file_rotation_interval` after provisioning.\nThe maximum duration for which new events are added before a file is closed and a new file is created.\nValues within the range of 15-60 seconds are allowed.\nA duration in seconds with up to nine fractional digits, terminated by 's'. Example: \"3.5s\"."]
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
    ) -> terrars::ListRef<DatastreamStream_DestinationConfig_GcsDestinationConfig_AvroFileFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro_file_format", self.base))
    }

    #[doc = "Get a reference to the value of field `json_file_format` after provisioning.\n"]
    pub fn json_file_format(
        &self,
    ) -> terrars::ListRef<DatastreamStream_DestinationConfig_GcsDestinationConfig_JsonFileFormatRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_file_format", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_DestinationConfigDynamic {
    bigquery_destination_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_DestinationConfig_BigqueryDestinationConfig>>,
    gcs_destination_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_DestinationConfig_GcsDestinationConfig>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_DestinationConfig {
    destination_connection_profile: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_destination_config: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_DestinationConfig_BigqueryDestinationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_destination_config: std::option::Option<std::vec::Vec<DatastreamStream_DestinationConfig_GcsDestinationConfig>>,
    dynamic: DatastreamStream_DestinationConfigDynamic,
}

impl DatastreamStream_DestinationConfig {
    #[doc = "Set the field `bigquery_destination_config`.\n"]
    pub fn set_bigquery_destination_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStream_DestinationConfig_BigqueryDestinationConfig>>,
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
                    ::Into<terrars::BlockAssignable<DatastreamStream_DestinationConfig_GcsDestinationConfig>>,
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

impl terrars::ToListMappable for DatastreamStream_DestinationConfig {
    type O = terrars::BlockAssignable<DatastreamStream_DestinationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_DestinationConfig {
    #[doc =
        "Destination connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}"]
    pub destination_connection_profile: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_DestinationConfig {
    pub fn build(self) -> DatastreamStream_DestinationConfig {
        DatastreamStream_DestinationConfig {
            destination_connection_profile: self.destination_connection_profile,
            bigquery_destination_config: core::default::Default::default(),
            gcs_destination_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_DestinationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_DestinationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStream_DestinationConfigRef {
        DatastreamStream_DestinationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_DestinationConfigRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DatastreamStream_DestinationConfig_BigqueryDestinationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_destination_config", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_destination_config` after provisioning.\n"]
    pub fn gcs_destination_config(&self) -> terrars::ListRef<DatastreamStream_DestinationConfig_GcsDestinationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_destination_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_CustomizationRules_BigqueryClustering {
    columns: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DatastreamStream_RuleSets_CustomizationRules_BigqueryClustering { }

impl terrars::ToListMappable for DatastreamStream_RuleSets_CustomizationRules_BigqueryClustering {
    type O = terrars::BlockAssignable<DatastreamStream_RuleSets_CustomizationRules_BigqueryClustering>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_CustomizationRules_BigqueryClustering {
    #[doc = "Column names to set as clustering columns."]
    pub columns: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDatastreamStream_RuleSets_CustomizationRules_BigqueryClustering {
    pub fn build(self) -> DatastreamStream_RuleSets_CustomizationRules_BigqueryClustering {
        DatastreamStream_RuleSets_CustomizationRules_BigqueryClustering { columns: self.columns }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_CustomizationRules_BigqueryClusteringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_CustomizationRules_BigqueryClusteringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_CustomizationRules_BigqueryClusteringRef {
        DatastreamStream_RuleSets_CustomizationRules_BigqueryClusteringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_CustomizationRules_BigqueryClusteringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `columns` after provisioning.\nColumn names to set as clustering columns."]
    pub fn columns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partitioning_time_granularity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartition {
    #[doc =
        "Set the field `partitioning_time_granularity`.\nPartition granularity. Possible values: [\"PARTITIONING_TIME_GRANULARITY_UNSPECIFIED\", \"PARTITIONING_TIME_GRANULARITY_HOUR\", \"PARTITIONING_TIME_GRANULARITY_DAY\", \"PARTITIONING_TIME_GRANULARITY_MONTH\", \"PARTITIONING_TIME_GRANULARITY_YEAR\"]"]
    pub fn set_partitioning_time_granularity(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.partitioning_time_granularity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartition {
    type O =
        terrars
        ::BlockAssignable<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartition {}

impl BuildDatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartition {
    pub fn build(self) -> DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartition {
        DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartition {
            partitioning_time_granularity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartitionRef {
        DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `partitioning_time_granularity` after provisioning.\nPartition granularity. Possible values: [\"PARTITIONING_TIME_GRANULARITY_UNSPECIFIED\", \"PARTITIONING_TIME_GRANULARITY_HOUR\", \"PARTITIONING_TIME_GRANULARITY_DAY\", \"PARTITIONING_TIME_GRANULARITY_MONTH\", \"PARTITIONING_TIME_GRANULARITY_YEAR\"]"]
    pub fn partitioning_time_granularity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partitioning_time_granularity", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartition {
    column: terrars::PrimField<std::string::String>,
    end: terrars::PrimField<f64>,
    interval: terrars::PrimField<f64>,
    start: terrars::PrimField<f64>,
}

impl DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartition { }

impl terrars
::ToListMappable for DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartition {
    type O =
        terrars
        ::BlockAssignable<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartition {
    #[doc = "The partitioning column."]
    pub column: terrars::PrimField<std::string::String>,
    #[doc = "The ending value for range partitioning (exclusive)."]
    pub end: terrars::PrimField<f64>,
    #[doc = "The interval of each range within the partition."]
    pub interval: terrars::PrimField<f64>,
    #[doc = "The starting value for range partitioning (inclusive)."]
    pub start: terrars::PrimField<f64>,
}

impl BuildDatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartition {
    pub fn build(self) -> DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartition {
        DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartition {
            column: self.column,
            end: self.end,
            interval: self.interval,
            start: self.start,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartitionRef {
        DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nThe partitioning column."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `end` after provisioning.\nThe ending value for range partitioning (exclusive)."]
    pub fn end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interval` after provisioning.\nThe interval of each range within the partition."]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start` after provisioning.\nThe starting value for range partitioning (inclusive)."]
    pub fn start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartition {
    column: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partitioning_time_granularity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartition {
    #[doc =
        "Set the field `partitioning_time_granularity`.\nPartition granularity. Possible values: [\"PARTITIONING_TIME_GRANULARITY_UNSPECIFIED\", \"PARTITIONING_TIME_GRANULARITY_HOUR\", \"PARTITIONING_TIME_GRANULARITY_DAY\", \"PARTITIONING_TIME_GRANULARITY_MONTH\", \"PARTITIONING_TIME_GRANULARITY_YEAR\"]"]
    pub fn set_partitioning_time_granularity(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.partitioning_time_granularity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartition {
    type O =
        terrars
        ::BlockAssignable<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartition {
    #[doc = "The partitioning column."]
    pub column: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartition {
    pub fn build(self) -> DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartition {
        DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartition {
            column: self.column,
            partitioning_time_granularity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartitionRef {
        DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nThe partitioning column."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `partitioning_time_granularity` after provisioning.\nPartition granularity. Possible values: [\"PARTITIONING_TIME_GRANULARITY_UNSPECIFIED\", \"PARTITIONING_TIME_GRANULARITY_HOUR\", \"PARTITIONING_TIME_GRANULARITY_DAY\", \"PARTITIONING_TIME_GRANULARITY_MONTH\", \"PARTITIONING_TIME_GRANULARITY_YEAR\"]"]
    pub fn partitioning_time_granularity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partitioning_time_granularity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioningDynamic {
    ingestion_time_partition: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartition>,
    >,
    integer_range_partition: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartition>,
    >,
    time_unit_partition: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartition>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_partition_filter: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingestion_time_partition: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    integer_range_partition: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_unit_partition: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartition>>,
    dynamic: DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioningDynamic,
}

impl DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning {
    #[doc = "Set the field `require_partition_filter`.\nIf true, queries over the table require a partition filter."]
    pub fn set_require_partition_filter(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_partition_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingestion_time_partition`.\n"]
    pub fn set_ingestion_time_partition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ingestion_time_partition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ingestion_time_partition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `integer_range_partition`.\n"]
    pub fn set_integer_range_partition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.integer_range_partition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.integer_range_partition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_unit_partition`.\n"]
    pub fn set_time_unit_partition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_unit_partition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_unit_partition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning {
    type O = terrars::BlockAssignable<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning {}

impl BuildDatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning {
    pub fn build(self) -> DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning {
        DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning {
            require_partition_filter: core::default::Default::default(),
            ingestion_time_partition: core::default::Default::default(),
            integer_range_partition: core::default::Default::default(),
            time_unit_partition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioningRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioningRef {
        DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `require_partition_filter` after provisioning.\nIf true, queries over the table require a partition filter."]
    pub fn require_partition_filter(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_partition_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `ingestion_time_partition` after provisioning.\n"]
    pub fn ingestion_time_partition(
        &self,
    ) -> terrars
    ::ListRef<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IngestionTimePartitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingestion_time_partition", self.base))
    }

    #[doc = "Get a reference to the value of field `integer_range_partition` after provisioning.\n"]
    pub fn integer_range_partition(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_IntegerRangePartitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.integer_range_partition", self.base))
    }

    #[doc = "Get a reference to the value of field `time_unit_partition` after provisioning.\n"]
    pub fn time_unit_partition(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning_TimeUnitPartitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_unit_partition", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_RuleSets_CustomizationRulesDynamic {
    bigquery_clustering: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_RuleSets_CustomizationRules_BigqueryClustering>>,
    bigquery_partitioning: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_CustomizationRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_clustering: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_CustomizationRules_BigqueryClustering>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_partitioning: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning>>,
    dynamic: DatastreamStream_RuleSets_CustomizationRulesDynamic,
}

impl DatastreamStream_RuleSets_CustomizationRules {
    #[doc = "Set the field `bigquery_clustering`.\n"]
    pub fn set_bigquery_clustering(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStream_RuleSets_CustomizationRules_BigqueryClustering>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_clustering = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_clustering = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bigquery_partitioning`.\n"]
    pub fn set_bigquery_partitioning(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioning>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_partitioning = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_partitioning = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_RuleSets_CustomizationRules {
    type O = terrars::BlockAssignable<DatastreamStream_RuleSets_CustomizationRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_CustomizationRules {}

impl BuildDatastreamStream_RuleSets_CustomizationRules {
    pub fn build(self) -> DatastreamStream_RuleSets_CustomizationRules {
        DatastreamStream_RuleSets_CustomizationRules {
            bigquery_clustering: core::default::Default::default(),
            bigquery_partitioning: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_CustomizationRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_CustomizationRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_CustomizationRulesRef {
        DatastreamStream_RuleSets_CustomizationRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_CustomizationRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bigquery_clustering` after provisioning.\n"]
    pub fn bigquery_clustering(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_CustomizationRules_BigqueryClusteringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_clustering", self.base))
    }

    #[doc = "Get a reference to the value of field `bigquery_partitioning` after provisioning.\n"]
    pub fn bigquery_partitioning(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_CustomizationRules_BigqueryPartitioningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_partitioning", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifier {
    collection: terrars::PrimField<std::string::String>,
    database: terrars::PrimField<std::string::String>,
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifier { }

impl terrars::ToListMappable for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifier {
    type O =
        terrars::BlockAssignable<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifier {
    #[doc = "The MongoDB collection name."]
    pub collection: terrars::PrimField<std::string::String>,
    #[doc = "The MongoDB database name."]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifier {
    pub fn build(self) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifier {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifier {
            collection: self.collection,
            database: self.database,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifierRef {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection` after provisioning.\nThe MongoDB collection name."]
    pub fn collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection", self.base))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe MongoDB database name."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifier {
    database: terrars::PrimField<std::string::String>,
    table: terrars::PrimField<std::string::String>,
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifier { }

impl terrars::ToListMappable for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifier {
    type O = terrars::BlockAssignable<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifier {
    #[doc = "The database name."]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = "The table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifier {
    pub fn build(self) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifier {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifier {
            database: self.database,
            table: self.table,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifierRef {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nThe database name."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe table name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifier {
    schema: terrars::PrimField<std::string::String>,
    table: terrars::PrimField<std::string::String>,
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifier { }

impl terrars::ToListMappable for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifier {
    type O =
        terrars::BlockAssignable<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifier {
    #[doc = "The schema name."]
    pub schema: terrars::PrimField<std::string::String>,
    #[doc = "The table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifier {
    pub fn build(self) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifier {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifier {
            schema: self.schema,
            table: self.table,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifierRef {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nThe schema name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe table name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifier {
    schema: terrars::PrimField<std::string::String>,
    table: terrars::PrimField<std::string::String>,
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifier { }

impl terrars::ToListMappable for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifier {
    type O =
        terrars::BlockAssignable<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifier {
    #[doc = "The schema name."]
    pub schema: terrars::PrimField<std::string::String>,
    #[doc = "The table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifier {
    pub fn build(self) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifier {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifier {
            schema: self.schema,
            table: self.table,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifierRef {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nThe schema name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe table name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifier {
    object_name: terrars::PrimField<std::string::String>,
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifier { }

impl terrars::ToListMappable for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifier {
    type O =
        terrars::BlockAssignable<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifier {
    #[doc = "The Salesforce object name."]
    pub object_name: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifier {
    pub fn build(self) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifier {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifier {
            object_name: self.object_name,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifierRef {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `object_name` after provisioning.\nThe Salesforce object name."]
    pub fn object_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifier {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::PrimField<std::string::String>>,
    table: terrars::PrimField<std::string::String>,
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifier {
    #[doc = "Set the field `schema`.\nThe schema name."]
    pub fn set_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifier {
    type O =
        terrars::BlockAssignable<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifier {
    #[doc = "The table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifier {
    pub fn build(self) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifier {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifier {
            schema: core::default::Default::default(),
            table: self.table,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifierRef {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nThe schema name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe table name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifier {
    schema: terrars::PrimField<std::string::String>,
    table: terrars::PrimField<std::string::String>,
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifier { }

impl terrars::ToListMappable for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifier {
    type O =
        terrars::BlockAssignable<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifier {
    #[doc = "The schema name."]
    pub schema: terrars::PrimField<std::string::String>,
    #[doc = "The table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifier {
    pub fn build(self) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifier {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifier {
            schema: self.schema,
            table: self.table,
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifierRef {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nThe schema name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nThe table name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifierDynamic {
    mongodb_identifier: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifier>>,
    mysql_identifier: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifier>>,
    oracle_identifier: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifier>>,
    postgresql_identifier: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifier>,
    >,
    salesforce_identifier: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifier>,
    >,
    spanner_identifier: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifier>>,
    sql_server_identifier: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifier>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mongodb_identifier: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifier>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_identifier: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifier>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_identifier: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifier>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_identifier: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifier>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce_identifier: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifier>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spanner_identifier: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifier>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_server_identifier: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifier>>,
    dynamic: DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifierDynamic,
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier {
    #[doc = "Set the field `mongodb_identifier`.\n"]
    pub fn set_mongodb_identifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifier,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mongodb_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mongodb_identifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mysql_identifier`.\n"]
    pub fn set_mysql_identifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifier,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql_identifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oracle_identifier`.\n"]
    pub fn set_oracle_identifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifier,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle_identifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postgresql_identifier`.\n"]
    pub fn set_postgresql_identifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifier,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql_identifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce_identifier`.\n"]
    pub fn set_salesforce_identifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifier,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce_identifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spanner_identifier`.\n"]
    pub fn set_spanner_identifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifier,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spanner_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spanner_identifier = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sql_server_identifier`.\n"]
    pub fn set_sql_server_identifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifier,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sql_server_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_server_identifier = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier {
    type O = terrars::BlockAssignable<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier {}

impl BuildDatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier {
    pub fn build(self) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier {
            mongodb_identifier: core::default::Default::default(),
            mysql_identifier: core::default::Default::default(),
            oracle_identifier: core::default::Default::default(),
            postgresql_identifier: core::default::Default::default(),
            salesforce_identifier: core::default::Default::default(),
            spanner_identifier: core::default::Default::default(),
            sql_server_identifier: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifierRef {
        DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifierRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mongodb_identifier` after provisioning.\n"]
    pub fn mongodb_identifier(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MongodbIdentifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_identifier` after provisioning.\n"]
    pub fn mysql_identifier(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_MysqlIdentifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_identifier` after provisioning.\n"]
    pub fn oracle_identifier(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_OracleIdentifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_identifier` after provisioning.\n"]
    pub fn postgresql_identifier(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_PostgresqlIdentifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce_identifier` after provisioning.\n"]
    pub fn salesforce_identifier(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SalesforceIdentifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `spanner_identifier` after provisioning.\n"]
    pub fn spanner_identifier(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SpannerIdentifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spanner_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_server_identifier` after provisioning.\n"]
    pub fn sql_server_identifier(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier_SqlServerIdentifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_RuleSets_ObjectFilterDynamic {
    source_object_identifier: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets_ObjectFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_object_identifier: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier>>,
    dynamic: DatastreamStream_RuleSets_ObjectFilterDynamic,
}

impl DatastreamStream_RuleSets_ObjectFilter {
    #[doc = "Set the field `source_object_identifier`.\n"]
    pub fn set_source_object_identifier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifier>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_object_identifier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_object_identifier = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_RuleSets_ObjectFilter {
    type O = terrars::BlockAssignable<DatastreamStream_RuleSets_ObjectFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets_ObjectFilter {}

impl BuildDatastreamStream_RuleSets_ObjectFilter {
    pub fn build(self) -> DatastreamStream_RuleSets_ObjectFilter {
        DatastreamStream_RuleSets_ObjectFilter {
            source_object_identifier: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSets_ObjectFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSets_ObjectFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStream_RuleSets_ObjectFilterRef {
        DatastreamStream_RuleSets_ObjectFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSets_ObjectFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_object_identifier` after provisioning.\n"]
    pub fn source_object_identifier(
        &self,
    ) -> terrars::ListRef<DatastreamStream_RuleSets_ObjectFilter_SourceObjectIdentifierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_object_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_RuleSetsDynamic {
    customization_rules: std::option::Option<terrars::DynamicBlock<DatastreamStream_RuleSets_CustomizationRules>>,
    object_filter: std::option::Option<terrars::DynamicBlock<DatastreamStream_RuleSets_ObjectFilter>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_RuleSets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customization_rules: std::option::Option<std::vec::Vec<DatastreamStream_RuleSets_CustomizationRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_filter: std::option::Option<std::vec::Vec<DatastreamStream_RuleSets_ObjectFilter>>,
    dynamic: DatastreamStream_RuleSetsDynamic,
}

impl DatastreamStream_RuleSets {
    #[doc = "Set the field `customization_rules`.\n"]
    pub fn set_customization_rules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_RuleSets_CustomizationRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.customization_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.customization_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `object_filter`.\n"]
    pub fn set_object_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_RuleSets_ObjectFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.object_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.object_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_RuleSets {
    type O = terrars::BlockAssignable<DatastreamStream_RuleSets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_RuleSets {}

impl BuildDatastreamStream_RuleSets {
    pub fn build(self) -> DatastreamStream_RuleSets {
        DatastreamStream_RuleSets {
            customization_rules: core::default::Default::default(),
            object_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_RuleSetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_RuleSetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStream_RuleSetsRef {
        DatastreamStream_RuleSetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_RuleSetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customization_rules` after provisioning.\n"]
    pub fn customization_rules(&self) -> terrars::ListRef<DatastreamStream_RuleSets_CustomizationRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customization_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `object_filter` after provisioning.\n"]
    pub fn object_filter(&self) -> terrars::ListRef<DatastreamStream_RuleSets_ObjectFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_CollectionsDynamic {
    fields: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_Collections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields>>,
    dynamic: DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_CollectionsDynamic,
}

impl DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_Collections {
    #[doc = "Set the field `collection`.\nCollection name."]
    pub fn set_collection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_Fields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_Collections {
    type O =
        terrars
        ::BlockAssignable<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_Collections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_Collections {}

impl BuildDatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_Collections {
    pub fn build(self) -> DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_Collections {
        DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_Collections {
            collection: core::default::Default::default(),
            fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_CollectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_CollectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_CollectionsRef {
        DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_CollectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_CollectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection` after provisioning.\nCollection name."]
    pub fn collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection", self.base))
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\n"]
    pub fn fields(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_MongodbExcludedObjects_Databases_Collections_FieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_DatabasesDynamic {
    collections: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_Collections>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collections: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_Collections>>,
    dynamic: DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_DatabasesDynamic,
}

impl DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases {
    #[doc = "Set the field `database`.\nDatabase name."]
    pub fn set_database(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collections`.\n"]
    pub fn set_collections(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_Collections,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.collections = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.collections = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases {}

impl BuildDatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases {
    pub fn build(self) -> DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases {
        DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases {
            database: core::default::Default::default(),
            collections: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_DatabasesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_DatabasesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_DatabasesRef {
        DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_DatabasesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_DatabasesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\nDatabase name."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `collections` after provisioning.\n"]
    pub fn collections(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases_CollectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collections", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjectsDynamic {
    databases: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    databases: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases>>,
    dynamic: DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects {
    #[doc = "Set the field `databases`.\n"]
    pub fn set_databases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.databases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.databases = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects {}

impl BuildDatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects {
        DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects {
            databases: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjectsRef {
        DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `databases` after provisioning.\n"]
    pub fn databases(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_DatabasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.databases", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjectsDynamic {
    databases: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    databases: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases>>,
    dynamic: DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjects {
    #[doc = "Set the field `databases`.\n"]
    pub fn set_databases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_Databases>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.databases = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.databases = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjects {}

impl BuildDatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjects {
        DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjects {
            databases: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjectsRef {
        DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `databases` after provisioning.\n"]
    pub fn databases(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects_DatabasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.databases", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_MongodbSourceConfigDynamic {
    exclude_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects>>,
    include_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjects>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_MongodbSourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_backfill_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_objects: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_objects: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjects>>,
    dynamic: DatastreamStream_SourceConfig_MongodbSourceConfigDynamic,
}

impl DatastreamStream_SourceConfig_MongodbSourceConfig {
    #[doc =
        "Set the field `max_concurrent_backfill_tasks`.\nOptional. Maximum number of concurrent backfill tasks. The number\nshould be non-negative and less than or equal to 50. If not set\n(or set to 0), the system''s default value is used"]
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
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjects>,
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
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjects>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_MongodbSourceConfig {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_MongodbSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_MongodbSourceConfig {}

impl BuildDatastreamStream_SourceConfig_MongodbSourceConfig {
    pub fn build(self) -> DatastreamStream_SourceConfig_MongodbSourceConfig {
        DatastreamStream_SourceConfig_MongodbSourceConfig {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_MongodbSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_MongodbSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_MongodbSourceConfigRef {
        DatastreamStream_SourceConfig_MongodbSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_MongodbSourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nOptional. Maximum number of concurrent backfill tasks. The number\nshould be non-negative and less than or equal to 50. If not set\n(or set to 0), the system''s default value is used"]
    pub fn max_concurrent_backfill_tasks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_backfill_tasks", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_MongodbSourceConfig_ExcludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_MongodbSourceConfig_IncludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPosition {}

impl DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPosition { }

impl terrars::ToListMappable for DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPosition {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPosition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPosition {}

impl BuildDatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPosition {
    pub fn build(self) -> DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPosition {
        DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPosition {}
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPositionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPositionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPositionRef {
        DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPositionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPositionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjectsDynamic {
    mysql_databases: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_databases: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases>>,
    dynamic: DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjects {
    #[doc = "Set the field `mysql_databases`.\n"]
    pub fn set_mysql_databases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjects {}

impl BuildDatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjects {
        DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjects {
            mysql_databases: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjectsRef {
        DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mysql_databases` after provisioning.\n"]
    pub fn mysql_databases(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_databases", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_MysqlSourceConfig_Gtid {}

impl DatastreamStream_SourceConfig_MysqlSourceConfig_Gtid { }

impl terrars::ToListMappable for DatastreamStream_SourceConfig_MysqlSourceConfig_Gtid {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_MysqlSourceConfig_Gtid>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_MysqlSourceConfig_Gtid {}

impl BuildDatastreamStream_SourceConfig_MysqlSourceConfig_Gtid {
    pub fn build(self) -> DatastreamStream_SourceConfig_MysqlSourceConfig_Gtid {
        DatastreamStream_SourceConfig_MysqlSourceConfig_Gtid {}
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_MysqlSourceConfig_GtidRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_MysqlSourceConfig_GtidRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_MysqlSourceConfig_GtidRef {
        DatastreamStream_SourceConfig_MysqlSourceConfig_GtidRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_MysqlSourceConfig_GtidRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjectsDynamic {
    mysql_databases: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_databases: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases>>,
    dynamic: DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjects {
    #[doc = "Set the field `mysql_databases`.\n"]
    pub fn set_mysql_databases(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabases>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjects {}

impl BuildDatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjects {
        DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjects {
            mysql_databases: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjectsRef {
        DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mysql_databases` after provisioning.\n"]
    pub fn mysql_databases(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_MysqlExcludedObjects_MysqlDatabasesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_databases", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_MysqlSourceConfigDynamic {
    binary_log_position: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPosition>>,
    exclude_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjects>>,
    gtid: std::option::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_MysqlSourceConfig_Gtid>>,
    include_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjects>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_MysqlSourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_backfill_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_cdc_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_log_position: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPosition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_objects: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gtid: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_MysqlSourceConfig_Gtid>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_objects: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjects>>,
    dynamic: DatastreamStream_SourceConfig_MysqlSourceConfigDynamic,
}

impl DatastreamStream_SourceConfig_MysqlSourceConfig {
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

    #[doc = "Set the field `binary_log_position`.\n"]
    pub fn set_binary_log_position(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPosition>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.binary_log_position = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.binary_log_position = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjects>>,
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

    #[doc = "Set the field `gtid`.\n"]
    pub fn set_gtid(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_MysqlSourceConfig_Gtid>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gtid = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gtid = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjects>>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_MysqlSourceConfig {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_MysqlSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_MysqlSourceConfig {}

impl BuildDatastreamStream_SourceConfig_MysqlSourceConfig {
    pub fn build(self) -> DatastreamStream_SourceConfig_MysqlSourceConfig {
        DatastreamStream_SourceConfig_MysqlSourceConfig {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            max_concurrent_cdc_tasks: core::default::Default::default(),
            binary_log_position: core::default::Default::default(),
            exclude_objects: core::default::Default::default(),
            gtid: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_MysqlSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_MysqlSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_MysqlSourceConfigRef {
        DatastreamStream_SourceConfig_MysqlSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_MysqlSourceConfigRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `binary_log_position` after provisioning.\n"]
    pub fn binary_log_position(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_MysqlSourceConfig_BinaryLogPositionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.binary_log_position", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(&self) -> terrars::ListRef<DatastreamStream_SourceConfig_MysqlSourceConfig_ExcludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `gtid` after provisioning.\n"]
    pub fn gtid(&self) -> terrars::ListRef<DatastreamStream_SourceConfig_MysqlSourceConfig_GtidRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gtid", self.base))
    }

    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(&self) -> terrars::ListRef<DatastreamStream_SourceConfig_MysqlSourceConfig_IncludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjects {}

impl DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjects { }

impl terrars::ToListMappable for DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjects {}

impl BuildDatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjects {
        DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjects {}
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjectsRef {
        DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjectsDynamic {
    oracle_schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_schemas: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas>>,
    dynamic: DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjects {
    #[doc = "Set the field `oracle_schemas`.\n"]
    pub fn set_oracle_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjects {}

impl BuildDatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjects {
        DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjects {
            oracle_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjectsRef {
        DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oracle_schemas` after provisioning.\n"]
    pub fn oracle_schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_schemas", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjectsDynamic {
    oracle_schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_schemas: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas>>,
    dynamic: DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjects {
    #[doc = "Set the field `oracle_schemas`.\n"]
    pub fn set_oracle_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemas>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjects {}

impl BuildDatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjects {
        DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjects {
            oracle_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjectsRef {
        DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oracle_schemas` after provisioning.\n"]
    pub fn oracle_schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_OracleExcludedObjects_OracleSchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_schemas", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjects {}

impl DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjects { }

impl terrars::ToListMappable for DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjects {}

impl BuildDatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjects {
        DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjects {}
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjectsRef {
        DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_OracleSourceConfigDynamic {
    drop_large_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjects>>,
    exclude_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjects>>,
    include_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjects>>,
    stream_large_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjects>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_OracleSourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_backfill_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_cdc_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop_large_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_objects: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_objects: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_large_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjects>>,
    dynamic: DatastreamStream_SourceConfig_OracleSourceConfigDynamic,
}

impl DatastreamStream_SourceConfig_OracleSourceConfig {
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
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjects>,
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
                    ::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjects>>,
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
                    ::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjects>>,
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
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjects>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_OracleSourceConfig {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_OracleSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_OracleSourceConfig {}

impl BuildDatastreamStream_SourceConfig_OracleSourceConfig {
    pub fn build(self) -> DatastreamStream_SourceConfig_OracleSourceConfig {
        DatastreamStream_SourceConfig_OracleSourceConfig {
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

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_OracleSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_OracleSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_OracleSourceConfigRef {
        DatastreamStream_SourceConfig_OracleSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_OracleSourceConfigRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_OracleSourceConfig_DropLargeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.drop_large_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_OracleSourceConfig_ExcludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_OracleSourceConfig_IncludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_large_objects` after provisioning.\n"]
    pub fn stream_large_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_OracleSourceConfig_StreamLargeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stream_large_objects", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjectsDynamic {
    postgresql_schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_schemas: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas>>,
    dynamic: DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjects {
    #[doc = "Set the field `postgresql_schemas`.\n"]
    pub fn set_postgresql_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjects {}

impl BuildDatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjects {
        DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjects {
            postgresql_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjectsRef {
        DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `postgresql_schemas` after provisioning.\n"]
    pub fn postgresql_schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_schemas", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjectsDynamic {
    postgresql_schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_schemas: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas>>,
    dynamic: DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjects {
    #[doc = "Set the field `postgresql_schemas`.\n"]
    pub fn set_postgresql_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemas>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjects {}

impl BuildDatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjects {
        DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjects {
            postgresql_schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjectsRef {
        DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `postgresql_schemas` after provisioning.\n"]
    pub fn postgresql_schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStream_BackfillAll_PostgresqlExcludedObjects_PostgresqlSchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_schemas", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_PostgresqlSourceConfigDynamic {
    exclude_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjects>>,
    include_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjects>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_PostgresqlSourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_backfill_tasks: std::option::Option<terrars::PrimField<f64>>,
    publication: terrars::PrimField<std::string::String>,
    replication_slot: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjects>>,
    dynamic: DatastreamStream_SourceConfig_PostgresqlSourceConfigDynamic,
}

impl DatastreamStream_SourceConfig_PostgresqlSourceConfig {
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
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjects>,
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
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjects>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_PostgresqlSourceConfig {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_PostgresqlSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_PostgresqlSourceConfig {
    #[doc =
        "The name of the publication that includes the set of all tables\nthat are defined in the stream's include_objects."]
    pub publication: terrars::PrimField<std::string::String>,
    #[doc = "The name of the logical replication slot that's configured with\nthe pgoutput plugin."]
    pub replication_slot: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_SourceConfig_PostgresqlSourceConfig {
    pub fn build(self) -> DatastreamStream_SourceConfig_PostgresqlSourceConfig {
        DatastreamStream_SourceConfig_PostgresqlSourceConfig {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            publication: self.publication,
            replication_slot: self.replication_slot,
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_PostgresqlSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_PostgresqlSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_PostgresqlSourceConfigRef {
        DatastreamStream_SourceConfig_PostgresqlSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_PostgresqlSourceConfigRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_PostgresqlSourceConfig_ExcludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_PostgresqlSourceConfig_IncludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_objects", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjectsDynamic {
    objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    objects: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects>>,
    dynamic: DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjects {
    #[doc = "Set the field `objects`.\n"]
    pub fn set_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.objects = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjects {}

impl BuildDatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjects {
        DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjects {
            objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjectsRef {
        DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `objects` after provisioning.\n"]
    pub fn objects(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_SalesforceExcludedObjects_ObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjectsDynamic {
    objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    objects: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects>>,
    dynamic: DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjects {
    #[doc = "Set the field `objects`.\n"]
    pub fn set_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_SalesforceExcludedObjects_Objects>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.objects = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.objects = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjects {}

impl BuildDatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjects {
        DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjects {
            objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjectsRef {
        DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `objects` after provisioning.\n"]
    pub fn objects(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_SalesforceExcludedObjects_ObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.objects", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SalesforceSourceConfigDynamic {
    exclude_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjects>>,
    include_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjects>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SalesforceSourceConfig {
    polling_interval: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjects>>,
    dynamic: DatastreamStream_SourceConfig_SalesforceSourceConfigDynamic,
}

impl DatastreamStream_SourceConfig_SalesforceSourceConfig {
    #[doc = "Set the field `exclude_objects`.\n"]
    pub fn set_exclude_objects(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjects>,
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
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjects>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SalesforceSourceConfig {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SalesforceSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SalesforceSourceConfig {
    #[doc =
        "Salesforce objects polling interval. The interval at which new changes will be polled for each object. The duration must be between 5 minutes and 24 hours."]
    pub polling_interval: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_SourceConfig_SalesforceSourceConfig {
    pub fn build(self) -> DatastreamStream_SourceConfig_SalesforceSourceConfig {
        DatastreamStream_SourceConfig_SalesforceSourceConfig {
            polling_interval: self.polling_interval,
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SalesforceSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SalesforceSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SalesforceSourceConfigRef {
        DatastreamStream_SourceConfig_SalesforceSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SalesforceSourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `polling_interval` after provisioning.\nSalesforce objects polling interval. The interval at which new changes will be polled for each object. The duration must be between 5 minutes and 24 hours."]
    pub fn polling_interval(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.polling_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SalesforceSourceConfig_ExcludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SalesforceSourceConfig_IncludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_Columns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_Columns {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_Columns {
    type O =
        terrars
        ::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_Columns {}

impl BuildDatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_Columns {
    pub fn build(self) -> DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_Columns {
        DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_Columns {
            column: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_ColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_ColumnsRef {
        DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_ColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe Spanner data type. Full data types list can be found here:\nhttps://docs.cloud.google.com/spanner/docs/reference/standard-sql/data-types"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_primary_key` after provisioning.\nWhether the column is a primary key."]
    pub fn is_primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_primary_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_TablesDynamic {
    columns: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_Columns>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_Columns>>,
    dynamic: DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_TablesDynamic,
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_Columns,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables {
    type O =
        terrars::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables {
    pub fn build(self) -> DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables {
        DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables {
            table: self.table,
            columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_TablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_TablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_TablesRef {
        DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_TablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_TablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_SchemasDynamic {
    tables: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tables: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables>>,
    dynamic: DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_SchemasDynamic,
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas {
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_Tables,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas {
    #[doc = "Schema name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas {
    pub fn build(self) -> DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas {
        DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas {
            schema: self.schema,
            tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_SchemasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_SchemasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_SchemasRef {
        DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_SchemasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_SchemasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas_TablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjectsDynamic {
    schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schemas: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas>>,
    dynamic: DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_Schemas>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects {}

impl BuildDatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects {
        DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects {
            schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjectsRef {
        DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects_SchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_Columns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    column: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_Columns {
    #[doc = "Set the field `column`.\nColumn name."]
    pub fn set_column(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.column = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_Columns {
    type O =
        terrars
        ::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_Columns {}

impl BuildDatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_Columns {
    pub fn build(self) -> DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_Columns {
        DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_Columns {
            column: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_ColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_ColumnsRef {
        DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_ColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `column` after provisioning.\nColumn name."]
    pub fn column(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.column", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nThe Spanner data type. Full data types list can be found here:\nhttps://docs.cloud.google.com/spanner/docs/reference/standard-sql/data-types"]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_primary_key` after provisioning.\nWhether or not the column is a primary key."]
    pub fn is_primary_key(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_primary_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ordinal_position` after provisioning.\nThe ordinal position of the column in the table."]
    pub fn ordinal_position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ordinal_position", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_TablesDynamic {
    columns: std
    ::option
    ::Option<
        terrars::DynamicBlock<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_Columns>,
    >,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables {
    table: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_Columns>>,
    dynamic: DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_TablesDynamic,
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_Columns,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables {
    type O =
        terrars::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables {
    #[doc = "Table name."]
    pub table: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables {
    pub fn build(self) -> DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables {
        DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables {
            table: self.table,
            columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_TablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_TablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_TablesRef {
        DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_TablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_TablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\nTable name."]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_SchemasDynamic {
    tables: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas {
    schema: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tables: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables>>,
    dynamic: DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_SchemasDynamic,
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas {
    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_Tables,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas {
    #[doc = "Schema name."]
    pub schema: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas {
    pub fn build(self) -> DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas {
        DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas {
            schema: self.schema,
            tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_SchemasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_SchemasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_SchemasRef {
        DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_SchemasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_SchemasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\nSchema name."]
    pub fn schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema", self.base))
    }

    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas_TablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjectsDynamic {
    schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schemas: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas>>,
    dynamic: DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_Schemas>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects {}

impl BuildDatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects {
        DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects {
            schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjectsRef {
        DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects_SchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SpannerSourceConfigDynamic {
    exclude_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects>>,
    include_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backfill_data_boost_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    change_stream_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fgac_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_backfill_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_cdc_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spanner_rpc_priority: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_objects: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_objects: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects>>,
    dynamic: DatastreamStream_SourceConfig_SpannerSourceConfigDynamic,
}

impl DatastreamStream_SourceConfig_SpannerSourceConfig {
    #[doc = "Set the field `backfill_data_boost_enabled`.\nWhether to use DataBoost for backfill queries."]
    pub fn set_backfill_data_boost_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.backfill_data_boost_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `change_stream_name`.\nThe Spanner change stream name to use."]
    pub fn set_change_stream_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.change_stream_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fgac_role`.\nThe FGAC role to use for Spanner queries."]
    pub fn set_fgac_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fgac_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrent_backfill_tasks`.\nMax concurrent backfill tasks."]
    pub fn set_max_concurrent_backfill_tasks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_backfill_tasks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrent_cdc_tasks`.\nMax concurrent CDC tasks."]
    pub fn set_max_concurrent_cdc_tasks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_cdc_tasks = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `spanner_rpc_priority`.\nThe RPC priority to use for Spanner queries. Possible values: [\"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn set_spanner_rpc_priority(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.spanner_rpc_priority = std::option::Option::Some(v.into());
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
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjects>,
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
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjects>,
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

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SpannerSourceConfig {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SpannerSourceConfig {}

impl BuildDatastreamStream_SourceConfig_SpannerSourceConfig {
    pub fn build(self) -> DatastreamStream_SourceConfig_SpannerSourceConfig {
        DatastreamStream_SourceConfig_SpannerSourceConfig {
            backfill_data_boost_enabled: core::default::Default::default(),
            change_stream_name: core::default::Default::default(),
            fgac_role: core::default::Default::default(),
            max_concurrent_backfill_tasks: core::default::Default::default(),
            max_concurrent_cdc_tasks: core::default::Default::default(),
            spanner_rpc_priority: core::default::Default::default(),
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SpannerSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SpannerSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SpannerSourceConfigRef {
        DatastreamStream_SourceConfig_SpannerSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SpannerSourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `backfill_data_boost_enabled` after provisioning.\nWhether to use DataBoost for backfill queries."]
    pub fn backfill_data_boost_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.backfill_data_boost_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `change_stream_name` after provisioning.\nThe Spanner change stream name to use."]
    pub fn change_stream_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.change_stream_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fgac_role` after provisioning.\nThe FGAC role to use for Spanner queries."]
    pub fn fgac_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fgac_role", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nMax concurrent backfill tasks."]
    pub fn max_concurrent_backfill_tasks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_backfill_tasks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_cdc_tasks` after provisioning.\nMax concurrent CDC tasks."]
    pub fn max_concurrent_cdc_tasks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_cdc_tasks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `spanner_rpc_priority` after provisioning.\nThe RPC priority to use for Spanner queries. Possible values: [\"LOW\", \"MEDIUM\", \"HIGH\"]"]
    pub fn spanner_rpc_priority(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spanner_rpc_priority", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SpannerSourceConfig_ExcludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SpannerSourceConfig_IncludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_objects", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTables {}

impl DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTables { }

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTables {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTables {}

impl BuildDatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTables {
    pub fn build(self) -> DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTables {
        DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTables {}
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTablesRef {
        DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjectsDynamic {
    schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schemas: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas>>,
    dynamic: DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjects {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjects {}

impl BuildDatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjects {
        DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjects {
            schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjectsRef {
        DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_SqlServerExcludedObjects_SchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjectsDynamic {
    schemas: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjects {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schemas: std::option::Option<std::vec::Vec<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas>>,
    dynamic: DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjectsDynamic,
}

impl DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjects {
    #[doc = "Set the field `schemas`.\n"]
    pub fn set_schemas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatastreamStream_BackfillAll_SqlServerExcludedObjects_Schemas>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schemas = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schemas = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjects {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjects>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjects {}

impl BuildDatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjects {
    pub fn build(self) -> DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjects {
        DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjects {
            schemas: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjectsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjectsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjectsRef {
        DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjectsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjectsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schemas` after provisioning.\n"]
    pub fn schemas(&self) -> terrars::ListRef<DatastreamStream_BackfillAll_SqlServerExcludedObjects_SchemasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schemas", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogs {}

impl DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogs { }

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogs {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogs {}

impl BuildDatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogs {
    pub fn build(self) -> DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogs {
        DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogs {}
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogsRef {
        DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfig_SqlServerSourceConfigDynamic {
    change_tables: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTables>>,
    exclude_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjects>>,
    include_objects: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjects>>,
    transaction_logs: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogs>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig_SqlServerSourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_backfill_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_cdc_tasks: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    change_tables: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTables>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_objects: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjects>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transaction_logs: std
    ::option
    ::Option<std::vec::Vec<DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogs>>,
    dynamic: DatastreamStream_SourceConfig_SqlServerSourceConfigDynamic,
}

impl DatastreamStream_SourceConfig_SqlServerSourceConfig {
    #[doc = "Set the field `max_concurrent_backfill_tasks`.\nMax concurrent backfill tasks."]
    pub fn set_max_concurrent_backfill_tasks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_backfill_tasks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrent_cdc_tasks`.\nMax concurrent CDC tasks."]
    pub fn set_max_concurrent_cdc_tasks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_concurrent_cdc_tasks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `change_tables`.\n"]
    pub fn set_change_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTables>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.change_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.change_tables = std::option::Option::Some(d);
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
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjects>,
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
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjects>,
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

    #[doc = "Set the field `transaction_logs`.\n"]
    pub fn set_transaction_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transaction_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transaction_logs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig_SqlServerSourceConfig {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig_SqlServerSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig_SqlServerSourceConfig {}

impl BuildDatastreamStream_SourceConfig_SqlServerSourceConfig {
    pub fn build(self) -> DatastreamStream_SourceConfig_SqlServerSourceConfig {
        DatastreamStream_SourceConfig_SqlServerSourceConfig {
            max_concurrent_backfill_tasks: core::default::Default::default(),
            max_concurrent_cdc_tasks: core::default::Default::default(),
            change_tables: core::default::Default::default(),
            exclude_objects: core::default::Default::default(),
            include_objects: core::default::Default::default(),
            transaction_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfig_SqlServerSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfig_SqlServerSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatastreamStream_SourceConfig_SqlServerSourceConfigRef {
        DatastreamStream_SourceConfig_SqlServerSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfig_SqlServerSourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_backfill_tasks` after provisioning.\nMax concurrent backfill tasks."]
    pub fn max_concurrent_backfill_tasks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_backfill_tasks", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_concurrent_cdc_tasks` after provisioning.\nMax concurrent CDC tasks."]
    pub fn max_concurrent_cdc_tasks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_concurrent_cdc_tasks", self.base))
    }

    #[doc = "Get a reference to the value of field `change_tables` after provisioning.\n"]
    pub fn change_tables(&self) -> terrars::ListRef<DatastreamStream_SourceConfig_SqlServerSourceConfig_ChangeTablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.change_tables", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_objects` after provisioning.\n"]
    pub fn exclude_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SqlServerSourceConfig_ExcludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `include_objects` after provisioning.\n"]
    pub fn include_objects(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SqlServerSourceConfig_IncludeObjectsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_objects", self.base))
    }

    #[doc = "Get a reference to the value of field `transaction_logs` after provisioning.\n"]
    pub fn transaction_logs(
        &self,
    ) -> terrars::ListRef<DatastreamStream_SourceConfig_SqlServerSourceConfig_TransactionLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transaction_logs", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatastreamStream_SourceConfigDynamic {
    mongodb_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_MongodbSourceConfig>>,
    mysql_source_config: std::option::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_MysqlSourceConfig>>,
    oracle_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_OracleSourceConfig>>,
    postgresql_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_PostgresqlSourceConfig>>,
    salesforce_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SalesforceSourceConfig>>,
    spanner_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SpannerSourceConfig>>,
    sql_server_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<DatastreamStream_SourceConfig_SqlServerSourceConfig>>,
}

#[derive(Serialize)]
pub struct DatastreamStream_SourceConfig {
    source_connection_profile: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mongodb_source_config: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_MongodbSourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql_source_config: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_MysqlSourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle_source_config: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_OracleSourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql_source_config: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_PostgresqlSourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce_source_config: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_SalesforceSourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spanner_source_config: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_SpannerSourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_server_source_config: std::option::Option<std::vec::Vec<DatastreamStream_SourceConfig_SqlServerSourceConfig>>,
    dynamic: DatastreamStream_SourceConfigDynamic,
}

impl DatastreamStream_SourceConfig {
    #[doc = "Set the field `mongodb_source_config`.\n"]
    pub fn set_mongodb_source_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_MongodbSourceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mongodb_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mongodb_source_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mysql_source_config`.\n"]
    pub fn set_mysql_source_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_MysqlSourceConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_OracleSourceConfig>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_PostgresqlSourceConfig>>,
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

    #[doc = "Set the field `salesforce_source_config`.\n"]
    pub fn set_salesforce_source_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_SalesforceSourceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce_source_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spanner_source_config`.\n"]
    pub fn set_spanner_source_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_SpannerSourceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spanner_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spanner_source_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sql_server_source_config`.\n"]
    pub fn set_sql_server_source_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatastreamStream_SourceConfig_SqlServerSourceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sql_server_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_server_source_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatastreamStream_SourceConfig {
    type O = terrars::BlockAssignable<DatastreamStream_SourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_SourceConfig {
    #[doc =
        "Source connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}"]
    pub source_connection_profile: terrars::PrimField<std::string::String>,
}

impl BuildDatastreamStream_SourceConfig {
    pub fn build(self) -> DatastreamStream_SourceConfig {
        DatastreamStream_SourceConfig {
            source_connection_profile: self.source_connection_profile,
            mongodb_source_config: core::default::Default::default(),
            mysql_source_config: core::default::Default::default(),
            oracle_source_config: core::default::Default::default(),
            postgresql_source_config: core::default::Default::default(),
            salesforce_source_config: core::default::Default::default(),
            spanner_source_config: core::default::Default::default(),
            sql_server_source_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_SourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_SourceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStream_SourceConfigRef {
        DatastreamStream_SourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_SourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `source_connection_profile` after provisioning.\nSource connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}"]
    pub fn source_connection_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_connection_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `mongodb_source_config` after provisioning.\n"]
    pub fn mongodb_source_config(&self) -> terrars::ListRef<DatastreamStream_SourceConfig_MongodbSourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mongodb_source_config", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql_source_config` after provisioning.\n"]
    pub fn mysql_source_config(&self) -> terrars::ListRef<DatastreamStream_SourceConfig_MysqlSourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql_source_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle_source_config` after provisioning.\n"]
    pub fn oracle_source_config(&self) -> terrars::ListRef<DatastreamStream_SourceConfig_OracleSourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle_source_config", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql_source_config` after provisioning.\n"]
    pub fn postgresql_source_config(&self) -> terrars::ListRef<DatastreamStream_SourceConfig_PostgresqlSourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql_source_config", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce_source_config` after provisioning.\n"]
    pub fn salesforce_source_config(&self) -> terrars::ListRef<DatastreamStream_SourceConfig_SalesforceSourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce_source_config", self.base))
    }

    #[doc = "Get a reference to the value of field `spanner_source_config` after provisioning.\n"]
    pub fn spanner_source_config(&self) -> terrars::ListRef<DatastreamStream_SourceConfig_SpannerSourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spanner_source_config", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_server_source_config` after provisioning.\n"]
    pub fn sql_server_source_config(&self) -> terrars::ListRef<DatastreamStream_SourceConfig_SqlServerSourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server_source_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DatastreamStream_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatastreamStream_Timeouts {
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

impl terrars::ToListMappable for DatastreamStream_Timeouts {
    type O = terrars::BlockAssignable<DatastreamStream_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatastreamStream_Timeouts {}

impl BuildDatastreamStream_Timeouts {
    pub fn build(self) -> DatastreamStream_Timeouts {
        DatastreamStream_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatastreamStream_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatastreamStream_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatastreamStream_TimeoutsRef {
        DatastreamStream_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatastreamStream_TimeoutsRef {
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
