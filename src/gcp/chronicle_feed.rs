#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleFeedData_ {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feed: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<std::vec::Vec<ChronicleFeed_Details>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_details: std::option::Option<std::vec::Vec<ChronicleFeed_FailureDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleFeed_Timeouts>,
    dynamic: ChronicleFeedDynamic,
}

struct ChronicleFeed_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleFeedData_>,
}

#[derive(Clone)]
pub struct ChronicleFeed(std::rc::Rc<ChronicleFeed_>);

impl ChronicleFeed {
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

    #[doc = "Set the field `display_name`.\nCustomer-provided feed name."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nWhether the feed is enabled."]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `feed`.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn set_feed(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().feed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(self, v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `failure_details`.\n"]
    pub fn set_failure_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_FailureDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().failure_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.failure_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleFeed_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nCustomer-provided feed name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the feed is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_msg` after provisioning.\nDetails about the most recent failure when feed state is FAILED."]
    pub fn failure_msg(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_msg", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `feed` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn feed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `feed_service_account` after provisioning.\nOutput only. The service account used by Chronicle to ingest data from Cloud Storage. This is only available when the feed source type is GOOGLE_CLOUD_STORAGE_EVENT_DRIVEN or GOOGLE_CLOUD_STORAGE."]
    pub fn feed_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feed_service_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_feed_initiation_time` after provisioning.\nLatest timestamp when the transfer was successful for the feed."]
    pub fn last_feed_initiation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_feed_initiation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the feed.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/feeds/{feed}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nWhether this feed can be updated or deleted."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reference_id` after provisioning.\nReference ID, this field will contain the legacy id of the feed."]
    pub fn reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nOutput only. The secret generated for the feed. This is only available when the feed source type is HTTPS_PUSH_AMAZON_KINESIS_FIREHOSE."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the feed (e.g., ACTIVE, INACTIVE)."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier for the feed."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<ChronicleFeed_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_details` after provisioning.\n"]
    pub fn failure_details(&self) -> terrars::ListRef<ChronicleFeed_FailureDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleFeed_TimeoutsRef {
        ChronicleFeed_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleFeed {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleFeed { }

impl terrars::ToListMappable for ChronicleFeed {
    type O = terrars::ListRef<ChronicleFeedRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleFeed_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_feed".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleFeed {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleFeed {
        let out = ChronicleFeed(std::rc::Rc::new(ChronicleFeed_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleFeedData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                enabled: core::default::Default::default(),
                feed: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                details: core::default::Default::default(),
                failure_details: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleFeedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeedRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleFeedRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nCustomer-provided feed name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nWhether the feed is enabled."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_msg` after provisioning.\nDetails about the most recent failure when feed state is FAILED."]
    pub fn failure_msg(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_msg", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `feed` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn feed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `feed_service_account` after provisioning.\nOutput only. The service account used by Chronicle to ingest data from Cloud Storage. This is only available when the feed source type is GOOGLE_CLOUD_STORAGE_EVENT_DRIVEN or GOOGLE_CLOUD_STORAGE."]
    pub fn feed_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feed_service_account", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_feed_initiation_time` after provisioning.\nLatest timestamp when the transfer was successful for the feed."]
    pub fn last_feed_initiation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_feed_initiation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the feed.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/feeds/{feed}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nWhether this feed can be updated or deleted."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `reference_id` after provisioning.\nReference ID, this field will contain the legacy id of the feed."]
    pub fn reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nOutput only. The secret generated for the feed. This is only available when the feed source type is HTTPS_PUSH_AMAZON_KINESIS_FIREHOSE."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the feed (e.g., ACTIVE, INACTIVE)."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nUnique identifier for the feed."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<ChronicleFeed_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_details` after provisioning.\n"]
    pub fn failure_details(&self) -> terrars::ListRef<ChronicleFeed_FailureDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.failure_details", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleFeed_TimeoutsRef {
        ChronicleFeed_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeedDynamic {
    details: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details>>,
    failure_details: std::option::Option<terrars::DynamicBlock<ChronicleFeed_FailureDetails>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonKinesisFirehoseSettings {}

impl ChronicleFeed_Details_AmazonKinesisFirehoseSettings { }

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonKinesisFirehoseSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonKinesisFirehoseSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonKinesisFirehoseSettings {}

impl BuildChronicleFeed_Details_AmazonKinesisFirehoseSettings {
    pub fn build(self) -> ChronicleFeed_Details_AmazonKinesisFirehoseSettings {
        ChronicleFeed_Details_AmazonKinesisFirehoseSettings {}
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonKinesisFirehoseSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonKinesisFirehoseSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AmazonKinesisFirehoseSettingsRef {
        ChronicleFeed_Details_AmazonKinesisFirehoseSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonKinesisFirehoseSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonS3Settings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_AmazonS3Settings_Authentication {
    #[doc = "Set the field `access_key_id`.\nAccess key ID. Used when using access key auth."]
    pub fn set_access_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_id`.\nClient ID. Used when using OAuth auth."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nClient secret. Used when using OAuth auth."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_uri`.\nRefresh URI. Used when using OAuth auth."]
    pub fn set_refresh_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_access_key`.\nSecret access key. Used when using access key auth."]
    pub fn set_secret_access_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_access_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonS3Settings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonS3Settings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonS3Settings_Authentication {
    #[doc =
        "Possible values:\nUS_EAST_1\nUS_EAST_2\nUS_WEST_1\nUS_WEST_2\nUS_GOV_CLOUD\nUS_GOV_EAST_1\nEU_WEST_1\nEU_WEST_2\nEU_WEST_3\nEU_CENTRAL_1\nEU_NORTH_1\nEU_SOUTH_1\nAP_SOUTH_1\nAP_SOUTHEAST_1\nAP_SOUTHEAST_2\nAP_SOUTHEAST_3\nAP_NORTHEAST_1\nAP_NORTHEAST_2\nAP_NORTHEAST_3\nAP_EAST_1\nSA_EAST_1\nCN_NORTH_1\nCN_NORTHWEST_1\nCA_CENTRAL_1\nAF_SOUTH_1\nME_SOUTH_1\nAP_SOUTH_2\nAP_SOUTHEAST_4\nCA_WEST_1\nEU_SOUTH_2\nEU_CENTRAL_2\nIL_CENTRAL_1\nME_CENTRAL_1"]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_AmazonS3Settings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_AmazonS3Settings_Authentication {
        ChronicleFeed_Details_AmazonS3Settings_Authentication {
            access_key_id: core::default::Default::default(),
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            refresh_uri: core::default::Default::default(),
            region: self.region,
            secret_access_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonS3Settings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonS3Settings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AmazonS3Settings_AuthenticationRef {
        ChronicleFeed_Details_AmazonS3Settings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonS3Settings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nAccess key ID. Used when using access key auth."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nClient ID. Used when using OAuth auth."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nClient secret. Used when using OAuth auth."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `refresh_uri` after provisioning.\nRefresh URI. Used when using OAuth auth."]
    pub fn refresh_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nPossible values:\nUS_EAST_1\nUS_EAST_2\nUS_WEST_1\nUS_WEST_2\nUS_GOV_CLOUD\nUS_GOV_EAST_1\nEU_WEST_1\nEU_WEST_2\nEU_WEST_3\nEU_CENTRAL_1\nEU_NORTH_1\nEU_SOUTH_1\nAP_SOUTH_1\nAP_SOUTHEAST_1\nAP_SOUTHEAST_2\nAP_SOUTHEAST_3\nAP_NORTHEAST_1\nAP_NORTHEAST_2\nAP_NORTHEAST_3\nAP_EAST_1\nSA_EAST_1\nCN_NORTH_1\nCN_NORTHWEST_1\nCA_CENTRAL_1\nAF_SOUTH_1\nME_SOUTH_1\nAP_SOUTH_2\nAP_SOUTHEAST_4\nCA_WEST_1\nEU_SOUTH_2\nEU_CENTRAL_2\nIL_CENTRAL_1\nME_CENTRAL_1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key` after provisioning.\nSecret access key. Used when using access key auth."]
    pub fn secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AmazonS3SettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonS3Settings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonS3Settings {
    s3_uri: terrars::PrimField<std::string::String>,
    source_deletion_option: terrars::PrimField<std::string::String>,
    source_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AmazonS3Settings_Authentication>>,
    dynamic: ChronicleFeed_Details_AmazonS3SettingsDynamic,
}

impl ChronicleFeed_Details_AmazonS3Settings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AmazonS3Settings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonS3Settings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonS3Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonS3Settings {
    #[doc = "S3 URI."]
    pub s3_uri: terrars::PrimField<std::string::String>,
    #[doc =
        "Possible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub source_deletion_option: terrars::PrimField<std::string::String>,
    #[doc = "Possible values:\nFILES\nFOLDERS\nFOLDERS_RECURSIVE"]
    pub source_type: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_AmazonS3Settings {
    pub fn build(self) -> ChronicleFeed_Details_AmazonS3Settings {
        ChronicleFeed_Details_AmazonS3Settings {
            s3_uri: self.s3_uri,
            source_deletion_option: self.source_deletion_option,
            source_type: self.source_type,
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonS3SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonS3SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_AmazonS3SettingsRef {
        ChronicleFeed_Details_AmazonS3SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonS3SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\nS3 URI."]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_deletion_option` after provisioning.\nPossible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub fn source_deletion_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_deletion_option", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_type` after provisioning.\nPossible values:\nFILES\nFOLDERS\nFOLDERS_RECURSIVE"]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AmazonS3Settings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuth {
    access_key_id: terrars::PrimField<std::string::String>,
    secret_access_key: terrars::PrimField<std::string::String>,
}

impl ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuth { }

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuth {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuth {
    #[doc = "Access Key ID for an AWS account (a 20-character, alphanumeric string)."]
    pub access_key_id: terrars::PrimField<std::string::String>,
    #[doc = "Secret Access Key for an AWS account (a 40-character string)."]
    pub secret_access_key: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuth {
    pub fn build(self) -> ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuth {
        ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuth {
            access_key_id: self.access_key_id,
            secret_access_key: self.secret_access_key,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuthRef {
        ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nAccess Key ID for an AWS account (a 20-character, alphanumeric string)."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key` after provisioning.\nSecret Access Key for an AWS account (a 40-character string)."]
    pub fn secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_iam_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuth {
    #[doc = "Set the field `aws_iam_role_arn`.\nAWS IAM Role for Identity Federation."]
    pub fn set_aws_iam_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aws_iam_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_id`.\nSubject ID to use for S3."]
    pub fn set_subject_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuth {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuth {}

impl BuildChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuth {
    pub fn build(self) -> ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuth {
        ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuth {
            aws_iam_role_arn: core::default::Default::default(),
            subject_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuthRef {
        ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aws_iam_role_arn` after provisioning.\nAWS IAM Role for Identity Federation."]
    pub fn aws_iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_iam_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_id` after provisioning.\nSubject ID to use for S3."]
    pub fn subject_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AmazonS3V2Settings_AuthenticationDynamic {
    access_key_secret_auth: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuth>>,
    aws_iam_role_auth: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuth>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonS3V2Settings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_secret_auth: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuth>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_iam_role_auth: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuth>>,
    dynamic: ChronicleFeed_Details_AmazonS3V2Settings_AuthenticationDynamic,
}

impl ChronicleFeed_Details_AmazonS3V2Settings_Authentication {
    #[doc = "Set the field `access_key_secret_auth`.\n"]
    pub fn set_access_key_secret_auth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuth>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_key_secret_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_key_secret_auth = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_iam_role_auth`.\n"]
    pub fn set_aws_iam_role_auth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuth>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_iam_role_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_iam_role_auth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonS3V2Settings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonS3V2Settings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonS3V2Settings_Authentication {}

impl BuildChronicleFeed_Details_AmazonS3V2Settings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_AmazonS3V2Settings_Authentication {
        ChronicleFeed_Details_AmazonS3V2Settings_Authentication {
            access_key_secret_auth: core::default::Default::default(),
            aws_iam_role_auth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonS3V2Settings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonS3V2Settings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AmazonS3V2Settings_AuthenticationRef {
        ChronicleFeed_Details_AmazonS3V2Settings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonS3V2Settings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_key_secret_auth` after provisioning.\n"]
    pub fn access_key_secret_auth(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AccessKeySecretAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_key_secret_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_iam_role_auth` after provisioning.\n"]
    pub fn aws_iam_role_auth(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_AmazonS3V2Settings_Authentication_AwsIamRoleAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_iam_role_auth", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AmazonS3V2SettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonS3V2Settings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonS3V2Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_lookback_days: std::option::Option<terrars::PrimField<f64>>,
    s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_deletion_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AmazonS3V2Settings_Authentication>>,
    dynamic: ChronicleFeed_Details_AmazonS3V2SettingsDynamic,
}

impl ChronicleFeed_Details_AmazonS3V2Settings {
    #[doc = "Set the field `max_lookback_days`.\nMaximum File Age to ingest in days."]
    pub fn set_max_lookback_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_lookback_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_deletion_option`.\nPossible values:\nNEVER\nON_SUCCESS"]
    pub fn set_source_deletion_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_deletion_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_AmazonS3V2Settings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonS3V2Settings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonS3V2Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonS3V2Settings {
    #[doc = "S3 URI."]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_AmazonS3V2Settings {
    pub fn build(self) -> ChronicleFeed_Details_AmazonS3V2Settings {
        ChronicleFeed_Details_AmazonS3V2Settings {
            max_lookback_days: core::default::Default::default(),
            s3_uri: self.s3_uri,
            source_deletion_option: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonS3V2SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonS3V2SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_AmazonS3V2SettingsRef {
        ChronicleFeed_Details_AmazonS3V2SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonS3V2SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `chronicle_service_account` after provisioning.\nSA that will read data, this is Storage Transfer Service SA of Customer's\nTenancy Project."]
    pub fn chronicle_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chronicle_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_lookback_days` after provisioning.\nMaximum File Age to ingest in days."]
    pub fn max_lookback_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_lookback_days", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\nS3 URI."]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_deletion_option` after provisioning.\nPossible values:\nNEVER\nON_SUCCESS"]
    pub fn source_deletion_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_deletion_option", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AmazonS3V2Settings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuth {
    #[doc = "Set the field `access_key_id`.\nAccess key ID."]
    pub fn set_access_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_access_key`.\nSecret access key."]
    pub fn set_secret_access_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_access_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuth {
    type O =
        terrars
        ::BlockAssignable<ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuth {}

impl BuildChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuth {
    pub fn build(self) -> ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuth {
        ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuth {
            access_key_id: core::default::Default::default(),
            secret_access_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuthRef {
        ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\nAccess key ID."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_access_key` after provisioning.\nSecret access key."]
    pub fn secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuth {
    #[doc = "Set the field `access_key_id`.\nAccess key ID."]
    pub fn set_access_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_access_key`.\nSecret access key."]
    pub fn set_secret_access_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_access_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuth {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuth {}

impl BuildChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuth {
    pub fn build(self) -> ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuth {
        ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuth {
            access_key_id: core::default::Default::default(),
            secret_access_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuthRef {
        ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\nAccess key ID."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_access_key` after provisioning.\nSecret access key."]
    pub fn secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AmazonSqsSettings_AuthenticationDynamic {
    additional_s3_access_key_secret_auth: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuth>,
    >,
    sqs_access_key_secret_auth: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuth>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonSqsSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_s3_access_key_secret_auth: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuth>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs_access_key_secret_auth: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuth>>,
    dynamic: ChronicleFeed_Details_AmazonSqsSettings_AuthenticationDynamic,
}

impl ChronicleFeed_Details_AmazonSqsSettings_Authentication {
    #[doc = "Set the field `additional_s3_access_key_secret_auth`.\n"]
    pub fn set_additional_s3_access_key_secret_auth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuth,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_s3_access_key_secret_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_s3_access_key_secret_auth = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqs_access_key_secret_auth`.\n"]
    pub fn set_sqs_access_key_secret_auth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuth,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqs_access_key_secret_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_access_key_secret_auth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonSqsSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonSqsSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonSqsSettings_Authentication {}

impl BuildChronicleFeed_Details_AmazonSqsSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_AmazonSqsSettings_Authentication {
        ChronicleFeed_Details_AmazonSqsSettings_Authentication {
            additional_s3_access_key_secret_auth: core::default::Default::default(),
            sqs_access_key_secret_auth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonSqsSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonSqsSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AmazonSqsSettings_AuthenticationRef {
        ChronicleFeed_Details_AmazonSqsSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonSqsSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_s3_access_key_secret_auth` after provisioning.\n"]
    pub fn additional_s3_access_key_secret_auth(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_AmazonSqsSettings_Authentication_AdditionalS3AccessKeySecretAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_s3_access_key_secret_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `sqs_access_key_secret_auth` after provisioning.\n"]
    pub fn sqs_access_key_secret_auth(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_AmazonSqsSettings_Authentication_SqsAccessKeySecretAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs_access_key_secret_auth", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AmazonSqsSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonSqsSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonSqsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    queue: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_deletion_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AmazonSqsSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_AmazonSqsSettingsDynamic,
}

impl ChronicleFeed_Details_AmazonSqsSettings {
    #[doc = "Set the field `account_number`.\nAccount number of the owner of the queue."]
    pub fn set_account_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `queue`.\nName of the queue."]
    pub fn set_queue(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.queue = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nPossible values:\nUS_EAST_1\nUS_EAST_2\nUS_WEST_1\nUS_WEST_2\nUS_GOV_CLOUD\nUS_GOV_EAST_1\nEU_WEST_1\nEU_WEST_2\nEU_WEST_3\nEU_CENTRAL_1\nEU_NORTH_1\nEU_SOUTH_1\nAP_SOUTH_1\nAP_SOUTHEAST_1\nAP_SOUTHEAST_2\nAP_SOUTHEAST_3\nAP_NORTHEAST_1\nAP_NORTHEAST_2\nAP_NORTHEAST_3\nAP_EAST_1\nSA_EAST_1\nCN_NORTH_1\nCN_NORTHWEST_1\nCA_CENTRAL_1\nAF_SOUTH_1\nME_SOUTH_1\nAP_SOUTH_2\nAP_SOUTHEAST_4\nCA_WEST_1\nEU_SOUTH_2\nEU_CENTRAL_2\nIL_CENTRAL_1\nME_CENTRAL_1"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_deletion_option`.\nPossible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub fn set_source_deletion_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_deletion_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AmazonSqsSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonSqsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonSqsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonSqsSettings {}

impl BuildChronicleFeed_Details_AmazonSqsSettings {
    pub fn build(self) -> ChronicleFeed_Details_AmazonSqsSettings {
        ChronicleFeed_Details_AmazonSqsSettings {
            account_number: core::default::Default::default(),
            queue: core::default::Default::default(),
            region: core::default::Default::default(),
            source_deletion_option: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonSqsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonSqsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_AmazonSqsSettingsRef {
        ChronicleFeed_Details_AmazonSqsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonSqsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_number` after provisioning.\nAccount number of the owner of the queue."]
    pub fn account_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_number", self.base))
    }

    #[doc = "Get a reference to the value of field `queue` after provisioning.\nName of the queue."]
    pub fn queue(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue", self.base))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nPossible values:\nUS_EAST_1\nUS_EAST_2\nUS_WEST_1\nUS_WEST_2\nUS_GOV_CLOUD\nUS_GOV_EAST_1\nEU_WEST_1\nEU_WEST_2\nEU_WEST_3\nEU_CENTRAL_1\nEU_NORTH_1\nEU_SOUTH_1\nAP_SOUTH_1\nAP_SOUTHEAST_1\nAP_SOUTHEAST_2\nAP_SOUTHEAST_3\nAP_NORTHEAST_1\nAP_NORTHEAST_2\nAP_NORTHEAST_3\nAP_EAST_1\nSA_EAST_1\nCN_NORTH_1\nCN_NORTHWEST_1\nCA_CENTRAL_1\nAF_SOUTH_1\nME_SOUTH_1\nAP_SOUTH_2\nAP_SOUTHEAST_4\nCA_WEST_1\nEU_SOUTH_2\nEU_CENTRAL_2\nIL_CENTRAL_1\nME_CENTRAL_1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_deletion_option` after provisioning.\nPossible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub fn source_deletion_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_deletion_option", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AmazonSqsSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_iam_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuth {
    #[doc = "Set the field `aws_iam_role_arn`.\nAWS IAM Role for Identity Federation."]
    pub fn set_aws_iam_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.aws_iam_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_id`.\nSubject ID to use for SQS."]
    pub fn set_subject_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuth {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuth {}

impl BuildChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuth {
    pub fn build(self) -> ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuth {
        ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuth {
            aws_iam_role_arn: core::default::Default::default(),
            subject_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuthRef {
        ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `aws_iam_role_arn` after provisioning.\nAWS IAM Role for Identity Federation."]
    pub fn aws_iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_iam_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `subject_id` after provisioning.\nSubject ID to use for SQS."]
    pub fn subject_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject_id", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuth {
    #[doc = "Set the field `access_key_id`.\nAccess key ID of the S3 bucket.  Ex: AKIABCDEFGHIJKL."]
    pub fn set_access_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_access_key`.\nSecret access key to access the S3 bucket."]
    pub fn set_secret_access_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_access_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuth {
    type O =
        terrars::BlockAssignable<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuth {}

impl BuildChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuth {
    pub fn build(self) -> ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuth {
        ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuth {
            access_key_id: core::default::Default::default(),
            secret_access_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuthRef {
        ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key_id` after provisioning.\nAccess key ID of the S3 bucket.  Ex: AKIABCDEFGHIJKL."]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key` after provisioning.\nSecret access key to access the S3 bucket."]
    pub fn secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AmazonSqsV2Settings_AuthenticationDynamic {
    aws_iam_role_auth: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuth>>,
    sqs_v2_access_key_secret_auth: std
    ::option
    ::Option<
        terrars::DynamicBlock<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuth>,
    >,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonSqsV2Settings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_iam_role_auth: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuth>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs_v2_access_key_secret_auth: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuth>>,
    dynamic: ChronicleFeed_Details_AmazonSqsV2Settings_AuthenticationDynamic,
}

impl ChronicleFeed_Details_AmazonSqsV2Settings_Authentication {
    #[doc = "Set the field `aws_iam_role_auth`.\n"]
    pub fn set_aws_iam_role_auth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuth>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_iam_role_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_iam_role_auth = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqs_v2_access_key_secret_auth`.\n"]
    pub fn set_sqs_v2_access_key_secret_auth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuth,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqs_v2_access_key_secret_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_v2_access_key_secret_auth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonSqsV2Settings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonSqsV2Settings_Authentication {}

impl BuildChronicleFeed_Details_AmazonSqsV2Settings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_AmazonSqsV2Settings_Authentication {
        ChronicleFeed_Details_AmazonSqsV2Settings_Authentication {
            aws_iam_role_auth: core::default::Default::default(),
            sqs_v2_access_key_secret_auth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonSqsV2Settings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonSqsV2Settings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AmazonSqsV2Settings_AuthenticationRef {
        ChronicleFeed_Details_AmazonSqsV2Settings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonSqsV2Settings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_iam_role_auth` after provisioning.\n"]
    pub fn aws_iam_role_auth(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_AwsIamRoleAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_iam_role_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `sqs_v2_access_key_secret_auth` after provisioning.\n"]
    pub fn sqs_v2_access_key_secret_auth(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication_SqsV2AccessKeySecretAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs_v2_access_key_secret_auth", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AmazonSqsV2SettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AmazonSqsV2Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_lookback_days: std::option::Option<terrars::PrimField<f64>>,
    queue: terrars::PrimField<std::string::String>,
    s3_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_deletion_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication>>,
    dynamic: ChronicleFeed_Details_AmazonSqsV2SettingsDynamic,
}

impl ChronicleFeed_Details_AmazonSqsV2Settings {
    #[doc = "Set the field `max_lookback_days`.\nMaximum File Age to ingest in days."]
    pub fn set_max_lookback_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_lookback_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_deletion_option`.\nPossible values:\nNEVER\nON_SUCCESS"]
    pub fn set_source_deletion_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_deletion_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_AmazonSqsV2Settings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AmazonSqsV2Settings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AmazonSqsV2Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AmazonSqsV2Settings {
    #[doc = "Amazon Resource Name(ARN) of the queue."]
    pub queue: terrars::PrimField<std::string::String>,
    #[doc = "S3 URI."]
    pub s3_uri: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_AmazonSqsV2Settings {
    pub fn build(self) -> ChronicleFeed_Details_AmazonSqsV2Settings {
        ChronicleFeed_Details_AmazonSqsV2Settings {
            max_lookback_days: core::default::Default::default(),
            queue: self.queue,
            s3_uri: self.s3_uri,
            source_deletion_option: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AmazonSqsV2SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AmazonSqsV2SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_AmazonSqsV2SettingsRef {
        ChronicleFeed_Details_AmazonSqsV2SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AmazonSqsV2SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `chronicle_service_account` after provisioning.\nSA that will read data, this is Storage Transfer Service SA of Customer's\nTenancy Project."]
    pub fn chronicle_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chronicle_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_lookback_days` after provisioning.\nMaximum File Age to ingest in days."]
    pub fn max_lookback_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_lookback_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `queue` after provisioning.\nAmazon Resource Name(ARN) of the queue."]
    pub fn queue(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.queue", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_uri` after provisioning.\nS3 URI."]
    pub fn s3_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_deletion_option` after provisioning.\nPossible values:\nNEVER\nON_SUCCESS"]
    pub fn source_deletion_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_deletion_option", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AmazonSqsV2Settings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AwsIamSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_AwsIamSettings_Authentication {
    #[doc = "Set the field `secret`.\nSecret of the account identified by user_name."]
    pub fn set_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nUsername of an identity used for authentication."]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AwsIamSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AwsIamSettings_Authentication {}

impl BuildChronicleFeed_Details_AwsIamSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_AwsIamSettings_Authentication {
        ChronicleFeed_Details_AwsIamSettings_Authentication {
            secret: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AwsIamSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AwsIamSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AwsIamSettings_AuthenticationRef {
        ChronicleFeed_Details_AwsIamSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AwsIamSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nSecret of the account identified by user_name."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nUsername of an identity used for authentication."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AnomaliSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AnomaliSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_AnomaliSettingsDynamic,
}

impl ChronicleFeed_Details_AnomaliSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AnomaliSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AnomaliSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AnomaliSettings {}

impl BuildChronicleFeed_Details_AnomaliSettings {
    pub fn build(self) -> ChronicleFeed_Details_AnomaliSettings {
        ChronicleFeed_Details_AnomaliSettings {
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AnomaliSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AnomaliSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_AnomaliSettingsRef {
        ChronicleFeed_Details_AnomaliSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AnomaliSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AwsEc2HostsSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AwsEc2HostsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_AwsEc2HostsSettingsDynamic,
}

impl ChronicleFeed_Details_AwsEc2HostsSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AwsEc2HostsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AwsEc2HostsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AwsEc2HostsSettings {}

impl BuildChronicleFeed_Details_AwsEc2HostsSettings {
    pub fn build(self) -> ChronicleFeed_Details_AwsEc2HostsSettings {
        ChronicleFeed_Details_AwsEc2HostsSettings {
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AwsEc2HostsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AwsEc2HostsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_AwsEc2HostsSettingsRef {
        ChronicleFeed_Details_AwsEc2HostsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AwsEc2HostsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AwsEc2InstancesSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AwsEc2InstancesSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_AwsEc2InstancesSettingsDynamic,
}

impl ChronicleFeed_Details_AwsEc2InstancesSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AwsEc2InstancesSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AwsEc2InstancesSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AwsEc2InstancesSettings {}

impl BuildChronicleFeed_Details_AwsEc2InstancesSettings {
    pub fn build(self) -> ChronicleFeed_Details_AwsEc2InstancesSettings {
        ChronicleFeed_Details_AwsEc2InstancesSettings {
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AwsEc2InstancesSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AwsEc2InstancesSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AwsEc2InstancesSettingsRef {
        ChronicleFeed_Details_AwsEc2InstancesSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AwsEc2InstancesSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AwsEc2VpcsSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AwsEc2VpcsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_AwsEc2VpcsSettingsDynamic,
}

impl ChronicleFeed_Details_AwsEc2VpcsSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AwsEc2VpcsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AwsEc2VpcsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AwsEc2VpcsSettings {}

impl BuildChronicleFeed_Details_AwsEc2VpcsSettings {
    pub fn build(self) -> ChronicleFeed_Details_AwsEc2VpcsSettings {
        ChronicleFeed_Details_AwsEc2VpcsSettings {
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AwsEc2VpcsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AwsEc2VpcsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_AwsEc2VpcsSettingsRef {
        ChronicleFeed_Details_AwsEc2VpcsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AwsEc2VpcsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AwsIamSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AwsIamSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_AwsIamSettingsDynamic,
}

impl ChronicleFeed_Details_AwsIamSettings {
    #[doc = "Set the field `api_type`.\nSupported AWS IAM api type.\nPossible values:\nUSERS\nROLES\nGROUPS"]
    pub fn set_api_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AwsIamSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AwsIamSettings {}

impl BuildChronicleFeed_Details_AwsIamSettings {
    pub fn build(self) -> ChronicleFeed_Details_AwsIamSettings {
        ChronicleFeed_Details_AwsIamSettings {
            api_type: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AwsIamSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AwsIamSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_AwsIamSettingsRef {
        ChronicleFeed_Details_AwsIamSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AwsIamSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_type` after provisioning.\nSupported AWS IAM api type.\nPossible values:\nUSERS\nROLES\nGROUPS"]
    pub fn api_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_type", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AzureAdSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_AzureAdSettings_Authentication {
    #[doc = "Set the field `client_id`.\nClient ID."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nClient secret."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AzureAdSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AzureAdSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AzureAdSettings_Authentication {}

impl BuildChronicleFeed_Details_AzureAdSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_AzureAdSettings_Authentication {
        ChronicleFeed_Details_AzureAdSettings_Authentication {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AzureAdSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AzureAdSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AzureAdSettings_AuthenticationRef {
        ChronicleFeed_Details_AzureAdSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AzureAdSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nClient ID."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nClient secret."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AzureAdAuditSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AzureAdAuditSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_AzureAdAuditSettingsDynamic,
}

impl ChronicleFeed_Details_AzureAdAuditSettings {
    #[doc = "Set the field `auth_endpoint`.\nAPI Auth Endpoint."]
    pub fn set_auth_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\nTenant ID."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AzureAdAuditSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AzureAdAuditSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AzureAdAuditSettings {}

impl BuildChronicleFeed_Details_AzureAdAuditSettings {
    pub fn build(self) -> ChronicleFeed_Details_AzureAdAuditSettings {
        ChronicleFeed_Details_AzureAdAuditSettings {
            auth_endpoint: core::default::Default::default(),
            hostname: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AzureAdAuditSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AzureAdAuditSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_AzureAdAuditSettingsRef {
        ChronicleFeed_Details_AzureAdAuditSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AzureAdAuditSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_endpoint` after provisioning.\nAPI Auth Endpoint."]
    pub fn auth_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nTenant ID."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureAdSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AzureAdContextSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AzureAdContextSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieve_devices: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retrieve_groups: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_AzureAdContextSettingsDynamic,
}

impl ChronicleFeed_Details_AzureAdContextSettings {
    #[doc = "Set the field `auth_endpoint`.\nAPI Auth Endpoint."]
    pub fn set_auth_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retrieve_devices`.\nWhether to retrieve device information in user context."]
    pub fn set_retrieve_devices(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.retrieve_devices = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retrieve_groups`.\nWhether to retrieve group information in user context."]
    pub fn set_retrieve_groups(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.retrieve_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\nTenant ID."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AzureAdContextSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AzureAdContextSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AzureAdContextSettings {}

impl BuildChronicleFeed_Details_AzureAdContextSettings {
    pub fn build(self) -> ChronicleFeed_Details_AzureAdContextSettings {
        ChronicleFeed_Details_AzureAdContextSettings {
            auth_endpoint: core::default::Default::default(),
            hostname: core::default::Default::default(),
            retrieve_devices: core::default::Default::default(),
            retrieve_groups: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AzureAdContextSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AzureAdContextSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AzureAdContextSettingsRef {
        ChronicleFeed_Details_AzureAdContextSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AzureAdContextSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_endpoint` after provisioning.\nAPI Auth Endpoint."]
    pub fn auth_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retrieve_devices` after provisioning.\nWhether to retrieve device information in user context."]
    pub fn retrieve_devices(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retrieve_devices", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retrieve_groups` after provisioning.\nWhether to retrieve group information in user context."]
    pub fn retrieve_groups(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retrieve_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nTenant ID."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureAdSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AzureAdSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AzureAdSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_AzureAdSettingsDynamic,
}

impl ChronicleFeed_Details_AzureAdSettings {
    #[doc = "Set the field `auth_endpoint`.\nAPI Auth Endpoint."]
    pub fn set_auth_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\nTenant ID."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AzureAdSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AzureAdSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AzureAdSettings {}

impl BuildChronicleFeed_Details_AzureAdSettings {
    pub fn build(self) -> ChronicleFeed_Details_AzureAdSettings {
        ChronicleFeed_Details_AzureAdSettings {
            auth_endpoint: core::default::Default::default(),
            hostname: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AzureAdSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AzureAdSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_AzureAdSettingsRef {
        ChronicleFeed_Details_AzureAdSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AzureAdSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_endpoint` after provisioning.\nAPI Auth Endpoint."]
    pub fn auth_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nTenant ID."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureAdSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AzureBlobStoreSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sas_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_AzureBlobStoreSettings_Authentication {
    #[doc = "Set the field `sas_token`.\nSAS Token."]
    pub fn set_sas_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sas_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shared_key`.\nShared Key."]
    pub fn set_shared_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shared_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AzureBlobStoreSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AzureBlobStoreSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AzureBlobStoreSettings_Authentication {}

impl BuildChronicleFeed_Details_AzureBlobStoreSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_AzureBlobStoreSettings_Authentication {
        ChronicleFeed_Details_AzureBlobStoreSettings_Authentication {
            sas_token: core::default::Default::default(),
            shared_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AzureBlobStoreSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AzureBlobStoreSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AzureBlobStoreSettings_AuthenticationRef {
        ChronicleFeed_Details_AzureBlobStoreSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AzureBlobStoreSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sas_token` after provisioning.\nSAS Token."]
    pub fn sas_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sas_token", self.base))
    }

    #[doc = "Get a reference to the value of field `shared_key` after provisioning.\nShared Key."]
    pub fn shared_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shared_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AzureBlobStoreSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureBlobStoreSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AzureBlobStoreSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_deletion_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureBlobStoreSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_AzureBlobStoreSettingsDynamic,
}

impl ChronicleFeed_Details_AzureBlobStoreSettings {
    #[doc = "Set the field `azure_uri`.\nAzure URI."]
    pub fn set_azure_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.azure_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_deletion_option`.\nPossible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub fn set_source_deletion_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_deletion_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_type`.\nPossible values:\nFILES\nFOLDERS\nFOLDERS_RECURSIVE"]
    pub fn set_source_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureBlobStoreSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AzureBlobStoreSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AzureBlobStoreSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AzureBlobStoreSettings {}

impl BuildChronicleFeed_Details_AzureBlobStoreSettings {
    pub fn build(self) -> ChronicleFeed_Details_AzureBlobStoreSettings {
        ChronicleFeed_Details_AzureBlobStoreSettings {
            azure_uri: core::default::Default::default(),
            source_deletion_option: core::default::Default::default(),
            source_type: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AzureBlobStoreSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AzureBlobStoreSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AzureBlobStoreSettingsRef {
        ChronicleFeed_Details_AzureBlobStoreSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AzureBlobStoreSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `azure_uri` after provisioning.\nAzure URI."]
    pub fn azure_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_deletion_option` after provisioning.\nPossible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub fn source_deletion_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_deletion_option", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_type` after provisioning.\nPossible values:\nFILES\nFOLDERS\nFOLDERS_RECURSIVE"]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureBlobStoreSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederation {
    client_id: terrars::PrimField<std::string::String>,
    subject_id: terrars::PrimField<std::string::String>,
    tenant_id: terrars::PrimField<std::string::String>,
}

impl ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederation { }

impl terrars
::ToListMappable for ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederation {
    type O =
        terrars
        ::BlockAssignable<
            ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederation,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederation {
    #[doc = "OAuth client ID."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = "Subject ID of the Azure subscription."]
    pub subject_id: terrars::PrimField<std::string::String>,
    #[doc = "Tenant ID."]
    pub tenant_id: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederation {
    pub fn build(
        self,
    ) -> ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederation {
        ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederation {
            client_id: self.client_id,
            subject_id: self.subject_id,
            tenant_id: self.tenant_id,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederationRef {
        ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nOAuth client ID."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subject_id` after provisioning.\nSubject ID of the Azure subscription."]
    pub fn subject_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nTenant ID."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AzureBlobStoreV2Settings_AuthenticationDynamic {
    azure_v2_workload_identity_federation: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederation,
        >,
    >,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication {
    access_key: terrars::PrimField<std::string::String>,
    sas_token: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_v2_workload_identity_federation: std
    ::option
    ::Option<
        std::vec::Vec<ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederation>,
    >,
    dynamic: ChronicleFeed_Details_AzureBlobStoreV2Settings_AuthenticationDynamic,
}

impl ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication {
    #[doc = "Set the field `azure_v2_workload_identity_federation`.\n"]
    pub fn set_azure_v2_workload_identity_federation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_v2_workload_identity_federation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_v2_workload_identity_federation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication {
    #[doc = "Access Key also known as shared key."]
    pub access_key: terrars::PrimField<std::string::String>,
    #[doc = "SAS Token."]
    pub sas_token: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication {
        ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication {
            access_key: self.access_key,
            sas_token: self.sas_token,
            azure_v2_workload_identity_federation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AzureBlobStoreV2Settings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AzureBlobStoreV2Settings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AzureBlobStoreV2Settings_AuthenticationRef {
        ChronicleFeed_Details_AzureBlobStoreV2Settings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AzureBlobStoreV2Settings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access_key` after provisioning.\nAccess Key also known as shared key."]
    pub fn access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key", self.base))
    }

    #[doc = "Get a reference to the value of field `sas_token` after provisioning.\nSAS Token."]
    pub fn sas_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sas_token", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_v2_workload_identity_federation` after provisioning.\n"]
    pub fn azure_v2_workload_identity_federation(
        &self,
    ) -> terrars
    ::ListRef<ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication_AzureV2WorkloadIdentityFederationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_v2_workload_identity_federation", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AzureBlobStoreV2SettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AzureBlobStoreV2Settings {
    azure_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_lookback_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_deletion_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication>>,
    dynamic: ChronicleFeed_Details_AzureBlobStoreV2SettingsDynamic,
}

impl ChronicleFeed_Details_AzureBlobStoreV2Settings {
    #[doc = "Set the field `max_lookback_days`.\nMaximum File Age to ingest in days."]
    pub fn set_max_lookback_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_lookback_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_deletion_option`.\nPossible values:\nNEVER\nON_SUCCESS"]
    pub fn set_source_deletion_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_deletion_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureBlobStoreV2Settings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AzureBlobStoreV2Settings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AzureBlobStoreV2Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AzureBlobStoreV2Settings {
    #[doc = "Azure URI."]
    pub azure_uri: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_AzureBlobStoreV2Settings {
    pub fn build(self) -> ChronicleFeed_Details_AzureBlobStoreV2Settings {
        ChronicleFeed_Details_AzureBlobStoreV2Settings {
            azure_uri: self.azure_uri,
            max_lookback_days: core::default::Default::default(),
            source_deletion_option: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AzureBlobStoreV2SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AzureBlobStoreV2SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AzureBlobStoreV2SettingsRef {
        ChronicleFeed_Details_AzureBlobStoreV2SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AzureBlobStoreV2SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `azure_uri` after provisioning.\nAzure URI."]
    pub fn azure_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `chronicle_service_account` after provisioning.\nSA that will read data, this is Storage Transfer Service SA of Customer's\nTenancy Project."]
    pub fn chronicle_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chronicle_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_lookback_days` after provisioning.\nMaximum File Age to ingest in days."]
    pub fn max_lookback_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_lookback_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_deletion_option` after provisioning.\nPossible values:\nNEVER\nON_SUCCESS"]
    pub fn source_deletion_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_deletion_option", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureBlobStoreV2Settings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AzureEventHubSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_sas_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_storage_connection_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_storage_container: std::option::Option<terrars::PrimField<std::string::String>>,
    consumer_group: terrars::PrimField<std::string::String>,
    event_hub_connection_string: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl ChronicleFeed_Details_AzureEventHubSettings {
    #[doc = "Set the field `azure_sas_token`.\nSAS token"]
    pub fn set_azure_sas_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.azure_sas_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure_storage_connection_string`.\nBlob store connection string for authentication."]
    pub fn set_azure_storage_connection_string(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.azure_storage_connection_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `azure_storage_container`.\nBlob storage container name."]
    pub fn set_azure_storage_container(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.azure_storage_container = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AzureEventHubSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AzureEventHubSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AzureEventHubSettings {
    #[doc = "Event hub consumer group to read from."]
    pub consumer_group: terrars::PrimField<std::string::String>,
    #[doc = "Event hub connection string for authentication."]
    pub event_hub_connection_string: terrars::PrimField<std::string::String>,
    #[doc = "Event hub to read from."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_AzureEventHubSettings {
    pub fn build(self) -> ChronicleFeed_Details_AzureEventHubSettings {
        ChronicleFeed_Details_AzureEventHubSettings {
            azure_sas_token: core::default::Default::default(),
            azure_storage_connection_string: core::default::Default::default(),
            azure_storage_container: core::default::Default::default(),
            consumer_group: self.consumer_group,
            event_hub_connection_string: self.event_hub_connection_string,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AzureEventHubSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AzureEventHubSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AzureEventHubSettingsRef {
        ChronicleFeed_Details_AzureEventHubSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AzureEventHubSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `azure_sas_token` after provisioning.\nSAS token"]
    pub fn azure_sas_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_sas_token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `azure_storage_connection_string` after provisioning.\nBlob store connection string for authentication."]
    pub fn azure_storage_connection_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_storage_connection_string", self.base))
    }

    #[doc =
        "Get a reference to the value of field `azure_storage_container` after provisioning.\nBlob storage container name."]
    pub fn azure_storage_container(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.azure_storage_container", self.base))
    }

    #[doc =
        "Get a reference to the value of field `consumer_group` after provisioning.\nEvent hub consumer group to read from."]
    pub fn consumer_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consumer_group", self.base))
    }

    #[doc =
        "Get a reference to the value of field `event_hub_connection_string` after provisioning.\nEvent hub connection string for authentication."]
    pub fn event_hub_connection_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_hub_connection_string", self.base))
    }

    #[doc = "Get a reference to the value of field `event_hub_namespace` after provisioning.\nEvent hub namespace"]
    pub fn event_hub_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_hub_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nEvent hub to read from."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_AzureMdmIntuneSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_AzureMdmIntuneSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_AzureMdmIntuneSettingsDynamic,
}

impl ChronicleFeed_Details_AzureMdmIntuneSettings {
    #[doc = "Set the field `auth_endpoint`.\nAPI Auth Endpoint."]
    pub fn set_auth_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\nTenant ID."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_AzureMdmIntuneSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_AzureMdmIntuneSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_AzureMdmIntuneSettings {}

impl BuildChronicleFeed_Details_AzureMdmIntuneSettings {
    pub fn build(self) -> ChronicleFeed_Details_AzureMdmIntuneSettings {
        ChronicleFeed_Details_AzureMdmIntuneSettings {
            auth_endpoint: core::default::Default::default(),
            hostname: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_AzureMdmIntuneSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_AzureMdmIntuneSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_AzureMdmIntuneSettingsRef {
        ChronicleFeed_Details_AzureMdmIntuneSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_AzureMdmIntuneSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_endpoint` after provisioning.\nAPI Auth Endpoint."]
    pub fn auth_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nTenant ID."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureAdSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_CloudPassageSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_CloudPassageSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_CloudPassageSettingsDynamic,
}

impl ChronicleFeed_Details_CloudPassageSettings {
    #[doc = "Set the field `event_types`.\nEvent types filter for the events API."]
    pub fn set_event_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.event_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_CloudPassageSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_CloudPassageSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_CloudPassageSettings {}

impl BuildChronicleFeed_Details_CloudPassageSettings {
    pub fn build(self) -> ChronicleFeed_Details_CloudPassageSettings {
        ChronicleFeed_Details_CloudPassageSettings {
            event_types: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_CloudPassageSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_CloudPassageSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_CloudPassageSettingsRef {
        ChronicleFeed_Details_CloudPassageSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_CloudPassageSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `event_types` after provisioning.\nEvent types filter for the events API."]
    pub fn event_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_types", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValues {
    #[doc = "Set the field `key`.\nKey."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nValue."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValues {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValues {}

impl BuildChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValues {
    pub fn build(self) -> ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValues {
        ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValues {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValuesRef {
        ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_OktaSettings_AuthenticationDynamic {
    header_key_values: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValues>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_OktaSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_key_values: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValues>>,
    dynamic: ChronicleFeed_Details_OktaSettings_AuthenticationDynamic,
}

impl ChronicleFeed_Details_OktaSettings_Authentication {
    #[doc = "Set the field `header_key_values`.\n"]
    pub fn set_header_key_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValues>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_key_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_key_values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_OktaSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_OktaSettings_Authentication {}

impl BuildChronicleFeed_Details_OktaSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_OktaSettings_Authentication {
        ChronicleFeed_Details_OktaSettings_Authentication {
            header_key_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_OktaSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_OktaSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_OktaSettings_AuthenticationRef {
        ChronicleFeed_Details_OktaSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_OktaSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_key_values` after provisioning.\n"]
    pub fn header_key_values(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_Authentication_HeaderKeyValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_key_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_CortexXdrSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_CortexXdrSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_CortexXdrSettingsDynamic,
}

impl ChronicleFeed_Details_CortexXdrSettings {
    #[doc = "Set the field `endpoint`.\nAPI Endpoint."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_CortexXdrSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_CortexXdrSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_CortexXdrSettings {}

impl BuildChronicleFeed_Details_CortexXdrSettings {
    pub fn build(self) -> ChronicleFeed_Details_CortexXdrSettings {
        ChronicleFeed_Details_CortexXdrSettings {
            endpoint: core::default::Default::default(),
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_CortexXdrSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_CortexXdrSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_CortexXdrSettingsRef {
        ChronicleFeed_Details_CortexXdrSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_CortexXdrSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\nAPI Endpoint."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_RhIsacIocSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_RhIsacIocSettings_Authentication {
    #[doc = "Set the field `client_id`.\nClient ID."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nClient secret."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\nToken endpoint."]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_RhIsacIocSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_RhIsacIocSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_RhIsacIocSettings_Authentication {}

impl BuildChronicleFeed_Details_RhIsacIocSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_RhIsacIocSettings_Authentication {
        ChronicleFeed_Details_RhIsacIocSettings_Authentication {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_RhIsacIocSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_RhIsacIocSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_RhIsacIocSettings_AuthenticationRef {
        ChronicleFeed_Details_RhIsacIocSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_RhIsacIocSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nClient ID."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nClient secret."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\nToken endpoint."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_CrowdstrikeAlertsSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_RhIsacIocSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_CrowdstrikeAlertsSettings {
    hostname: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingestion_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_RhIsacIocSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_CrowdstrikeAlertsSettingsDynamic,
}

impl ChronicleFeed_Details_CrowdstrikeAlertsSettings {
    #[doc =
        "Set the field `ingestion_type`.\nIngestion Type.\nPossible values:\nBRING_ALL_ALERTS\nBRING_ONLY_NEW_ALERTS"]
    pub fn set_ingestion_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ingestion_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_RhIsacIocSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_CrowdstrikeAlertsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_CrowdstrikeAlertsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_CrowdstrikeAlertsSettings {
    #[doc = "API Hostname."]
    pub hostname: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_CrowdstrikeAlertsSettings {
    pub fn build(self) -> ChronicleFeed_Details_CrowdstrikeAlertsSettings {
        ChronicleFeed_Details_CrowdstrikeAlertsSettings {
            hostname: self.hostname,
            ingestion_type: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_CrowdstrikeAlertsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_CrowdstrikeAlertsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_CrowdstrikeAlertsSettingsRef {
        ChronicleFeed_Details_CrowdstrikeAlertsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_CrowdstrikeAlertsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ingestion_type` after provisioning.\nIngestion Type.\nPossible values:\nBRING_ALL_ALERTS\nBRING_ONLY_NEW_ALERTS"]
    pub fn ingestion_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingestion_type", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_RhIsacIocSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_CrowdstrikeDetectsSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_RhIsacIocSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_CrowdstrikeDetectsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingestion_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_RhIsacIocSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_CrowdstrikeDetectsSettingsDynamic,
}

impl ChronicleFeed_Details_CrowdstrikeDetectsSettings {
    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ingestion_type`.\nIngestion Type.\nPossible values:\nBRING_ONLY_NEW_DETECTIONS\nBRING_ALL_DETECTIONS"]
    pub fn set_ingestion_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ingestion_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_RhIsacIocSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_CrowdstrikeDetectsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_CrowdstrikeDetectsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_CrowdstrikeDetectsSettings {}

impl BuildChronicleFeed_Details_CrowdstrikeDetectsSettings {
    pub fn build(self) -> ChronicleFeed_Details_CrowdstrikeDetectsSettings {
        ChronicleFeed_Details_CrowdstrikeDetectsSettings {
            hostname: core::default::Default::default(),
            ingestion_type: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_CrowdstrikeDetectsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_CrowdstrikeDetectsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_CrowdstrikeDetectsSettingsRef {
        ChronicleFeed_Details_CrowdstrikeDetectsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_CrowdstrikeDetectsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ingestion_type` after provisioning.\nIngestion Type.\nPossible values:\nBRING_ONLY_NEW_DETECTIONS\nBRING_ALL_DETECTIONS"]
    pub fn ingestion_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingestion_type", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_RhIsacIocSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_DummyLogTypeSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_DummyLogTypeSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_DummyLogTypeSettingsDynamic,
}

impl ChronicleFeed_Details_DummyLogTypeSettings {
    #[doc = "Set the field `api_endpoint`.\nFull API Endpoint."]
    pub fn set_api_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_DummyLogTypeSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_DummyLogTypeSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_DummyLogTypeSettings {}

impl BuildChronicleFeed_Details_DummyLogTypeSettings {
    pub fn build(self) -> ChronicleFeed_Details_DummyLogTypeSettings {
        ChronicleFeed_Details_DummyLogTypeSettings {
            api_endpoint: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_DummyLogTypeSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_DummyLogTypeSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_DummyLogTypeSettingsRef {
        ChronicleFeed_Details_DummyLogTypeSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_DummyLogTypeSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_endpoint` after provisioning.\nFull API Endpoint."]
    pub fn api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_DuoAuthSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_DuoAuthSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_DuoAuthSettingsDynamic,
}

impl ChronicleFeed_Details_DuoAuthSettings {
    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_DuoAuthSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_DuoAuthSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_DuoAuthSettings {}

impl BuildChronicleFeed_Details_DuoAuthSettings {
    pub fn build(self) -> ChronicleFeed_Details_DuoAuthSettings {
        ChronicleFeed_Details_DuoAuthSettings {
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_DuoAuthSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_DuoAuthSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_DuoAuthSettingsRef {
        ChronicleFeed_Details_DuoAuthSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_DuoAuthSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_DuoUserContextSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_DuoUserContextSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_DuoUserContextSettingsDynamic,
}

impl ChronicleFeed_Details_DuoUserContextSettings {
    #[doc = "Set the field `hostname`.\nAPI hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_DuoUserContextSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_DuoUserContextSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_DuoUserContextSettings {}

impl BuildChronicleFeed_Details_DuoUserContextSettings {
    pub fn build(self) -> ChronicleFeed_Details_DuoUserContextSettings {
        ChronicleFeed_Details_DuoUserContextSettings {
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_DuoUserContextSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_DuoUserContextSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_DuoUserContextSettingsRef {
        ChronicleFeed_Details_DuoUserContextSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_DuoUserContextSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_FoxItStixSettings_Ssl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoded_private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_FoxItStixSettings_Ssl {
    #[doc =
        "Set the field `encoded_private_key`.\nThe encoded private key. The string should be a private key in PEM format,\nand should include the begin header and end footer lines. It may also\ninclude newlines.\n\nExample:\n-----BEGIN RSA PRIVATE KEY-----\nProc-Type: 4,ENCRYPTED\nDEK-Info: DES-EDE3-CBC,F23074E02CF47304\n\n-----END RSA PRIVATE KEY-----"]
    pub fn set_encoded_private_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.encoded_private_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ssl_certificate`.\nThe encoded SSL certificate. The string should be an SSL certificate in\nPEM format, and should include the begin header and end footer lines. It\nmay also include newlines.\n\nExample:\n-----BEGIN CERTIFICATE-----\n\n-----END CERTIFICATE-----"]
    pub fn set_ssl_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ssl_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_FoxItStixSettings_Ssl {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_FoxItStixSettings_Ssl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_FoxItStixSettings_Ssl {}

impl BuildChronicleFeed_Details_FoxItStixSettings_Ssl {
    pub fn build(self) -> ChronicleFeed_Details_FoxItStixSettings_Ssl {
        ChronicleFeed_Details_FoxItStixSettings_Ssl {
            encoded_private_key: core::default::Default::default(),
            ssl_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_FoxItStixSettings_SslRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_FoxItStixSettings_SslRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_FoxItStixSettings_SslRef {
        ChronicleFeed_Details_FoxItStixSettings_SslRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_FoxItStixSettings_SslRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `encoded_private_key` after provisioning.\nThe encoded private key. The string should be a private key in PEM format,\nand should include the begin header and end footer lines. It may also\ninclude newlines.\n\nExample:\n-----BEGIN RSA PRIVATE KEY-----\nProc-Type: 4,ENCRYPTED\nDEK-Info: DES-EDE3-CBC,F23074E02CF47304\n\n-----END RSA PRIVATE KEY-----"]
    pub fn encoded_private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoded_private_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_certificate` after provisioning.\nThe encoded SSL certificate. The string should be an SSL certificate in\nPEM format, and should include the begin header and end footer lines. It\nmay also include newlines.\n\nExample:\n-----BEGIN CERTIFICATE-----\n\n-----END CERTIFICATE-----"]
    pub fn ssl_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_certificate", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_FoxItStixSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ssl: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_FoxItStixSettings_Ssl>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_FoxItStixSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    poll_service_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl: std::option::Option<std::vec::Vec<ChronicleFeed_Details_FoxItStixSettings_Ssl>>,
    dynamic: ChronicleFeed_Details_FoxItStixSettingsDynamic,
}

impl ChronicleFeed_Details_FoxItStixSettings {
    #[doc = "Set the field `collection`.\nCollection available at the poll service."]
    pub fn set_collection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `poll_service_uri`.\nTAXII poll service URI."]
    pub fn set_poll_service_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.poll_service_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssl`.\n"]
    pub fn set_ssl(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_FoxItStixSettings_Ssl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssl = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssl = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_FoxItStixSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_FoxItStixSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_FoxItStixSettings {}

impl BuildChronicleFeed_Details_FoxItStixSettings {
    pub fn build(self) -> ChronicleFeed_Details_FoxItStixSettings {
        ChronicleFeed_Details_FoxItStixSettings {
            collection: core::default::Default::default(),
            poll_service_uri: core::default::Default::default(),
            authentication: core::default::Default::default(),
            ssl: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_FoxItStixSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_FoxItStixSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_FoxItStixSettingsRef {
        ChronicleFeed_Details_FoxItStixSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_FoxItStixSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `collection` after provisioning.\nCollection available at the poll service."]
    pub fn collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection", self.base))
    }

    #[doc = "Get a reference to the value of field `poll_service_uri` after provisioning.\nTAXII poll service URI."]
    pub fn poll_service_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.poll_service_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl` after provisioning.\n"]
    pub fn ssl(&self) -> terrars::ListRef<ChronicleFeed_Details_FoxItStixSettings_SslRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_GcsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_deletion_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_GcsSettings {
    #[doc = "Set the field `bucket_uri`.\nBucket URI."]
    pub fn set_bucket_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `source_deletion_option`.\nPossible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub fn set_source_deletion_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_deletion_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_type`.\nPossible values:\nFILES\nFOLDERS\nFOLDERS_RECURSIVE"]
    pub fn set_source_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_GcsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_GcsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_GcsSettings {}

impl BuildChronicleFeed_Details_GcsSettings {
    pub fn build(self) -> ChronicleFeed_Details_GcsSettings {
        ChronicleFeed_Details_GcsSettings {
            bucket_uri: core::default::Default::default(),
            source_deletion_option: core::default::Default::default(),
            source_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_GcsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_GcsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_GcsSettingsRef {
        ChronicleFeed_Details_GcsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_GcsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_uri` after provisioning.\nBucket URI."]
    pub fn bucket_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `chronicle_service_account` after provisioning.\nService Account Chronicle will be using to pull data."]
    pub fn chronicle_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chronicle_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_deletion_option` after provisioning.\nPossible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub fn source_deletion_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_deletion_option", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_type` after provisioning.\nPossible values:\nFILES\nFOLDERS\nFOLDERS_RECURSIVE"]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_GcsV2Settings {
    bucket_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_lookback_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_deletion_option: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_GcsV2Settings {
    #[doc = "Set the field `max_lookback_days`.\nMaximum File Age to ingest in days."]
    pub fn set_max_lookback_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_lookback_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_deletion_option`.\nPossible values:\nNEVER\nON_SUCCESS"]
    pub fn set_source_deletion_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_deletion_option = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_GcsV2Settings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_GcsV2Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_GcsV2Settings {
    #[doc = "Google Cloud Storage Bucket URI for the feed."]
    pub bucket_uri: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_GcsV2Settings {
    pub fn build(self) -> ChronicleFeed_Details_GcsV2Settings {
        ChronicleFeed_Details_GcsV2Settings {
            bucket_uri: self.bucket_uri,
            max_lookback_days: core::default::Default::default(),
            source_deletion_option: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_GcsV2SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_GcsV2SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_GcsV2SettingsRef {
        ChronicleFeed_Details_GcsV2SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_GcsV2SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_uri` after provisioning.\nGoogle Cloud Storage Bucket URI for the feed."]
    pub fn bucket_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `chronicle_service_account` after provisioning.\nSA that will read data, this is Storage Transfer Service SA of Customer's\nTenancy Project."]
    pub fn chronicle_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chronicle_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_lookback_days` after provisioning.\nMaximum File Age to ingest in days."]
    pub fn max_lookback_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_lookback_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_deletion_option` after provisioning.\nPossible values:\nNEVER\nON_SUCCESS"]
    pub fn source_deletion_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_deletion_option", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audience: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    issuer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims {
    #[doc = "Set the field `audience`.\nAudience."]
    pub fn set_audience(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audience = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `issuer`.\nIssuer. Usually the client_id."]
    pub fn set_issuer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.issuer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject`.\nSubject. Usually the email."]
    pub fn set_subject(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subject = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims {}

impl BuildChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims {
    pub fn build(self) -> ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims {
        ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims {
            audience: core::default::Default::default(),
            issuer: core::default::Default::default(),
            subject: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_ClaimsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_ClaimsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_ClaimsRef {
        ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_ClaimsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_ClaimsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audience` after provisioning.\nAudience."]
    pub fn audience(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }

    #[doc = "Get a reference to the value of field `issuer` after provisioning.\nIssuer. Usually the client_id."]
    pub fn issuer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.issuer", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\nSubject. Usually the email."]
    pub fn subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subject", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials {
    #[doc = "Set the field `private_key`.\nPrivate key in PEM format."]
    pub fn set_private_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials {}

impl BuildChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials {
    pub fn build(self) -> ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials {
        ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials {
            private_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentialsRef {
        ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\nPrivate key in PEM format."]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationDynamic {
    claims: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims>>,
    rs_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkspaceUsersSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    claims: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rs_credentials: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials>>,
    dynamic: ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationDynamic,
}

impl ChronicleFeed_Details_WorkspaceUsersSettings_Authentication {
    #[doc = "Set the field `token_endpoint`.\nToken endpoint to get the OAuth token from."]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `claims`.\n"]
    pub fn set_claims(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.claims = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.claims = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rs_credentials`.\n"]
    pub fn set_rs_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rs_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rs_credentials = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkspaceUsersSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkspaceUsersSettings_Authentication {}

impl BuildChronicleFeed_Details_WorkspaceUsersSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_WorkspaceUsersSettings_Authentication {
        ChronicleFeed_Details_WorkspaceUsersSettings_Authentication {
            token_endpoint: core::default::Default::default(),
            claims: core::default::Default::default(),
            rs_credentials: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef {
        ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nToken endpoint to get the OAuth token from."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `claims` after provisioning.\n"]
    pub fn claims(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_ClaimsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.claims", self.base))
    }

    #[doc = "Get a reference to the value of field `rs_credentials` after provisioning.\n"]
    pub fn rs_credentials(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rs_credentials", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettingsDynamic,
}

impl ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettings {}

impl BuildChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettings {
    pub fn build(self) -> ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettings {
        ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettings {
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettingsRef {
        ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_GoogleCloudIdentityDevicesSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_GoogleCloudIdentityDevicesSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_GoogleCloudIdentityDevicesSettingsDynamic,
}

impl ChronicleFeed_Details_GoogleCloudIdentityDevicesSettings {
    #[doc = "Set the field `api_version`.\nAPI Version"]
    pub fn set_api_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_GoogleCloudIdentityDevicesSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_GoogleCloudIdentityDevicesSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_GoogleCloudIdentityDevicesSettings {}

impl BuildChronicleFeed_Details_GoogleCloudIdentityDevicesSettings {
    pub fn build(self) -> ChronicleFeed_Details_GoogleCloudIdentityDevicesSettings {
        ChronicleFeed_Details_GoogleCloudIdentityDevicesSettings {
            api_version: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_GoogleCloudIdentityDevicesSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_GoogleCloudIdentityDevicesSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_GoogleCloudIdentityDevicesSettingsRef {
        ChronicleFeed_Details_GoogleCloudIdentityDevicesSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_GoogleCloudIdentityDevicesSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_version` after provisioning.\nAPI Version"]
    pub fn api_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_version", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettings {
    bucket_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_lookback_days: std::option::Option<terrars::PrimField<f64>>,
    pubsub_subscription: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_deletion_option: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettings {
    #[doc = "Set the field `max_lookback_days`.\nMaximum File Age to ingest in days."]
    pub fn set_max_lookback_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_lookback_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_deletion_option`.\nPossible values:\nNEVER\nON_SUCCESS"]
    pub fn set_source_deletion_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_deletion_option = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_GoogleCloudStorageEventDrivenSettings {
    #[doc = "Google Cloud Storage Bucket URI for the feed."]
    pub bucket_uri: terrars::PrimField<std::string::String>,
    #[doc = "Subscription name for pubsub topic."]
    pub pubsub_subscription: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_GoogleCloudStorageEventDrivenSettings {
    pub fn build(self) -> ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettings {
        ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettings {
            bucket_uri: self.bucket_uri,
            max_lookback_days: core::default::Default::default(),
            pubsub_subscription: self.pubsub_subscription,
            source_deletion_option: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettingsRef {
        ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_uri` after provisioning.\nGoogle Cloud Storage Bucket URI for the feed."]
    pub fn bucket_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `chronicle_service_account` after provisioning.\nSA that will read data, this is Storage Transfer Service SA of Customer's\nTenancy Project."]
    pub fn chronicle_service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chronicle_service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_lookback_days` after provisioning.\nMaximum File Age to ingest in days."]
    pub fn max_lookback_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_lookback_days", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pubsub_subscription` after provisioning.\nSubscription name for pubsub topic."]
    pub fn pubsub_subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pubsub_subscription", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_deletion_option` after provisioning.\nPossible values:\nNEVER\nON_SUCCESS"]
    pub fn source_deletion_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_deletion_option", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_HttpSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_deletion_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_HttpSettings {
    #[doc =
        "Set the field `source_deletion_option`.\nPossible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub fn set_source_deletion_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_deletion_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_type`.\nPossible values:\nFILES\nFOLDERS\nFOLDERS_RECURSIVE"]
    pub fn set_source_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\nHTTP URI."]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_HttpSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_HttpSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_HttpSettings {}

impl BuildChronicleFeed_Details_HttpSettings {
    pub fn build(self) -> ChronicleFeed_Details_HttpSettings {
        ChronicleFeed_Details_HttpSettings {
            source_deletion_option: core::default::Default::default(),
            source_type: core::default::Default::default(),
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_HttpSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_HttpSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_HttpSettingsRef {
        ChronicleFeed_Details_HttpSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_HttpSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `source_deletion_option` after provisioning.\nPossible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub fn source_deletion_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_deletion_option", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_type` after provisioning.\nPossible values:\nFILES\nFOLDERS\nFOLDERS_RECURSIVE"]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nHTTP URI."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    split_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettings {
    #[doc = "Set the field `split_delimiter`.\nDelimiter to split on for the feed."]
    pub fn set_split_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.split_delimiter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettings {}

impl BuildChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettings {
    pub fn build(self) -> ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettings {
        ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettings {
            split_delimiter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettingsRef {
        ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `split_delimiter` after provisioning.\nDelimiter to split on for the feed."]
    pub fn split_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.split_delimiter", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    split_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettings {
    #[doc = "Set the field `split_delimiter`.\nDelimiter to split on for the feed."]
    pub fn set_split_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.split_delimiter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettings {}

impl BuildChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettings {
    pub fn build(self) -> ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettings {
        ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettings {
            split_delimiter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettingsRef {
        ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `split_delimiter` after provisioning.\nDelimiter to split on for the feed."]
    pub fn split_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.split_delimiter", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_HttpsPushWebhookSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    split_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_HttpsPushWebhookSettings {
    #[doc = "Set the field `split_delimiter`.\nDelimiter to split on for the feed."]
    pub fn set_split_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.split_delimiter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_HttpsPushWebhookSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_HttpsPushWebhookSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_HttpsPushWebhookSettings {}

impl BuildChronicleFeed_Details_HttpsPushWebhookSettings {
    pub fn build(self) -> ChronicleFeed_Details_HttpsPushWebhookSettings {
        ChronicleFeed_Details_HttpsPushWebhookSettings { split_delimiter: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_HttpsPushWebhookSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_HttpsPushWebhookSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_HttpsPushWebhookSettingsRef {
        ChronicleFeed_Details_HttpsPushWebhookSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_HttpsPushWebhookSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `split_delimiter` after provisioning.\nDelimiter to split on for the feed."]
    pub fn split_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.split_delimiter", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_ImpervaWafSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_ImpervaWafSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_ImpervaWafSettingsDynamic,
}

impl ChronicleFeed_Details_ImpervaWafSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_ImpervaWafSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_ImpervaWafSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_ImpervaWafSettings {}

impl BuildChronicleFeed_Details_ImpervaWafSettings {
    pub fn build(self) -> ChronicleFeed_Details_ImpervaWafSettings {
        ChronicleFeed_Details_ImpervaWafSettings {
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_ImpervaWafSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_ImpervaWafSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_ImpervaWafSettingsRef {
        ChronicleFeed_Details_ImpervaWafSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_ImpervaWafSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_MandiantIocSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_MandiantIocSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_MandiantIocSettingsDynamic,
}

impl ChronicleFeed_Details_MandiantIocSettings {
    #[doc = "Set the field `start_time`.\ntime since when to start fetching the IOCs"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_MandiantIocSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_MandiantIocSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_MandiantIocSettings {}

impl BuildChronicleFeed_Details_MandiantIocSettings {
    pub fn build(self) -> ChronicleFeed_Details_MandiantIocSettings {
        ChronicleFeed_Details_MandiantIocSettings {
            start_time: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_MandiantIocSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_MandiantIocSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_MandiantIocSettingsRef {
        ChronicleFeed_Details_MandiantIocSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_MandiantIocSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\ntime since when to start fetching the IOCs"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_MicrosoftGraphAlertSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_MicrosoftGraphAlertSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_MicrosoftGraphAlertSettingsDynamic,
}

impl ChronicleFeed_Details_MicrosoftGraphAlertSettings {
    #[doc = "Set the field `auth_endpoint`.\nAPI Auth Endpoint."]
    pub fn set_auth_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\nTenant ID."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_MicrosoftGraphAlertSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_MicrosoftGraphAlertSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_MicrosoftGraphAlertSettings {}

impl BuildChronicleFeed_Details_MicrosoftGraphAlertSettings {
    pub fn build(self) -> ChronicleFeed_Details_MicrosoftGraphAlertSettings {
        ChronicleFeed_Details_MicrosoftGraphAlertSettings {
            auth_endpoint: core::default::Default::default(),
            hostname: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_MicrosoftGraphAlertSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_MicrosoftGraphAlertSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_MicrosoftGraphAlertSettingsRef {
        ChronicleFeed_Details_MicrosoftGraphAlertSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_MicrosoftGraphAlertSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_endpoint` after provisioning.\nAPI Auth Endpoint."]
    pub fn auth_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nTenant ID."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureAdSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscription_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettingsDynamic,
}

impl ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettings {
    #[doc = "Set the field `auth_endpoint`.\nAPI Auth Endpoint."]
    pub fn set_auth_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subscription_id`.\nSubscription ID of the Microsoft security center alert settings alert."]
    pub fn set_subscription_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subscription_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\nTenant ID."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_MicrosoftSecurityCenterAlertSettings {}

impl BuildChronicleFeed_Details_MicrosoftSecurityCenterAlertSettings {
    pub fn build(self) -> ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettings {
        ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettings {
            auth_endpoint: core::default::Default::default(),
            hostname: core::default::Default::default(),
            subscription_id: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettingsRef {
        ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_endpoint` after provisioning.\nAPI Auth Endpoint."]
    pub fn auth_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subscription_id` after provisioning.\nSubscription ID of the Microsoft security center alert settings alert."]
    pub fn subscription_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_id", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nTenant ID."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureAdSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_MimecastMailSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_MimecastMailSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_MimecastMailSettingsDynamic,
}

impl ChronicleFeed_Details_MimecastMailSettings {
    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_MimecastMailSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_MimecastMailSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_MimecastMailSettings {}

impl BuildChronicleFeed_Details_MimecastMailSettings {
    pub fn build(self) -> ChronicleFeed_Details_MimecastMailSettings {
        ChronicleFeed_Details_MimecastMailSettings {
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_MimecastMailSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_MimecastMailSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_MimecastMailSettingsRef {
        ChronicleFeed_Details_MimecastMailSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_MimecastMailSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentials {
    #[doc = "Set the field `client_id`.\nClient ID."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nClient Secret."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentials {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_MimecastMailV2Settings_AuthCredentials {}

impl BuildChronicleFeed_Details_MimecastMailV2Settings_AuthCredentials {
    pub fn build(self) -> ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentials {
        ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentials {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentialsRef {
        ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nClient ID."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nClient Secret."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_MimecastMailV2SettingsDynamic {
    auth_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentials>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_MimecastMailV2Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_credentials: std::option::Option<std::vec::Vec<ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentials>>,
    dynamic: ChronicleFeed_Details_MimecastMailV2SettingsDynamic,
}

impl ChronicleFeed_Details_MimecastMailV2Settings {
    #[doc = "Set the field `auth_credentials`.\n"]
    pub fn set_auth_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentials>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_credentials = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_MimecastMailV2Settings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_MimecastMailV2Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_MimecastMailV2Settings {}

impl BuildChronicleFeed_Details_MimecastMailV2Settings {
    pub fn build(self) -> ChronicleFeed_Details_MimecastMailV2Settings {
        ChronicleFeed_Details_MimecastMailV2Settings {
            auth_credentials: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_MimecastMailV2SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_MimecastMailV2SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_MimecastMailV2SettingsRef {
        ChronicleFeed_Details_MimecastMailV2SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_MimecastMailV2SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_credentials` after provisioning.\n"]
    pub fn auth_credentials(&self) -> terrars::ListRef<ChronicleFeed_Details_MimecastMailV2Settings_AuthCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_credentials", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_NetskopeAlertSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_NetskopeAlertSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feedname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_NetskopeAlertSettingsDynamic,
}

impl ChronicleFeed_Details_NetskopeAlertSettings {
    #[doc = "Set the field `content_type`.\nContent type."]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feedname`.\nFeedname."]
    pub fn set_feedname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feedname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_NetskopeAlertSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_NetskopeAlertSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_NetskopeAlertSettings {}

impl BuildChronicleFeed_Details_NetskopeAlertSettings {
    pub fn build(self) -> ChronicleFeed_Details_NetskopeAlertSettings {
        ChronicleFeed_Details_NetskopeAlertSettings {
            content_type: core::default::Default::default(),
            feedname: core::default::Default::default(),
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_NetskopeAlertSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_NetskopeAlertSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_NetskopeAlertSettingsRef {
        ChronicleFeed_Details_NetskopeAlertSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_NetskopeAlertSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\nContent type."]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `feedname` after provisioning.\nFeedname."]
    pub fn feedname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feedname", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_NetskopeAlertV2SettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_NetskopeAlertV2Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_category: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_NetskopeAlertV2SettingsDynamic,
}

impl ChronicleFeed_Details_NetskopeAlertV2Settings {
    #[doc = "Set the field `content_category`.\nContent Category."]
    pub fn set_content_category(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_types`.\nContent type."]
    pub fn set_content_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.content_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_NetskopeAlertV2Settings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_NetskopeAlertV2Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_NetskopeAlertV2Settings {}

impl BuildChronicleFeed_Details_NetskopeAlertV2Settings {
    pub fn build(self) -> ChronicleFeed_Details_NetskopeAlertV2Settings {
        ChronicleFeed_Details_NetskopeAlertV2Settings {
            content_category: core::default::Default::default(),
            content_types: core::default::Default::default(),
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_NetskopeAlertV2SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_NetskopeAlertV2SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_NetskopeAlertV2SettingsRef {
        ChronicleFeed_Details_NetskopeAlertV2SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_NetskopeAlertV2SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_category` after provisioning.\nContent Category."]
    pub fn content_category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_category", self.base))
    }

    #[doc = "Get a reference to the value of field `content_types` after provisioning.\nContent type."]
    pub fn content_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_types", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_Office365SettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_Office365Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_Office365SettingsDynamic,
}

impl ChronicleFeed_Details_Office365Settings {
    #[doc = "Set the field `auth_endpoint`.\nAPI Auth Endpoint."]
    pub fn set_auth_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auth_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_type`.\nSupported office 365 content type.\nPossible values:\nAUDIT_AZURE_ACTIVE_DIRECTORY\nAUDIT_EXCHANGE\nAUDIT_SHARE_POINT\nAUDIT_GENERAL\nDLP_ALL"]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\nTenant ID."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureAdSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_Office365Settings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_Office365Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_Office365Settings {}

impl BuildChronicleFeed_Details_Office365Settings {
    pub fn build(self) -> ChronicleFeed_Details_Office365Settings {
        ChronicleFeed_Details_Office365Settings {
            auth_endpoint: core::default::Default::default(),
            content_type: core::default::Default::default(),
            hostname: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_Office365SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_Office365SettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_Office365SettingsRef {
        ChronicleFeed_Details_Office365SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_Office365SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_endpoint` after provisioning.\nAPI Auth Endpoint."]
    pub fn auth_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `content_type` after provisioning.\nSupported office 365 content type.\nPossible values:\nAUDIT_AZURE_ACTIVE_DIRECTORY\nAUDIT_EXCHANGE\nAUDIT_SHARE_POINT\nAUDIT_GENERAL\nDLP_ALL"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nTenant ID."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureAdSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_OktaSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_OktaSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_OktaSettingsDynamic,
}

impl ChronicleFeed_Details_OktaSettings {
    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_OktaSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_OktaSettings {}

impl BuildChronicleFeed_Details_OktaSettings {
    pub fn build(self) -> ChronicleFeed_Details_OktaSettings {
        ChronicleFeed_Details_OktaSettings {
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_OktaSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_OktaSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_OktaSettingsRef {
        ChronicleFeed_Details_OktaSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_OktaSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_OktaUserContextSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_OktaUserContextSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manager_id_reference_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_OktaUserContextSettingsDynamic,
}

impl ChronicleFeed_Details_OktaUserContextSettings {
    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manager_id_reference_field`.\nManager id reference field."]
    pub fn set_manager_id_reference_field(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.manager_id_reference_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_OktaUserContextSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_OktaUserContextSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_OktaUserContextSettings {}

impl BuildChronicleFeed_Details_OktaUserContextSettings {
    pub fn build(self) -> ChronicleFeed_Details_OktaUserContextSettings {
        ChronicleFeed_Details_OktaUserContextSettings {
            hostname: core::default::Default::default(),
            manager_id_reference_field: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_OktaUserContextSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_OktaUserContextSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_OktaUserContextSettingsRef {
        ChronicleFeed_Details_OktaUserContextSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_OktaUserContextSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `manager_id_reference_field` after provisioning.\nManager id reference field."]
    pub fn manager_id_reference_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manager_id_reference_field", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_PanIocSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_PanIocSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feed: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feed_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_PanIocSettingsDynamic,
}

impl ChronicleFeed_Details_PanIocSettings {
    #[doc = "Set the field `feed`.\nPAN IOC feed name."]
    pub fn set_feed(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feed_id`.\nPAN IOC feed ID."]
    pub fn set_feed_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feed_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_PanIocSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_PanIocSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_PanIocSettings {}

impl BuildChronicleFeed_Details_PanIocSettings {
    pub fn build(self) -> ChronicleFeed_Details_PanIocSettings {
        ChronicleFeed_Details_PanIocSettings {
            feed: core::default::Default::default(),
            feed_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_PanIocSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_PanIocSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_PanIocSettingsRef {
        ChronicleFeed_Details_PanIocSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_PanIocSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `feed` after provisioning.\nPAN IOC feed name."]
    pub fn feed(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feed", self.base))
    }

    #[doc = "Get a reference to the value of field `feed_id` after provisioning.\nPAN IOC feed ID."]
    pub fn feed_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feed_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_PanPrismaCloudSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_PanPrismaCloudSettings_Authentication {
    #[doc = "Set the field `password`.\nPassword."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nUsername."]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_PanPrismaCloudSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_PanPrismaCloudSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_PanPrismaCloudSettings_Authentication {}

impl BuildChronicleFeed_Details_PanPrismaCloudSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_PanPrismaCloudSettings_Authentication {
        ChronicleFeed_Details_PanPrismaCloudSettings_Authentication {
            password: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_PanPrismaCloudSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_PanPrismaCloudSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_PanPrismaCloudSettings_AuthenticationRef {
        ChronicleFeed_Details_PanPrismaCloudSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_PanPrismaCloudSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nUsername."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_PanPrismaCloudSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_PanPrismaCloudSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_PanPrismaCloudSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_PanPrismaCloudSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_PanPrismaCloudSettingsDynamic,
}

impl ChronicleFeed_Details_PanPrismaCloudSettings {
    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_PanPrismaCloudSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_PanPrismaCloudSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_PanPrismaCloudSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_PanPrismaCloudSettings {}

impl BuildChronicleFeed_Details_PanPrismaCloudSettings {
    pub fn build(self) -> ChronicleFeed_Details_PanPrismaCloudSettings {
        ChronicleFeed_Details_PanPrismaCloudSettings {
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_PanPrismaCloudSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_PanPrismaCloudSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_PanPrismaCloudSettingsRef {
        ChronicleFeed_Details_PanPrismaCloudSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_PanPrismaCloudSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_PanPrismaCloudSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_ProofpointMailSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_ProofpointMailSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_ProofpointMailSettingsDynamic,
}

impl ChronicleFeed_Details_ProofpointMailSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_ProofpointMailSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_ProofpointMailSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_ProofpointMailSettings {}

impl BuildChronicleFeed_Details_ProofpointMailSettings {
    pub fn build(self) -> ChronicleFeed_Details_ProofpointMailSettings {
        ChronicleFeed_Details_ProofpointMailSettings {
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_ProofpointMailSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_ProofpointMailSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_ProofpointMailSettingsRef {
        ChronicleFeed_Details_ProofpointMailSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_ProofpointMailSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_ProofpointOnDemandSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_ProofpointOnDemandSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_ProofpointOnDemandSettingsDynamic,
}

impl ChronicleFeed_Details_ProofpointOnDemandSettings {
    #[doc = "Set the field `cluster_id`.\nCluster ID."]
    pub fn set_cluster_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_ProofpointOnDemandSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_ProofpointOnDemandSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_ProofpointOnDemandSettings {}

impl BuildChronicleFeed_Details_ProofpointOnDemandSettings {
    pub fn build(self) -> ChronicleFeed_Details_ProofpointOnDemandSettings {
        ChronicleFeed_Details_ProofpointOnDemandSettings {
            cluster_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_ProofpointOnDemandSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_ProofpointOnDemandSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_ProofpointOnDemandSettingsRef {
        ChronicleFeed_Details_ProofpointOnDemandSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_ProofpointOnDemandSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\nCluster ID."]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_PubsubSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_PubsubSettings {
    #[doc = "Set the field `google_service_account_email`.\nGoogle Service Account Email."]
    pub fn set_google_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.google_service_account_email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_PubsubSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_PubsubSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_PubsubSettings {}

impl BuildChronicleFeed_Details_PubsubSettings {
    pub fn build(self) -> ChronicleFeed_Details_PubsubSettings {
        ChronicleFeed_Details_PubsubSettings { google_service_account_email: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_PubsubSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_PubsubSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_PubsubSettingsRef {
        ChronicleFeed_Details_PubsubSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_PubsubSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `google_service_account_email` after provisioning.\nGoogle Service Account Email."]
    pub fn google_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_service_account_email", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_QualysScanSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_QualysScanSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_QualysScanSettingsDynamic,
}

impl ChronicleFeed_Details_QualysScanSettings {
    #[doc =
        "Set the field `api_type`.\nSupported Qualys Scan api type.\nPossible values:\nSCAN_SUMMARY_OUTPUT\nSCAN_COMPLIANCE_OUTPUT\nSCAN_COMPLIANCE_CONTROL_OUTPUT"]
    pub fn set_api_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nHostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_QualysScanSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_QualysScanSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_QualysScanSettings {}

impl BuildChronicleFeed_Details_QualysScanSettings {
    pub fn build(self) -> ChronicleFeed_Details_QualysScanSettings {
        ChronicleFeed_Details_QualysScanSettings {
            api_type: core::default::Default::default(),
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_QualysScanSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_QualysScanSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_QualysScanSettingsRef {
        ChronicleFeed_Details_QualysScanSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_QualysScanSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_type` after provisioning.\nSupported Qualys Scan api type.\nPossible values:\nSCAN_SUMMARY_OUTPUT\nSCAN_COMPLIANCE_OUTPUT\nSCAN_COMPLIANCE_CONTROL_OUTPUT"]
    pub fn api_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_type", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nHostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_QualysVmSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_QualysVmSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_QualysVmSettingsDynamic,
}

impl ChronicleFeed_Details_QualysVmSettings {
    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_QualysVmSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_QualysVmSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_QualysVmSettings {}

impl BuildChronicleFeed_Details_QualysVmSettings {
    pub fn build(self) -> ChronicleFeed_Details_QualysVmSettings {
        ChronicleFeed_Details_QualysVmSettings {
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_QualysVmSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_QualysVmSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_QualysVmSettingsRef {
        ChronicleFeed_Details_QualysVmSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_QualysVmSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_Rapid7InsightSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_Rapid7InsightSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_Rapid7InsightSettingsDynamic,
}

impl ChronicleFeed_Details_Rapid7InsightSettings {
    #[doc = "Set the field `endpoint`.\nRapid7 API endpoint. Should be \"vulnerabilities\" or \"assets\"."]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_Rapid7InsightSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_Rapid7InsightSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_Rapid7InsightSettings {}

impl BuildChronicleFeed_Details_Rapid7InsightSettings {
    pub fn build(self) -> ChronicleFeed_Details_Rapid7InsightSettings {
        ChronicleFeed_Details_Rapid7InsightSettings {
            endpoint: core::default::Default::default(),
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_Rapid7InsightSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_Rapid7InsightSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_Rapid7InsightSettingsRef {
        ChronicleFeed_Details_Rapid7InsightSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_Rapid7InsightSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nRapid7 API endpoint. Should be \"vulnerabilities\" or \"assets\"."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_RecordedFutureIocSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_RecordedFutureIocSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_RecordedFutureIocSettingsDynamic,
}

impl ChronicleFeed_Details_RecordedFutureIocSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_RecordedFutureIocSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_RecordedFutureIocSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_RecordedFutureIocSettings {}

impl BuildChronicleFeed_Details_RecordedFutureIocSettings {
    pub fn build(self) -> ChronicleFeed_Details_RecordedFutureIocSettings {
        ChronicleFeed_Details_RecordedFutureIocSettings {
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_RecordedFutureIocSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_RecordedFutureIocSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_RecordedFutureIocSettingsRef {
        ChronicleFeed_Details_RecordedFutureIocSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_RecordedFutureIocSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_RhIsacIocSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_RhIsacIocSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_RhIsacIocSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_RhIsacIocSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_RhIsacIocSettingsDynamic,
}

impl ChronicleFeed_Details_RhIsacIocSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_RhIsacIocSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_RhIsacIocSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_RhIsacIocSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_RhIsacIocSettings {}

impl BuildChronicleFeed_Details_RhIsacIocSettings {
    pub fn build(self) -> ChronicleFeed_Details_RhIsacIocSettings {
        ChronicleFeed_Details_RhIsacIocSettings {
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_RhIsacIocSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_RhIsacIocSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_RhIsacIocSettingsRef {
        ChronicleFeed_Details_RhIsacIocSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_RhIsacIocSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_RhIsacIocSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentialsDynamic {
    claims: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims>>,
    rs_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    claims: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rs_credentials: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials>>,
    dynamic: ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentialsDynamic,
}

impl ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentials {
    #[doc = "Set the field `token_endpoint`.\nToken endpoint to get the OAuth token from."]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `claims`.\n"]
    pub fn set_claims(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_Claims>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.claims = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.claims = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rs_credentials`.\n"]
    pub fn set_rs_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rs_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rs_credentials = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentials {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_SalesforceSettings_OauthJwtCredentials {}

impl BuildChronicleFeed_Details_SalesforceSettings_OauthJwtCredentials {
    pub fn build(self) -> ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentials {
        ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentials {
            token_endpoint: core::default::Default::default(),
            claims: core::default::Default::default(),
            rs_credentials: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentialsRef {
        ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nToken endpoint to get the OAuth token from."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `claims` after provisioning.\n"]
    pub fn claims(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_ClaimsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.claims", self.base))
    }

    #[doc = "Get a reference to the value of field `rs_credentials` after provisioning.\n"]
    pub fn rs_credentials(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication_RsCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rs_credentials", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuth {
    #[doc = "Set the field `client_id`.\nClient ID."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nClient secret."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password`.\nPassword."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\nToken endpoint to get the OAuth token from."]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\nUsername."]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuth {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuth {}

impl BuildChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuth {
    pub fn build(self) -> ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuth {
        ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuth {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            password: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuthRef {
        ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nClient ID."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nClient secret."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\nPassword."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nToken endpoint to get the OAuth token from."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\nUsername."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_SalesforceSettingsDynamic {
    oauth_jwt_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentials>>,
    oauth_password_grant_auth: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuth>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_SalesforceSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_jwt_credentials: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_password_grant_auth: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuth>>,
    dynamic: ChronicleFeed_Details_SalesforceSettingsDynamic,
}

impl ChronicleFeed_Details_SalesforceSettings {
    #[doc = "Set the field `hostname`.\nAPI hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_jwt_credentials`.\n"]
    pub fn set_oauth_jwt_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentials>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_jwt_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_jwt_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth_password_grant_auth`.\n"]
    pub fn set_oauth_password_grant_auth(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuth>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_password_grant_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_password_grant_auth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_SalesforceSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_SalesforceSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_SalesforceSettings {}

impl BuildChronicleFeed_Details_SalesforceSettings {
    pub fn build(self) -> ChronicleFeed_Details_SalesforceSettings {
        ChronicleFeed_Details_SalesforceSettings {
            hostname: core::default::Default::default(),
            oauth_jwt_credentials: core::default::Default::default(),
            oauth_password_grant_auth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_SalesforceSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_SalesforceSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_SalesforceSettingsRef {
        ChronicleFeed_Details_SalesforceSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_SalesforceSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_jwt_credentials` after provisioning.\n"]
    pub fn oauth_jwt_credentials(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_SalesforceSettings_OauthJwtCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_jwt_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_password_grant_auth` after provisioning.\n"]
    pub fn oauth_password_grant_auth(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_SalesforceSettings_OauthPasswordGrantAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_password_grant_auth", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_SentineloneAlertSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_SentineloneAlertSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_alert_api_subscribed: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_SentineloneAlertSettingsDynamic,
}

impl ChronicleFeed_Details_SentineloneAlertSettings {
    #[doc = "Set the field `hostname`.\nHostname of SentinelOne alert settings."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_start_time`.\ninitialStartTime from when to fetch the alerts"]
    pub fn set_initial_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.initial_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_alert_api_subscribed`.\nIs the customer subscribed to Alerts Api"]
    pub fn set_is_alert_api_subscribed(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_alert_api_subscribed = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_SentineloneAlertSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_SentineloneAlertSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_SentineloneAlertSettings {}

impl BuildChronicleFeed_Details_SentineloneAlertSettings {
    pub fn build(self) -> ChronicleFeed_Details_SentineloneAlertSettings {
        ChronicleFeed_Details_SentineloneAlertSettings {
            hostname: core::default::Default::default(),
            initial_start_time: core::default::Default::default(),
            is_alert_api_subscribed: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_SentineloneAlertSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_SentineloneAlertSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_SentineloneAlertSettingsRef {
        ChronicleFeed_Details_SentineloneAlertSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_SentineloneAlertSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hostname` after provisioning.\nHostname of SentinelOne alert settings."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_start_time` after provisioning.\ninitialStartTime from when to fetch the alerts"]
    pub fn initial_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_start_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_alert_api_subscribed` after provisioning.\nIs the customer subscribed to Alerts Api"]
    pub fn is_alert_api_subscribed(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_alert_api_subscribed", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_ServiceNowCmdbSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_ServiceNowCmdbSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feedname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_ServiceNowCmdbSettingsDynamic,
}

impl ChronicleFeed_Details_ServiceNowCmdbSettings {
    #[doc = "Set the field `feedname`.\nFeedname."]
    pub fn set_feedname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feedname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_ServiceNowCmdbSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_ServiceNowCmdbSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_ServiceNowCmdbSettings {}

impl BuildChronicleFeed_Details_ServiceNowCmdbSettings {
    pub fn build(self) -> ChronicleFeed_Details_ServiceNowCmdbSettings {
        ChronicleFeed_Details_ServiceNowCmdbSettings {
            feedname: core::default::Default::default(),
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_ServiceNowCmdbSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_ServiceNowCmdbSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_ServiceNowCmdbSettingsRef {
        ChronicleFeed_Details_ServiceNowCmdbSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_ServiceNowCmdbSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `feedname` after provisioning.\nFeedname."]
    pub fn feedname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feedname", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_SftpSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_passphrase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_SftpSettings_Authentication {
    #[doc = "Set the field `password`.\nPassword. Used for username and password authentication."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key`.\nPrivate key. Used for private key authentication."]
    pub fn set_private_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key_passphrase`.\nPrivate key passphrase. Used for private key authentication."]
    pub fn set_private_key_passphrase(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_key_passphrase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\nUsername. Used for username and password authentication."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_SftpSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_SftpSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_SftpSettings_Authentication {}

impl BuildChronicleFeed_Details_SftpSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_SftpSettings_Authentication {
        ChronicleFeed_Details_SftpSettings_Authentication {
            password: core::default::Default::default(),
            private_key: core::default::Default::default(),
            private_key_passphrase: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_SftpSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_SftpSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_SftpSettings_AuthenticationRef {
        ChronicleFeed_Details_SftpSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_SftpSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword. Used for username and password authentication."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key` after provisioning.\nPrivate key. Used for private key authentication."]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `private_key_passphrase` after provisioning.\nPrivate key passphrase. Used for private key authentication."]
    pub fn private_key_passphrase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_passphrase", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nUsername. Used for username and password authentication."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_SftpSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_SftpSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_SftpSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_deletion_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_SftpSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_SftpSettingsDynamic,
}

impl ChronicleFeed_Details_SftpSettings {
    #[doc =
        "Set the field `source_deletion_option`.\nPossible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub fn set_source_deletion_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_deletion_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_type`.\nPossible values:\nFILES\nFOLDERS\nFOLDERS_RECURSIVE"]
    pub fn set_source_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\nSFTP URI."]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_SftpSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_SftpSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_SftpSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_SftpSettings {}

impl BuildChronicleFeed_Details_SftpSettings {
    pub fn build(self) -> ChronicleFeed_Details_SftpSettings {
        ChronicleFeed_Details_SftpSettings {
            source_deletion_option: core::default::Default::default(),
            source_type: core::default::Default::default(),
            uri: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_SftpSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_SftpSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_SftpSettingsRef {
        ChronicleFeed_Details_SftpSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_SftpSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `source_deletion_option` after provisioning.\nPossible values:\nSOURCE_DELETION_NEVER\nSOURCE_DELETION_ON_SUCCESS\nSOURCE_DELETION_ON_SUCCESS_FILES_ONLY"]
    pub fn source_deletion_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_deletion_option", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source_type` after provisioning.\nPossible values:\nFILES\nFOLDERS\nFOLDERS_RECURSIVE"]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nSFTP URI."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_SftpSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_SymantecEventExportSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_SymantecEventExportSettings_Authentication {
    #[doc = "Set the field `client_id`.\nClient ID."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nClient secret."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token`.\nRefresh token."]
    pub fn set_refresh_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\nToken endpoint to get the OAuth token from."]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_SymantecEventExportSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_SymantecEventExportSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_SymantecEventExportSettings_Authentication {}

impl BuildChronicleFeed_Details_SymantecEventExportSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_SymantecEventExportSettings_Authentication {
        ChronicleFeed_Details_SymantecEventExportSettings_Authentication {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_SymantecEventExportSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_SymantecEventExportSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_SymantecEventExportSettings_AuthenticationRef {
        ChronicleFeed_Details_SymantecEventExportSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_SymantecEventExportSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nClient ID."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nClient secret."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\nRefresh token."]
    pub fn refresh_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nToken endpoint to get the OAuth token from."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_SymantecEventExportSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_SymantecEventExportSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_SymantecEventExportSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_SymantecEventExportSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_SymantecEventExportSettingsDynamic,
}

impl ChronicleFeed_Details_SymantecEventExportSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ChronicleFeed_Details_SymantecEventExportSettings_Authentication>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_SymantecEventExportSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_SymantecEventExportSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_SymantecEventExportSettings {}

impl BuildChronicleFeed_Details_SymantecEventExportSettings {
    pub fn build(self) -> ChronicleFeed_Details_SymantecEventExportSettings {
        ChronicleFeed_Details_SymantecEventExportSettings {
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_SymantecEventExportSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_SymantecEventExportSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_SymantecEventExportSettingsRef {
        ChronicleFeed_Details_SymantecEventExportSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_SymantecEventExportSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_SymantecEventExportSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_ThinkstCanarySettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_ThinkstCanarySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_ThinkstCanarySettingsDynamic,
}

impl ChronicleFeed_Details_ThinkstCanarySettings {
    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_ThinkstCanarySettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_ThinkstCanarySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_ThinkstCanarySettings {}

impl BuildChronicleFeed_Details_ThinkstCanarySettings {
    pub fn build(self) -> ChronicleFeed_Details_ThinkstCanarySettings {
        ChronicleFeed_Details_ThinkstCanarySettings {
            hostname: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_ThinkstCanarySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_ThinkstCanarySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_ThinkstCanarySettingsRef {
        ChronicleFeed_Details_ThinkstCanarySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_ThinkstCanarySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_ThreatConnectIocSettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_ThreatConnectIocSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owners: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_ThreatConnectIocSettingsDynamic,
}

impl ChronicleFeed_Details_ThreatConnectIocSettings {
    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owners`.\nOwners."]
    pub fn set_owners(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.owners = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_ThreatConnectIocSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_ThreatConnectIocSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_ThreatConnectIocSettings {}

impl BuildChronicleFeed_Details_ThreatConnectIocSettings {
    pub fn build(self) -> ChronicleFeed_Details_ThreatConnectIocSettings {
        ChronicleFeed_Details_ThreatConnectIocSettings {
            hostname: core::default::Default::default(),
            owners: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_ThreatConnectIocSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_ThreatConnectIocSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_ThreatConnectIocSettingsRef {
        ChronicleFeed_Details_ThreatConnectIocSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_ThreatConnectIocSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `owners` after provisioning.\nOwners."]
    pub fn owners(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.owners", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_ThreatConnectIocV3SettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_ThreatConnectIocV3Settings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owners: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tql_query: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_ThreatConnectIocV3SettingsDynamic,
}

impl ChronicleFeed_Details_ThreatConnectIocV3Settings {
    #[doc = "Set the field `fields`.\nFields"]
    pub fn set_fields(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\nhostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owners`.\nOwners."]
    pub fn set_owners(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.owners = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\nSchedule"]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tql_query`.\nThreatConnect Query Language filter."]
    pub fn set_tql_query(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tql_query = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_ThreatConnectIocV3Settings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_ThreatConnectIocV3Settings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_ThreatConnectIocV3Settings {}

impl BuildChronicleFeed_Details_ThreatConnectIocV3Settings {
    pub fn build(self) -> ChronicleFeed_Details_ThreatConnectIocV3Settings {
        ChronicleFeed_Details_ThreatConnectIocV3Settings {
            fields: core::default::Default::default(),
            hostname: core::default::Default::default(),
            owners: core::default::Default::default(),
            schedule: core::default::Default::default(),
            tql_query: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_ThreatConnectIocV3SettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_ThreatConnectIocV3SettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_ThreatConnectIocV3SettingsRef {
        ChronicleFeed_Details_ThreatConnectIocV3SettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_ThreatConnectIocV3SettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fields` after provisioning.\nFields"]
    pub fn fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fields", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nhostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `owners` after provisioning.\nOwners."]
    pub fn owners(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.owners", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\nSchedule"]
    pub fn schedule(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tql_query` after provisioning.\nThreatConnect Query Language filter."]
    pub fn tql_query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tql_query", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_Msso {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_Msso {
    #[doc =
        "Set the field `api_endpoint`.\nThe login api endpoint url.\nThis must be a valid URL with an http or https scheme. It has no default."]
    pub fn set_api_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nPassword of the account identified by username.\nThere are no restrictions on the format of the password. It has no default,\nspecifically enforced min / max length or character set. The password\nwill have been provided by an MSSO administrator and it is assumed that\nthey have provided a password that is internally consistent with MSSO\nauthentication requirements / validation."]
    pub fn set_password(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `username`.\nUsername for MSSO authentication.\nThere are no restrictions on the format of the username. It has no default,\nspecifically enforced min / max length or character set. The username\nwill have been provided by an MSSO administrator and it is assumed that\nthey have provided a username that is internally consistent with MSSO\nauthentication requirements / validation."]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_Msso {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_Msso>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_Msso {}

impl BuildChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_Msso {
    pub fn build(self) -> ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_Msso {
        ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_Msso {
            api_endpoint: core::default::Default::default(),
            password: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_MssoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_MssoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_MssoRef {
        ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_MssoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_MssoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_endpoint` after provisioning.\nThe login api endpoint url.\nThis must be a valid URL with an http or https scheme. It has no default."]
    pub fn api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword of the account identified by username.\nThere are no restrictions on the format of the password. It has no default,\nspecifically enforced min / max length or character set. The password\nwill have been provided by an MSSO administrator and it is assumed that\nthey have provided a password that is internally consistent with MSSO\nauthentication requirements / validation."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nUsername for MSSO authentication.\nThere are no restrictions on the format of the username. It has no default,\nspecifically enforced min / max length or character set. The username\nwill have been provided by an MSSO administrator and it is assumed that\nthey have provided a username that is internally consistent with MSSO\nauthentication requirements / validation."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIam {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIam {
    #[doc =
        "Set the field `client_id`.\nClient ID generated in Trellix IAM.\nThis is a unique identifier for the user that is generated in Trellix IAM.\nIt has no default, specifically enforced min / max length or character set.\nIt is assumed that the Client ID generated in Trellix IAM is internally\nconsistent with Trellix IAM authentication requirements / validation."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `client_secret`.\nSecret associated with the Client ID.\nThis is the secret generated in Trellix IAM for the Client ID. It has no\ndefault, specifically enforced min / max length or character set. It is\nassumed that the secret generated in Trellix IAM is internally\nconsistent with Trellix IAM authentication requirements / validation."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `scope`.\nOAUTH 2 scope to request for the authentication token.\nThis is the OAUTH 2 scope to request for the authentication token. It has\nno default, specifically enforced min / max length or character set. It is\nassumed that the scope provided is internally consistent with Trellix IAM\nauthentication requirements / validation."]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIam {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIam>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIam {}

impl BuildChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIam {
    pub fn build(self) -> ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIam {
        ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIam {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIamRef {
        ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nClient ID generated in Trellix IAM.\nThis is a unique identifier for the user that is generated in Trellix IAM.\nIt has no default, specifically enforced min / max length or character set.\nIt is assumed that the Client ID generated in Trellix IAM is internally\nconsistent with Trellix IAM authentication requirements / validation."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nSecret associated with the Client ID.\nThis is the secret generated in Trellix IAM for the Client ID. It has no\ndefault, specifically enforced min / max length or character set. It is\nassumed that the secret generated in Trellix IAM is internally\nconsistent with Trellix IAM authentication requirements / validation."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nOAUTH 2 scope to request for the authentication token.\nThis is the OAUTH 2 scope to request for the authentication token. It has\nno default, specifically enforced min / max length or character set. It is\nassumed that the scope provided is internally consistent with Trellix IAM\nauthentication requirements / validation."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_TrellixHxAlertsSettings_AuthenticationDynamic {
    msso: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_Msso>>,
    trellix_iam: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIam>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    msso: std::option::Option<std::vec::Vec<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_Msso>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trellix_iam: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIam>>,
    dynamic: ChronicleFeed_Details_TrellixHxAlertsSettings_AuthenticationDynamic,
}

impl ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication {
    #[doc = "Set the field `msso`.\n"]
    pub fn set_msso(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_Msso>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.msso = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.msso = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trellix_iam`.\n"]
    pub fn set_trellix_iam(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIam>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trellix_iam = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trellix_iam = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_TrellixHxAlertsSettings_Authentication {}

impl BuildChronicleFeed_Details_TrellixHxAlertsSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication {
        ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication {
            msso: core::default::Default::default(),
            trellix_iam: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_TrellixHxAlertsSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_TrellixHxAlertsSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_TrellixHxAlertsSettings_AuthenticationRef {
        ChronicleFeed_Details_TrellixHxAlertsSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_TrellixHxAlertsSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `msso` after provisioning.\n"]
    pub fn msso(&self) -> terrars::ListRef<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_MssoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.msso", self.base))
    }

    #[doc = "Get a reference to the value of field `trellix_iam` after provisioning.\n"]
    pub fn trellix_iam(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication_TrellixIamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trellix_iam", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_TrellixHxAlertsSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_TrellixHxAlertsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_TrellixHxAlertsSettingsDynamic,
}

impl ChronicleFeed_Details_TrellixHxAlertsSettings {
    #[doc =
        "Set the field `endpoint`.\nTrellix HX Device URL.\nThis must be a valid URL with an http or https scheme. It has no default.\nUsually a device URL is in the form of either:\nhttps://xxx.trellix.com/hx/id//\n- or -\nhttps://htapdeviceproxy.md.mandiant.net/dphb/hx//"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxAlertsSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_TrellixHxAlertsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxAlertsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_TrellixHxAlertsSettings {}

impl BuildChronicleFeed_Details_TrellixHxAlertsSettings {
    pub fn build(self) -> ChronicleFeed_Details_TrellixHxAlertsSettings {
        ChronicleFeed_Details_TrellixHxAlertsSettings {
            endpoint: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_TrellixHxAlertsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_TrellixHxAlertsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_TrellixHxAlertsSettingsRef {
        ChronicleFeed_Details_TrellixHxAlertsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_TrellixHxAlertsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nTrellix HX Device URL.\nThis must be a valid URL with an http or https scheme. It has no default.\nUsually a device URL is in the form of either:\nhttps://xxx.trellix.com/hx/id//\n- or -\nhttps://htapdeviceproxy.md.mandiant.net/dphb/hx//"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_TrellixHxAlertsSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_Msso {
    api_endpoint: terrars::PrimField<std::string::String>,
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_Msso { }

impl terrars::ToListMappable for ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_Msso {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_Msso>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_TrellixHxHostsSettings_Authentication_Msso {
    #[doc = "The login api endpoint url.\nThis must be a valid URL with an http or https scheme. It has no default."]
    pub api_endpoint: terrars::PrimField<std::string::String>,
    #[doc =
        "Password of the account identified by username.\nThere are no restrictions on the format of the password. It has no default,\nspecifically enforced min / max length or character set. The password\nwill have been provided by an MSSO administrator and it is assumed that\nthey have provided a password that is internally consistent with MSSO\nauthentication requirements / validation."]
    pub password: terrars::PrimField<std::string::String>,
    #[doc =
        "Username for MSSO authentication.\nThere are no restrictions on the format of the username. It has no default,\nspecifically enforced min / max length or character set. The username\nwill have been provided by an MSSO administrator and it is assumed that\nthey have provided a username that is internally consistent with MSSO\nauthentication requirements / validation."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_TrellixHxHostsSettings_Authentication_Msso {
    pub fn build(self) -> ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_Msso {
        ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_Msso {
            api_endpoint: self.api_endpoint,
            password: self.password,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_MssoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_MssoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_MssoRef {
        ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_MssoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_MssoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_endpoint` after provisioning.\nThe login api endpoint url.\nThis must be a valid URL with an http or https scheme. It has no default."]
    pub fn api_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nPassword of the account identified by username.\nThere are no restrictions on the format of the password. It has no default,\nspecifically enforced min / max length or character set. The password\nwill have been provided by an MSSO administrator and it is assumed that\nthey have provided a password that is internally consistent with MSSO\nauthentication requirements / validation."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc =
        "Get a reference to the value of field `username` after provisioning.\nUsername for MSSO authentication.\nThere are no restrictions on the format of the username. It has no default,\nspecifically enforced min / max length or character set. The username\nwill have been provided by an MSSO administrator and it is assumed that\nthey have provided a username that is internally consistent with MSSO\nauthentication requirements / validation."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIam {
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
    scope: terrars::PrimField<std::string::String>,
}

impl ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIam { }

impl terrars::ToListMappable for ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIam {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIam>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIam {
    #[doc =
        "Client ID generated in Trellix IAM.\nThis is a unique identifier for the user that is generated in Trellix IAM.\nIt has no default, specifically enforced min / max length or character set.\nIt is assumed that the Client ID generated in Trellix IAM is internally\nconsistent with Trellix IAM authentication requirements / validation."]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Secret associated with the Client ID.\nThis is the secret generated in Trellix IAM for the Client ID. It has no\ndefault, specifically enforced min / max length or character set. It is\nassumed that the secret generated in Trellix IAM is internally\nconsistent with Trellix IAM authentication requirements / validation."]
    pub client_secret: terrars::PrimField<std::string::String>,
    #[doc =
        "OAUTH 2 scope to request for the authentication token.\nThis is the OAUTH 2 scope to request for the authentication token. It has\nno default, specifically enforced min / max length or character set. It is\nassumed that the scope provided is internally consistent with Trellix IAM\nauthentication requirements / validation."]
    pub scope: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIam {
    pub fn build(self) -> ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIam {
        ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIam {
            client_id: self.client_id,
            client_secret: self.client_secret,
            scope: self.scope,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIamRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIamRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIamRef {
        ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIamRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIamRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `client_id` after provisioning.\nClient ID generated in Trellix IAM.\nThis is a unique identifier for the user that is generated in Trellix IAM.\nIt has no default, specifically enforced min / max length or character set.\nIt is assumed that the Client ID generated in Trellix IAM is internally\nconsistent with Trellix IAM authentication requirements / validation."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `client_secret` after provisioning.\nSecret associated with the Client ID.\nThis is the secret generated in Trellix IAM for the Client ID. It has no\ndefault, specifically enforced min / max length or character set. It is\nassumed that the secret generated in Trellix IAM is internally\nconsistent with Trellix IAM authentication requirements / validation."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scope` after provisioning.\nOAUTH 2 scope to request for the authentication token.\nThis is the OAUTH 2 scope to request for the authentication token. It has\nno default, specifically enforced min / max length or character set. It is\nassumed that the scope provided is internally consistent with Trellix IAM\nauthentication requirements / validation."]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_TrellixHxHostsSettings_AuthenticationDynamic {
    msso: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_Msso>>,
    trellix_iam: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIam>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_TrellixHxHostsSettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    msso: std::option::Option<std::vec::Vec<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_Msso>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trellix_iam: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIam>>,
    dynamic: ChronicleFeed_Details_TrellixHxHostsSettings_AuthenticationDynamic,
}

impl ChronicleFeed_Details_TrellixHxHostsSettings_Authentication {
    #[doc = "Set the field `msso`.\n"]
    pub fn set_msso(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_Msso>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.msso = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.msso = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trellix_iam`.\n"]
    pub fn set_trellix_iam(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIam>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trellix_iam = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trellix_iam = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_TrellixHxHostsSettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_TrellixHxHostsSettings_Authentication {}

impl BuildChronicleFeed_Details_TrellixHxHostsSettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_TrellixHxHostsSettings_Authentication {
        ChronicleFeed_Details_TrellixHxHostsSettings_Authentication {
            msso: core::default::Default::default(),
            trellix_iam: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_TrellixHxHostsSettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_TrellixHxHostsSettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_TrellixHxHostsSettings_AuthenticationRef {
        ChronicleFeed_Details_TrellixHxHostsSettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_TrellixHxHostsSettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `msso` after provisioning.\n"]
    pub fn msso(&self) -> terrars::ListRef<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_MssoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.msso", self.base))
    }

    #[doc = "Get a reference to the value of field `trellix_iam` after provisioning.\n"]
    pub fn trellix_iam(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication_TrellixIamRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trellix_iam", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_TrellixHxBulkAcqsSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_TrellixHxBulkAcqsSettings {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_TrellixHxBulkAcqsSettingsDynamic,
}

impl ChronicleFeed_Details_TrellixHxBulkAcqsSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_TrellixHxBulkAcqsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxBulkAcqsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_TrellixHxBulkAcqsSettings {
    #[doc =
        "Trellix HX Device URL.\nThis must be a valid URL with an http or https scheme. It has no default.\nUsually a device URL is in the form of either:\nhttps://xxx.trellix.com/hx/id//\n- or -\nhttps://htapdeviceproxy.md.mandiant.net/dphb/hx//"]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_TrellixHxBulkAcqsSettings {
    pub fn build(self) -> ChronicleFeed_Details_TrellixHxBulkAcqsSettings {
        ChronicleFeed_Details_TrellixHxBulkAcqsSettings {
            endpoint: self.endpoint,
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_TrellixHxBulkAcqsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_TrellixHxBulkAcqsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_TrellixHxBulkAcqsSettingsRef {
        ChronicleFeed_Details_TrellixHxBulkAcqsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_TrellixHxBulkAcqsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nTrellix HX Device URL.\nThis must be a valid URL with an http or https scheme. It has no default.\nUsually a device URL is in the form of either:\nhttps://xxx.trellix.com/hx/id//\n- or -\nhttps://htapdeviceproxy.md.mandiant.net/dphb/hx//"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_TrellixHxHostsSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_TrellixHxHostsSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_TrellixHxHostsSettings {
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_TrellixHxHostsSettingsDynamic,
}

impl ChronicleFeed_Details_TrellixHxHostsSettings {
    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxHostsSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_TrellixHxHostsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxHostsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_TrellixHxHostsSettings {
    #[doc =
        "Trellix HX Device URL.\nThis must be a valid URL with an http or https scheme. It has no default.\nUsually a device URL is in the form of either:\nhttps://xxx.trellix.com/hx/id//\n- or -\nhttps://htapdeviceproxy.md.mandiant.net/dphb/hx//"]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details_TrellixHxHostsSettings {
    pub fn build(self) -> ChronicleFeed_Details_TrellixHxHostsSettings {
        ChronicleFeed_Details_TrellixHxHostsSettings {
            endpoint: self.endpoint,
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_TrellixHxHostsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_TrellixHxHostsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_TrellixHxHostsSettingsRef {
        ChronicleFeed_Details_TrellixHxHostsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_TrellixHxHostsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nTrellix HX Device URL.\nThis must be a valid URL with an http or https scheme. It has no default.\nUsually a device URL is in the form of either:\nhttps://xxx.trellix.com/hx/id//\n- or -\nhttps://htapdeviceproxy.md.mandiant.net/dphb/hx//"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_TrellixHxHostsSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WebhookSettings {}

impl ChronicleFeed_Details_WebhookSettings { }

impl terrars::ToListMappable for ChronicleFeed_Details_WebhookSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WebhookSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WebhookSettings {}

impl BuildChronicleFeed_Details_WebhookSettings {
    pub fn build(self) -> ChronicleFeed_Details_WebhookSettings {
        ChronicleFeed_Details_WebhookSettings {}
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WebhookSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WebhookSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_WebhookSettingsRef {
        ChronicleFeed_Details_WebhookSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WebhookSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkdaySettings_Authentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Details_WorkdaySettings_Authentication {
    #[doc = "Set the field `client_id`.\nClient ID."]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret`.\nClient Secret."]
    pub fn set_client_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token`.\nRefresh Token."]
    pub fn set_refresh_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_token = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret`.\nThe access token used to authenticate against Workday. This field is called\n\"secret\" to maintain backwards compatibility. Workday was (only) configured\nusing username (which was unused) and secret (which is used as the access\ntoken). Either this field or all of the other OAuth fields below must be\nspecified."]
    pub fn set_secret(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\nToken endpoint to get the OAuth token from."]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user`.\nUsername. This is unused: Workday feeds were originally configured using a\nusername and secret authentication method, but only the secret field was\nused, and it was used to supply the OAuth access token."]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkdaySettings_Authentication {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkdaySettings_Authentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkdaySettings_Authentication {}

impl BuildChronicleFeed_Details_WorkdaySettings_Authentication {
    pub fn build(self) -> ChronicleFeed_Details_WorkdaySettings_Authentication {
        ChronicleFeed_Details_WorkdaySettings_Authentication {
            client_id: core::default::Default::default(),
            client_secret: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
            secret: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkdaySettings_AuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkdaySettings_AuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_WorkdaySettings_AuthenticationRef {
        ChronicleFeed_Details_WorkdaySettings_AuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkdaySettings_AuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nClient ID."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\nClient Secret."]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\nRefresh Token."]
    pub fn refresh_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nThe access token used to authenticate against Workday. This field is called\n\"secret\" to maintain backwards compatibility. Workday was (only) configured\nusing username (which was unused) and secret (which is used as the access\ntoken). Either this field or all of the other OAuth fields below must be\nspecified."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `token_endpoint` after provisioning.\nToken endpoint to get the OAuth token from."]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nUsername. This is unused: Workday feeds were originally configured using a\nusername and secret authentication method, but only the secret field was\nused, and it was used to supply the OAuth access token."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_WorkdaySettingsDynamic {
    authentication: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkdaySettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkdaySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenant_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkdaySettings_Authentication>>,
    dynamic: ChronicleFeed_Details_WorkdaySettingsDynamic,
}

impl ChronicleFeed_Details_WorkdaySettings {
    #[doc = "Set the field `hostname`.\nAPI Hostname."]
    pub fn set_hostname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenant_id`.\nTenant ID."]
    pub fn set_tenant_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenant_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkdaySettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkdaySettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkdaySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkdaySettings {}

impl BuildChronicleFeed_Details_WorkdaySettings {
    pub fn build(self) -> ChronicleFeed_Details_WorkdaySettings {
        ChronicleFeed_Details_WorkdaySettings {
            hostname: core::default::Default::default(),
            tenant_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkdaySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkdaySettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_Details_WorkdaySettingsRef {
        ChronicleFeed_Details_WorkdaySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkdaySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\nAPI Hostname."]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc = "Get a reference to the value of field `tenant_id` after provisioning.\nTenant ID."]
    pub fn tenant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenant_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkdaySettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_WorkspaceActivitySettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkspaceActivitySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    applications: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_customer_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_WorkspaceActivitySettingsDynamic,
}

impl ChronicleFeed_Details_WorkspaceActivitySettings {
    #[doc = "Set the field `applications`.\nApplications."]
    pub fn set_applications(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.applications = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workspace_customer_id`.\nCustomer ID."]
    pub fn set_workspace_customer_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workspace_customer_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkspaceActivitySettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceActivitySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkspaceActivitySettings {}

impl BuildChronicleFeed_Details_WorkspaceActivitySettings {
    pub fn build(self) -> ChronicleFeed_Details_WorkspaceActivitySettings {
        ChronicleFeed_Details_WorkspaceActivitySettings {
            applications: core::default::Default::default(),
            workspace_customer_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkspaceActivitySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkspaceActivitySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_WorkspaceActivitySettingsRef {
        ChronicleFeed_Details_WorkspaceActivitySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkspaceActivitySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `applications` after provisioning.\nApplications."]
    pub fn applications(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.applications", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_customer_id` after provisioning.\nCustomer ID."]
    pub fn workspace_customer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_customer_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_WorkspaceAlertsSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkspaceAlertsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_customer_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_WorkspaceAlertsSettingsDynamic,
}

impl ChronicleFeed_Details_WorkspaceAlertsSettings {
    #[doc = "Set the field `workspace_customer_id`.\nCustomer ID."]
    pub fn set_workspace_customer_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workspace_customer_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkspaceAlertsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceAlertsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkspaceAlertsSettings {}

impl BuildChronicleFeed_Details_WorkspaceAlertsSettings {
    pub fn build(self) -> ChronicleFeed_Details_WorkspaceAlertsSettings {
        ChronicleFeed_Details_WorkspaceAlertsSettings {
            workspace_customer_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkspaceAlertsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkspaceAlertsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_WorkspaceAlertsSettingsRef {
        ChronicleFeed_Details_WorkspaceAlertsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkspaceAlertsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workspace_customer_id` after provisioning.\nCustomer ID."]
    pub fn workspace_customer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_customer_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_WorkspaceChromeOsSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkspaceChromeOsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_customer_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_WorkspaceChromeOsSettingsDynamic,
}

impl ChronicleFeed_Details_WorkspaceChromeOsSettings {
    #[doc = "Set the field `workspace_customer_id`.\nCustomer ID."]
    pub fn set_workspace_customer_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workspace_customer_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkspaceChromeOsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceChromeOsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkspaceChromeOsSettings {}

impl BuildChronicleFeed_Details_WorkspaceChromeOsSettings {
    pub fn build(self) -> ChronicleFeed_Details_WorkspaceChromeOsSettings {
        ChronicleFeed_Details_WorkspaceChromeOsSettings {
            workspace_customer_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkspaceChromeOsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkspaceChromeOsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_WorkspaceChromeOsSettingsRef {
        ChronicleFeed_Details_WorkspaceChromeOsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkspaceChromeOsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workspace_customer_id` after provisioning.\nCustomer ID."]
    pub fn workspace_customer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_customer_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_WorkspaceGroupsSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkspaceGroupsSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_customer_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_WorkspaceGroupsSettingsDynamic,
}

impl ChronicleFeed_Details_WorkspaceGroupsSettings {
    #[doc = "Set the field `workspace_customer_id`.\nCustomer ID."]
    pub fn set_workspace_customer_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workspace_customer_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkspaceGroupsSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceGroupsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkspaceGroupsSettings {}

impl BuildChronicleFeed_Details_WorkspaceGroupsSettings {
    pub fn build(self) -> ChronicleFeed_Details_WorkspaceGroupsSettings {
        ChronicleFeed_Details_WorkspaceGroupsSettings {
            workspace_customer_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkspaceGroupsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkspaceGroupsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_WorkspaceGroupsSettingsRef {
        ChronicleFeed_Details_WorkspaceGroupsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkspaceGroupsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workspace_customer_id` after provisioning.\nCustomer ID."]
    pub fn workspace_customer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_customer_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_WorkspaceMobileSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkspaceMobileSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_customer_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_WorkspaceMobileSettingsDynamic,
}

impl ChronicleFeed_Details_WorkspaceMobileSettings {
    #[doc = "Set the field `workspace_customer_id`.\nCustomer ID."]
    pub fn set_workspace_customer_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workspace_customer_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkspaceMobileSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceMobileSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkspaceMobileSettings {}

impl BuildChronicleFeed_Details_WorkspaceMobileSettings {
    pub fn build(self) -> ChronicleFeed_Details_WorkspaceMobileSettings {
        ChronicleFeed_Details_WorkspaceMobileSettings {
            workspace_customer_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkspaceMobileSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkspaceMobileSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_WorkspaceMobileSettingsRef {
        ChronicleFeed_Details_WorkspaceMobileSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkspaceMobileSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workspace_customer_id` after provisioning.\nCustomer ID."]
    pub fn workspace_customer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_customer_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_WorkspacePrivilegesSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkspacePrivilegesSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_customer_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_WorkspacePrivilegesSettingsDynamic,
}

impl ChronicleFeed_Details_WorkspacePrivilegesSettings {
    #[doc = "Set the field `workspace_customer_id`.\nCustomer ID."]
    pub fn set_workspace_customer_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workspace_customer_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkspacePrivilegesSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkspacePrivilegesSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkspacePrivilegesSettings {}

impl BuildChronicleFeed_Details_WorkspacePrivilegesSettings {
    pub fn build(self) -> ChronicleFeed_Details_WorkspacePrivilegesSettings {
        ChronicleFeed_Details_WorkspacePrivilegesSettings {
            workspace_customer_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkspacePrivilegesSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkspacePrivilegesSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_WorkspacePrivilegesSettingsRef {
        ChronicleFeed_Details_WorkspacePrivilegesSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkspacePrivilegesSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workspace_customer_id` after provisioning.\nCustomer ID."]
    pub fn workspace_customer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_customer_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_Details_WorkspaceUsersSettingsDynamic {
    authentication: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details_WorkspaceUsersSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    projection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_customer_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    dynamic: ChronicleFeed_Details_WorkspaceUsersSettingsDynamic,
}

impl ChronicleFeed_Details_WorkspaceUsersSettings {
    #[doc = "Set the field `projection_type`.\nProjection Type.\nPossible values:\nBASIC_PROJECTION\nFULL_PROJECTION"]
    pub fn set_projection_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.projection_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workspace_customer_id`.\nCustomer ID."]
    pub fn set_workspace_customer_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.workspace_customer_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings_Authentication>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authentication = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details_WorkspaceUsersSettings {
    type O = terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details_WorkspaceUsersSettings {}

impl BuildChronicleFeed_Details_WorkspaceUsersSettings {
    pub fn build(self) -> ChronicleFeed_Details_WorkspaceUsersSettings {
        ChronicleFeed_Details_WorkspaceUsersSettings {
            projection_type: core::default::Default::default(),
            workspace_customer_id: core::default::Default::default(),
            authentication: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_Details_WorkspaceUsersSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_Details_WorkspaceUsersSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleFeed_Details_WorkspaceUsersSettingsRef {
        ChronicleFeed_Details_WorkspaceUsersSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_Details_WorkspaceUsersSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `projection_type` after provisioning.\nProjection Type.\nPossible values:\nBASIC_PROJECTION\nFULL_PROJECTION"]
    pub fn projection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.projection_type", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_customer_id` after provisioning.\nCustomer ID."]
    pub fn workspace_customer_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workspace_customer_id", self.base))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettings_AuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleFeed_DetailsDynamic {
    amazon_kinesis_firehose_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonKinesisFirehoseSettings>>,
    amazon_s3_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonS3Settings>>,
    amazon_s3_v2_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonS3V2Settings>>,
    amazon_sqs_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonSqsSettings>>,
    amazon_sqs_v2_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AmazonSqsV2Settings>>,
    anomali_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AnomaliSettings>>,
    aws_ec2_hosts_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsEc2HostsSettings>>,
    aws_ec2_instances_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsEc2InstancesSettings>>,
    aws_ec2_vpcs_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsEc2VpcsSettings>>,
    aws_iam_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AwsIamSettings>>,
    azure_ad_audit_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureAdAuditSettings>>,
    azure_ad_context_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureAdContextSettings>>,
    azure_ad_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureAdSettings>>,
    azure_blob_store_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureBlobStoreSettings>>,
    azure_blob_store_v2_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureBlobStoreV2Settings>>,
    azure_event_hub_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureEventHubSettings>>,
    azure_mdm_intune_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_AzureMdmIntuneSettings>>,
    cloud_passage_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_CloudPassageSettings>>,
    cortex_xdr_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_CortexXdrSettings>>,
    crowdstrike_alerts_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_CrowdstrikeAlertsSettings>>,
    crowdstrike_detects_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_CrowdstrikeDetectsSettings>>,
    dummy_log_type_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_DummyLogTypeSettings>>,
    duo_auth_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_DuoAuthSettings>>,
    duo_user_context_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_DuoUserContextSettings>>,
    fox_it_stix_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_FoxItStixSettings>>,
    gcs_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_GcsSettings>>,
    gcs_v2_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_GcsV2Settings>>,
    google_cloud_identity_device_users_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettings>>,
    google_cloud_identity_devices_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_GoogleCloudIdentityDevicesSettings>>,
    google_cloud_storage_event_driven_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettings>>,
    http_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_HttpSettings>>,
    https_push_amazon_kinesis_firehose_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettings>>,
    https_push_google_cloud_pubsub_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettings>>,
    https_push_webhook_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_HttpsPushWebhookSettings>>,
    imperva_waf_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_ImpervaWafSettings>>,
    mandiant_ioc_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_MandiantIocSettings>>,
    microsoft_graph_alert_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_MicrosoftGraphAlertSettings>>,
    microsoft_security_center_alert_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettings>>,
    mimecast_mail_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_MimecastMailSettings>>,
    mimecast_mail_v2_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_MimecastMailV2Settings>>,
    netskope_alert_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_NetskopeAlertSettings>>,
    netskope_alert_v2_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_NetskopeAlertV2Settings>>,
    office365_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_Office365Settings>>,
    okta_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaSettings>>,
    okta_user_context_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_OktaUserContextSettings>>,
    pan_ioc_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_PanIocSettings>>,
    pan_prisma_cloud_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_PanPrismaCloudSettings>>,
    proofpoint_mail_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_ProofpointMailSettings>>,
    proofpoint_on_demand_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_ProofpointOnDemandSettings>>,
    pubsub_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_PubsubSettings>>,
    qualys_scan_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_QualysScanSettings>>,
    qualys_vm_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_QualysVmSettings>>,
    rapid7_insight_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_Rapid7InsightSettings>>,
    recorded_future_ioc_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_RecordedFutureIocSettings>>,
    rh_isac_ioc_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_RhIsacIocSettings>>,
    salesforce_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_SalesforceSettings>>,
    sentinelone_alert_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_SentineloneAlertSettings>>,
    service_now_cmdb_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_ServiceNowCmdbSettings>>,
    sftp_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_SftpSettings>>,
    symantec_event_export_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_SymantecEventExportSettings>>,
    thinkst_canary_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_ThinkstCanarySettings>>,
    threat_connect_ioc_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_ThreatConnectIocSettings>>,
    threat_connect_ioc_v3_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_ThreatConnectIocV3Settings>>,
    trellix_hx_alerts_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_TrellixHxAlertsSettings>>,
    trellix_hx_bulk_acqs_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_TrellixHxBulkAcqsSettings>>,
    trellix_hx_hosts_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_TrellixHxHostsSettings>>,
    webhook_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_WebhookSettings>>,
    workday_settings: std::option::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkdaySettings>>,
    workspace_activity_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceActivitySettings>>,
    workspace_alerts_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceAlertsSettings>>,
    workspace_chrome_os_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceChromeOsSettings>>,
    workspace_groups_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceGroupsSettings>>,
    workspace_mobile_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceMobileSettings>>,
    workspace_privileges_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspacePrivilegesSettings>>,
    workspace_users_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleFeed_Details_WorkspaceUsersSettings>>,
}

#[derive(Serialize)]
pub struct ChronicleFeed_Details {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset_namespace: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feed_source_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    log_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_kinesis_firehose_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_AmazonKinesisFirehoseSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_s3_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AmazonS3Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_s3_v2_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AmazonS3V2Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_sqs_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AmazonSqsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_sqs_v2_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AmazonSqsV2Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    anomali_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AnomaliSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_ec2_hosts_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsEc2HostsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_ec2_instances_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsEc2InstancesSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_ec2_vpcs_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsEc2VpcsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_iam_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AwsIamSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_ad_audit_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureAdAuditSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_ad_context_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureAdContextSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_ad_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureAdSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_blob_store_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureBlobStoreSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_blob_store_v2_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureBlobStoreV2Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_event_hub_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureEventHubSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    azure_mdm_intune_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_AzureMdmIntuneSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_passage_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_CloudPassageSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cortex_xdr_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_CortexXdrSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crowdstrike_alerts_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_CrowdstrikeAlertsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crowdstrike_detects_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_CrowdstrikeDetectsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dummy_log_type_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_DummyLogTypeSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duo_auth_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_DuoAuthSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duo_user_context_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_DuoUserContextSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fox_it_stix_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_FoxItStixSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_GcsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_v2_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_GcsV2Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_identity_device_users_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_identity_devices_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_GoogleCloudIdentityDevicesSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_storage_event_driven_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_HttpSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_push_amazon_kinesis_firehose_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_push_google_cloud_pubsub_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_push_webhook_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_HttpsPushWebhookSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    imperva_waf_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_ImpervaWafSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mandiant_ioc_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_MandiantIocSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    microsoft_graph_alert_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_MicrosoftGraphAlertSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    microsoft_security_center_alert_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mimecast_mail_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_MimecastMailSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mimecast_mail_v2_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_MimecastMailV2Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    netskope_alert_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_NetskopeAlertSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    netskope_alert_v2_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_NetskopeAlertV2Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    office365_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_Office365Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    okta_user_context_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_OktaUserContextSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pan_ioc_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_PanIocSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pan_prisma_cloud_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_PanPrismaCloudSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proofpoint_mail_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_ProofpointMailSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proofpoint_on_demand_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_ProofpointOnDemandSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_PubsubSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualys_scan_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_QualysScanSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    qualys_vm_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_QualysVmSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rapid7_insight_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_Rapid7InsightSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recorded_future_ioc_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_RecordedFutureIocSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rh_isac_ioc_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_RhIsacIocSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    salesforce_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_SalesforceSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sentinelone_alert_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_SentineloneAlertSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now_cmdb_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_ServiceNowCmdbSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sftp_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_SftpSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    symantec_event_export_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_SymantecEventExportSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thinkst_canary_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_ThinkstCanarySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_connect_ioc_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_ThreatConnectIocSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_connect_ioc_v3_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_ThreatConnectIocV3Settings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trellix_hx_alerts_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_TrellixHxAlertsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trellix_hx_bulk_acqs_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_TrellixHxBulkAcqsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trellix_hx_hosts_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_TrellixHxHostsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    webhook_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WebhookSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workday_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkdaySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_activity_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceActivitySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_alerts_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceAlertsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_chrome_os_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceChromeOsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_groups_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceGroupsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_mobile_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceMobileSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_privileges_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspacePrivilegesSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workspace_users_settings: std::option::Option<std::vec::Vec<ChronicleFeed_Details_WorkspaceUsersSettings>>,
    dynamic: ChronicleFeed_DetailsDynamic,
}

impl ChronicleFeed_Details {
    #[doc = "Set the field `asset_namespace`.\nThe asset namespace to apply to all logs ingested through this feed."]
    pub fn set_asset_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.asset_namespace = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `feed_source_type`.\nSource Type of the feed.\nPossible values:\nGOOGLE_CLOUD_STORAGE\nHTTP\nSFTP\nAMAZON_S3\nAZURE_BLOBSTORE\nAPI\nAMAZON_SQS\nPUBSUB\nAMAZON_KINESIS_FIREHOSE\nWEBHOOK\nHTTPS_PUSH_GOOGLE_CLOUD_PUBSUB\nHTTPS_PUSH_AMAZON_KINESIS_FIREHOSE\nHTTPS_PUSH_WEBHOOK\nAZURE_EVENT_HUB\nGOOGLE_CLOUD_STORAGE_V2\nAMAZON_S3_V2\nAMAZON_SQS_V2\nAZURE_BLOBSTORE_V2\nGOOGLE_CLOUD_STORAGE_EVENT_DRIVEN Possible values: [\"GOOGLE_CLOUD_STORAGE\", \"HTTP\", \"SFTP\", \"AMAZON_S3\", \"AZURE_BLOBSTORE\", \"API\", \"AMAZON_SQS\", \"PUBSUB\", \"AMAZON_KINESIS_FIREHOSE\", \"WEBHOOK\", \"HTTPS_PUSH_GOOGLE_CLOUD_PUBSUB\", \"HTTPS_PUSH_AMAZON_KINESIS_FIREHOSE\", \"HTTPS_PUSH_WEBHOOK\", \"AZURE_EVENT_HUB\", \"GOOGLE_CLOUD_STORAGE_V2\", \"AMAZON_S3_V2\", \"AMAZON_SQS_V2\", \"AZURE_BLOBSTORE_V2\", \"GOOGLE_CLOUD_STORAGE_EVENT_DRIVEN\"]"]
    pub fn set_feed_source_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feed_source_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe ingestion metadata labels to apply to all logs ingested through this\nfeed, and the resulting normalized data."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `amazon_kinesis_firehose_settings`.\n"]
    pub fn set_amazon_kinesis_firehose_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AmazonKinesisFirehoseSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_kinesis_firehose_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_kinesis_firehose_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `amazon_s3_settings`.\n"]
    pub fn set_amazon_s3_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AmazonS3Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_s3_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_s3_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `amazon_s3_v2_settings`.\n"]
    pub fn set_amazon_s3_v2_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AmazonS3V2Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_s3_v2_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_s3_v2_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `amazon_sqs_settings`.\n"]
    pub fn set_amazon_sqs_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AmazonSqsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_sqs_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_sqs_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `amazon_sqs_v2_settings`.\n"]
    pub fn set_amazon_sqs_v2_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AmazonSqsV2Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_sqs_v2_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_sqs_v2_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `anomali_settings`.\n"]
    pub fn set_anomali_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AnomaliSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.anomali_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.anomali_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_ec2_hosts_settings`.\n"]
    pub fn set_aws_ec2_hosts_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsEc2HostsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_ec2_hosts_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_ec2_hosts_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_ec2_instances_settings`.\n"]
    pub fn set_aws_ec2_instances_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsEc2InstancesSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_ec2_instances_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_ec2_instances_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_ec2_vpcs_settings`.\n"]
    pub fn set_aws_ec2_vpcs_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsEc2VpcsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_ec2_vpcs_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_ec2_vpcs_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_iam_settings`.\n"]
    pub fn set_aws_iam_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AwsIamSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_iam_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_iam_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_ad_audit_settings`.\n"]
    pub fn set_azure_ad_audit_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureAdAuditSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_ad_audit_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_ad_audit_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_ad_context_settings`.\n"]
    pub fn set_azure_ad_context_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureAdContextSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_ad_context_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_ad_context_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_ad_settings`.\n"]
    pub fn set_azure_ad_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureAdSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_ad_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_ad_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_blob_store_settings`.\n"]
    pub fn set_azure_blob_store_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureBlobStoreSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_blob_store_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_blob_store_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_blob_store_v2_settings`.\n"]
    pub fn set_azure_blob_store_v2_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureBlobStoreV2Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_blob_store_v2_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_blob_store_v2_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_event_hub_settings`.\n"]
    pub fn set_azure_event_hub_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureEventHubSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_event_hub_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_event_hub_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `azure_mdm_intune_settings`.\n"]
    pub fn set_azure_mdm_intune_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_AzureMdmIntuneSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.azure_mdm_intune_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.azure_mdm_intune_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_passage_settings`.\n"]
    pub fn set_cloud_passage_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_CloudPassageSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_passage_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_passage_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cortex_xdr_settings`.\n"]
    pub fn set_cortex_xdr_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_CortexXdrSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cortex_xdr_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cortex_xdr_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `crowdstrike_alerts_settings`.\n"]
    pub fn set_crowdstrike_alerts_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_CrowdstrikeAlertsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.crowdstrike_alerts_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.crowdstrike_alerts_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `crowdstrike_detects_settings`.\n"]
    pub fn set_crowdstrike_detects_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_CrowdstrikeDetectsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.crowdstrike_detects_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.crowdstrike_detects_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dummy_log_type_settings`.\n"]
    pub fn set_dummy_log_type_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_DummyLogTypeSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dummy_log_type_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dummy_log_type_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `duo_auth_settings`.\n"]
    pub fn set_duo_auth_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_DuoAuthSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.duo_auth_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.duo_auth_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `duo_user_context_settings`.\n"]
    pub fn set_duo_user_context_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_DuoUserContextSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.duo_user_context_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.duo_user_context_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fox_it_stix_settings`.\n"]
    pub fn set_fox_it_stix_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_FoxItStixSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fox_it_stix_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fox_it_stix_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_settings`.\n"]
    pub fn set_gcs_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_GcsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `gcs_v2_settings`.\n"]
    pub fn set_gcs_v2_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_GcsV2Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.gcs_v2_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.gcs_v2_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_cloud_identity_device_users_settings`.\n"]
    pub fn set_google_cloud_identity_device_users_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_cloud_identity_device_users_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud_identity_device_users_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_cloud_identity_devices_settings`.\n"]
    pub fn set_google_cloud_identity_devices_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_GoogleCloudIdentityDevicesSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_cloud_identity_devices_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud_identity_devices_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_cloud_storage_event_driven_settings`.\n"]
    pub fn set_google_cloud_storage_event_driven_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_cloud_storage_event_driven_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud_storage_event_driven_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_settings`.\n"]
    pub fn set_http_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_HttpSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `https_push_amazon_kinesis_firehose_settings`.\n"]
    pub fn set_https_push_amazon_kinesis_firehose_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.https_push_amazon_kinesis_firehose_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.https_push_amazon_kinesis_firehose_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `https_push_google_cloud_pubsub_settings`.\n"]
    pub fn set_https_push_google_cloud_pubsub_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.https_push_google_cloud_pubsub_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.https_push_google_cloud_pubsub_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `https_push_webhook_settings`.\n"]
    pub fn set_https_push_webhook_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_HttpsPushWebhookSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.https_push_webhook_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.https_push_webhook_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `imperva_waf_settings`.\n"]
    pub fn set_imperva_waf_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_ImpervaWafSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.imperva_waf_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.imperva_waf_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mandiant_ioc_settings`.\n"]
    pub fn set_mandiant_ioc_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_MandiantIocSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mandiant_ioc_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mandiant_ioc_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `microsoft_graph_alert_settings`.\n"]
    pub fn set_microsoft_graph_alert_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_MicrosoftGraphAlertSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.microsoft_graph_alert_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.microsoft_graph_alert_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `microsoft_security_center_alert_settings`.\n"]
    pub fn set_microsoft_security_center_alert_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.microsoft_security_center_alert_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.microsoft_security_center_alert_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mimecast_mail_settings`.\n"]
    pub fn set_mimecast_mail_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_MimecastMailSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mimecast_mail_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mimecast_mail_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mimecast_mail_v2_settings`.\n"]
    pub fn set_mimecast_mail_v2_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_MimecastMailV2Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mimecast_mail_v2_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mimecast_mail_v2_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `netskope_alert_settings`.\n"]
    pub fn set_netskope_alert_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_NetskopeAlertSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.netskope_alert_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.netskope_alert_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `netskope_alert_v2_settings`.\n"]
    pub fn set_netskope_alert_v2_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_NetskopeAlertV2Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.netskope_alert_v2_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.netskope_alert_v2_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `office365_settings`.\n"]
    pub fn set_office365_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_Office365Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.office365_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.office365_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `okta_settings`.\n"]
    pub fn set_okta_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.okta_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.okta_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `okta_user_context_settings`.\n"]
    pub fn set_okta_user_context_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_OktaUserContextSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.okta_user_context_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.okta_user_context_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pan_ioc_settings`.\n"]
    pub fn set_pan_ioc_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_PanIocSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pan_ioc_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pan_ioc_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pan_prisma_cloud_settings`.\n"]
    pub fn set_pan_prisma_cloud_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_PanPrismaCloudSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pan_prisma_cloud_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pan_prisma_cloud_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proofpoint_mail_settings`.\n"]
    pub fn set_proofpoint_mail_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_ProofpointMailSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proofpoint_mail_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proofpoint_mail_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proofpoint_on_demand_settings`.\n"]
    pub fn set_proofpoint_on_demand_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_ProofpointOnDemandSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proofpoint_on_demand_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proofpoint_on_demand_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pubsub_settings`.\n"]
    pub fn set_pubsub_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_PubsubSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pubsub_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `qualys_scan_settings`.\n"]
    pub fn set_qualys_scan_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_QualysScanSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.qualys_scan_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.qualys_scan_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `qualys_vm_settings`.\n"]
    pub fn set_qualys_vm_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_QualysVmSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.qualys_vm_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.qualys_vm_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rapid7_insight_settings`.\n"]
    pub fn set_rapid7_insight_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_Rapid7InsightSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rapid7_insight_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rapid7_insight_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recorded_future_ioc_settings`.\n"]
    pub fn set_recorded_future_ioc_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_RecordedFutureIocSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recorded_future_ioc_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recorded_future_ioc_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rh_isac_ioc_settings`.\n"]
    pub fn set_rh_isac_ioc_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_RhIsacIocSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rh_isac_ioc_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rh_isac_ioc_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `salesforce_settings`.\n"]
    pub fn set_salesforce_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_SalesforceSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.salesforce_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.salesforce_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sentinelone_alert_settings`.\n"]
    pub fn set_sentinelone_alert_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_SentineloneAlertSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sentinelone_alert_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sentinelone_alert_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_now_cmdb_settings`.\n"]
    pub fn set_service_now_cmdb_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_ServiceNowCmdbSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_now_cmdb_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_now_cmdb_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sftp_settings`.\n"]
    pub fn set_sftp_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_SftpSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sftp_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sftp_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `symantec_event_export_settings`.\n"]
    pub fn set_symantec_event_export_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_SymantecEventExportSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.symantec_event_export_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.symantec_event_export_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `thinkst_canary_settings`.\n"]
    pub fn set_thinkst_canary_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_ThinkstCanarySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.thinkst_canary_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.thinkst_canary_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_connect_ioc_settings`.\n"]
    pub fn set_threat_connect_ioc_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_ThreatConnectIocSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_connect_ioc_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_connect_ioc_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_connect_ioc_v3_settings`.\n"]
    pub fn set_threat_connect_ioc_v3_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_ThreatConnectIocV3Settings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_connect_ioc_v3_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_connect_ioc_v3_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trellix_hx_alerts_settings`.\n"]
    pub fn set_trellix_hx_alerts_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxAlertsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trellix_hx_alerts_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trellix_hx_alerts_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trellix_hx_bulk_acqs_settings`.\n"]
    pub fn set_trellix_hx_bulk_acqs_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxBulkAcqsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trellix_hx_bulk_acqs_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trellix_hx_bulk_acqs_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trellix_hx_hosts_settings`.\n"]
    pub fn set_trellix_hx_hosts_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_TrellixHxHostsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trellix_hx_hosts_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trellix_hx_hosts_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `webhook_settings`.\n"]
    pub fn set_webhook_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_WebhookSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.webhook_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.webhook_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workday_settings`.\n"]
    pub fn set_workday_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkdaySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workday_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workday_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_activity_settings`.\n"]
    pub fn set_workspace_activity_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceActivitySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workspace_activity_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workspace_activity_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_alerts_settings`.\n"]
    pub fn set_workspace_alerts_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceAlertsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workspace_alerts_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workspace_alerts_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_chrome_os_settings`.\n"]
    pub fn set_workspace_chrome_os_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceChromeOsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workspace_chrome_os_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workspace_chrome_os_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_groups_settings`.\n"]
    pub fn set_workspace_groups_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceGroupsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workspace_groups_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workspace_groups_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_mobile_settings`.\n"]
    pub fn set_workspace_mobile_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceMobileSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workspace_mobile_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workspace_mobile_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_privileges_settings`.\n"]
    pub fn set_workspace_privileges_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspacePrivilegesSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workspace_privileges_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workspace_privileges_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workspace_users_settings`.\n"]
    pub fn set_workspace_users_settings(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleFeed_Details_WorkspaceUsersSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workspace_users_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workspace_users_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleFeed_Details {
    type O = terrars::BlockAssignable<ChronicleFeed_Details>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Details {
    #[doc = "LogType.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/logTypes/{log_type}"]
    pub log_type: terrars::PrimField<std::string::String>,
}

impl BuildChronicleFeed_Details {
    pub fn build(self) -> ChronicleFeed_Details {
        ChronicleFeed_Details {
            asset_namespace: core::default::Default::default(),
            feed_source_type: core::default::Default::default(),
            labels: core::default::Default::default(),
            log_type: self.log_type,
            amazon_kinesis_firehose_settings: core::default::Default::default(),
            amazon_s3_settings: core::default::Default::default(),
            amazon_s3_v2_settings: core::default::Default::default(),
            amazon_sqs_settings: core::default::Default::default(),
            amazon_sqs_v2_settings: core::default::Default::default(),
            anomali_settings: core::default::Default::default(),
            aws_ec2_hosts_settings: core::default::Default::default(),
            aws_ec2_instances_settings: core::default::Default::default(),
            aws_ec2_vpcs_settings: core::default::Default::default(),
            aws_iam_settings: core::default::Default::default(),
            azure_ad_audit_settings: core::default::Default::default(),
            azure_ad_context_settings: core::default::Default::default(),
            azure_ad_settings: core::default::Default::default(),
            azure_blob_store_settings: core::default::Default::default(),
            azure_blob_store_v2_settings: core::default::Default::default(),
            azure_event_hub_settings: core::default::Default::default(),
            azure_mdm_intune_settings: core::default::Default::default(),
            cloud_passage_settings: core::default::Default::default(),
            cortex_xdr_settings: core::default::Default::default(),
            crowdstrike_alerts_settings: core::default::Default::default(),
            crowdstrike_detects_settings: core::default::Default::default(),
            dummy_log_type_settings: core::default::Default::default(),
            duo_auth_settings: core::default::Default::default(),
            duo_user_context_settings: core::default::Default::default(),
            fox_it_stix_settings: core::default::Default::default(),
            gcs_settings: core::default::Default::default(),
            gcs_v2_settings: core::default::Default::default(),
            google_cloud_identity_device_users_settings: core::default::Default::default(),
            google_cloud_identity_devices_settings: core::default::Default::default(),
            google_cloud_storage_event_driven_settings: core::default::Default::default(),
            http_settings: core::default::Default::default(),
            https_push_amazon_kinesis_firehose_settings: core::default::Default::default(),
            https_push_google_cloud_pubsub_settings: core::default::Default::default(),
            https_push_webhook_settings: core::default::Default::default(),
            imperva_waf_settings: core::default::Default::default(),
            mandiant_ioc_settings: core::default::Default::default(),
            microsoft_graph_alert_settings: core::default::Default::default(),
            microsoft_security_center_alert_settings: core::default::Default::default(),
            mimecast_mail_settings: core::default::Default::default(),
            mimecast_mail_v2_settings: core::default::Default::default(),
            netskope_alert_settings: core::default::Default::default(),
            netskope_alert_v2_settings: core::default::Default::default(),
            office365_settings: core::default::Default::default(),
            okta_settings: core::default::Default::default(),
            okta_user_context_settings: core::default::Default::default(),
            pan_ioc_settings: core::default::Default::default(),
            pan_prisma_cloud_settings: core::default::Default::default(),
            proofpoint_mail_settings: core::default::Default::default(),
            proofpoint_on_demand_settings: core::default::Default::default(),
            pubsub_settings: core::default::Default::default(),
            qualys_scan_settings: core::default::Default::default(),
            qualys_vm_settings: core::default::Default::default(),
            rapid7_insight_settings: core::default::Default::default(),
            recorded_future_ioc_settings: core::default::Default::default(),
            rh_isac_ioc_settings: core::default::Default::default(),
            salesforce_settings: core::default::Default::default(),
            sentinelone_alert_settings: core::default::Default::default(),
            service_now_cmdb_settings: core::default::Default::default(),
            sftp_settings: core::default::Default::default(),
            symantec_event_export_settings: core::default::Default::default(),
            thinkst_canary_settings: core::default::Default::default(),
            threat_connect_ioc_settings: core::default::Default::default(),
            threat_connect_ioc_v3_settings: core::default::Default::default(),
            trellix_hx_alerts_settings: core::default::Default::default(),
            trellix_hx_bulk_acqs_settings: core::default::Default::default(),
            trellix_hx_hosts_settings: core::default::Default::default(),
            webhook_settings: core::default::Default::default(),
            workday_settings: core::default::Default::default(),
            workspace_activity_settings: core::default::Default::default(),
            workspace_alerts_settings: core::default::Default::default(),
            workspace_chrome_os_settings: core::default::Default::default(),
            workspace_groups_settings: core::default::Default::default(),
            workspace_mobile_settings: core::default::Default::default(),
            workspace_privileges_settings: core::default::Default::default(),
            workspace_users_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_DetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_DetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_DetailsRef {
        ChronicleFeed_DetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_DetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `asset_namespace` after provisioning.\nThe asset namespace to apply to all logs ingested through this feed."]
    pub fn asset_namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_namespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `feed_source_type` after provisioning.\nSource Type of the feed.\nPossible values:\nGOOGLE_CLOUD_STORAGE\nHTTP\nSFTP\nAMAZON_S3\nAZURE_BLOBSTORE\nAPI\nAMAZON_SQS\nPUBSUB\nAMAZON_KINESIS_FIREHOSE\nWEBHOOK\nHTTPS_PUSH_GOOGLE_CLOUD_PUBSUB\nHTTPS_PUSH_AMAZON_KINESIS_FIREHOSE\nHTTPS_PUSH_WEBHOOK\nAZURE_EVENT_HUB\nGOOGLE_CLOUD_STORAGE_V2\nAMAZON_S3_V2\nAMAZON_SQS_V2\nAZURE_BLOBSTORE_V2\nGOOGLE_CLOUD_STORAGE_EVENT_DRIVEN Possible values: [\"GOOGLE_CLOUD_STORAGE\", \"HTTP\", \"SFTP\", \"AMAZON_S3\", \"AZURE_BLOBSTORE\", \"API\", \"AMAZON_SQS\", \"PUBSUB\", \"AMAZON_KINESIS_FIREHOSE\", \"WEBHOOK\", \"HTTPS_PUSH_GOOGLE_CLOUD_PUBSUB\", \"HTTPS_PUSH_AMAZON_KINESIS_FIREHOSE\", \"HTTPS_PUSH_WEBHOOK\", \"AZURE_EVENT_HUB\", \"GOOGLE_CLOUD_STORAGE_V2\", \"AMAZON_S3_V2\", \"AMAZON_SQS_V2\", \"AZURE_BLOBSTORE_V2\", \"GOOGLE_CLOUD_STORAGE_EVENT_DRIVEN\"]"]
    pub fn feed_source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feed_source_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe ingestion metadata labels to apply to all logs ingested through this\nfeed, and the resulting normalized data."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `log_type` after provisioning.\nLogType.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/logTypes/{log_type}"]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sts_migration_readiness` after provisioning.\nThe status of the feed's migration to STS.\nPossible values:\nNOT_READY\nREADY\nAUTH_RECONFIG_REQUIRED"]
    pub fn sts_migration_readiness(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sts_migration_readiness", self.base))
    }

    #[doc = "Get a reference to the value of field `amazon_kinesis_firehose_settings` after provisioning.\n"]
    pub fn amazon_kinesis_firehose_settings(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_AmazonKinesisFirehoseSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_kinesis_firehose_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `amazon_s3_settings` after provisioning.\n"]
    pub fn amazon_s3_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AmazonS3SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_s3_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `amazon_s3_v2_settings` after provisioning.\n"]
    pub fn amazon_s3_v2_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AmazonS3V2SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_s3_v2_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `amazon_sqs_settings` after provisioning.\n"]
    pub fn amazon_sqs_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AmazonSqsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_sqs_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `amazon_sqs_v2_settings` after provisioning.\n"]
    pub fn amazon_sqs_v2_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AmazonSqsV2SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_sqs_v2_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `anomali_settings` after provisioning.\n"]
    pub fn anomali_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AnomaliSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.anomali_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_ec2_hosts_settings` after provisioning.\n"]
    pub fn aws_ec2_hosts_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsEc2HostsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_ec2_hosts_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_ec2_instances_settings` after provisioning.\n"]
    pub fn aws_ec2_instances_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsEc2InstancesSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_ec2_instances_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_ec2_vpcs_settings` after provisioning.\n"]
    pub fn aws_ec2_vpcs_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsEc2VpcsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_ec2_vpcs_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_iam_settings` after provisioning.\n"]
    pub fn aws_iam_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AwsIamSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_iam_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_ad_audit_settings` after provisioning.\n"]
    pub fn azure_ad_audit_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureAdAuditSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_ad_audit_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_ad_context_settings` after provisioning.\n"]
    pub fn azure_ad_context_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureAdContextSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_ad_context_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_ad_settings` after provisioning.\n"]
    pub fn azure_ad_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureAdSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_ad_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_blob_store_settings` after provisioning.\n"]
    pub fn azure_blob_store_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureBlobStoreSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_blob_store_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_blob_store_v2_settings` after provisioning.\n"]
    pub fn azure_blob_store_v2_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureBlobStoreV2SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_blob_store_v2_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_event_hub_settings` after provisioning.\n"]
    pub fn azure_event_hub_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureEventHubSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_event_hub_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `azure_mdm_intune_settings` after provisioning.\n"]
    pub fn azure_mdm_intune_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_AzureMdmIntuneSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.azure_mdm_intune_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_passage_settings` after provisioning.\n"]
    pub fn cloud_passage_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_CloudPassageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_passage_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `cortex_xdr_settings` after provisioning.\n"]
    pub fn cortex_xdr_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_CortexXdrSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cortex_xdr_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `crowdstrike_alerts_settings` after provisioning.\n"]
    pub fn crowdstrike_alerts_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_CrowdstrikeAlertsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crowdstrike_alerts_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `crowdstrike_detects_settings` after provisioning.\n"]
    pub fn crowdstrike_detects_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_CrowdstrikeDetectsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crowdstrike_detects_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `dummy_log_type_settings` after provisioning.\n"]
    pub fn dummy_log_type_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_DummyLogTypeSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dummy_log_type_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `duo_auth_settings` after provisioning.\n"]
    pub fn duo_auth_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_DuoAuthSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.duo_auth_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `duo_user_context_settings` after provisioning.\n"]
    pub fn duo_user_context_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_DuoUserContextSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.duo_user_context_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `fox_it_stix_settings` after provisioning.\n"]
    pub fn fox_it_stix_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_FoxItStixSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fox_it_stix_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_settings` after provisioning.\n"]
    pub fn gcs_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_GcsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_v2_settings` after provisioning.\n"]
    pub fn gcs_v2_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_GcsV2SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gcs_v2_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `google_cloud_identity_device_users_settings` after provisioning.\n"]
    pub fn google_cloud_identity_device_users_settings(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_GoogleCloudIdentityDeviceUsersSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.google_cloud_identity_device_users_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `google_cloud_identity_devices_settings` after provisioning.\n"]
    pub fn google_cloud_identity_devices_settings(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_GoogleCloudIdentityDevicesSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_cloud_identity_devices_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `google_cloud_storage_event_driven_settings` after provisioning.\n"]
    pub fn google_cloud_storage_event_driven_settings(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_GoogleCloudStorageEventDrivenSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.google_cloud_storage_event_driven_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `http_settings` after provisioning.\n"]
    pub fn http_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_HttpSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `https_push_amazon_kinesis_firehose_settings` after provisioning.\n"]
    pub fn https_push_amazon_kinesis_firehose_settings(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_HttpsPushAmazonKinesisFirehoseSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.https_push_amazon_kinesis_firehose_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `https_push_google_cloud_pubsub_settings` after provisioning.\n"]
    pub fn https_push_google_cloud_pubsub_settings(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_HttpsPushGoogleCloudPubsubSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.https_push_google_cloud_pubsub_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `https_push_webhook_settings` after provisioning.\n"]
    pub fn https_push_webhook_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_HttpsPushWebhookSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.https_push_webhook_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `imperva_waf_settings` after provisioning.\n"]
    pub fn imperva_waf_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_ImpervaWafSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.imperva_waf_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `mandiant_ioc_settings` after provisioning.\n"]
    pub fn mandiant_ioc_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_MandiantIocSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mandiant_ioc_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `microsoft_graph_alert_settings` after provisioning.\n"]
    pub fn microsoft_graph_alert_settings(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_MicrosoftGraphAlertSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.microsoft_graph_alert_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `microsoft_security_center_alert_settings` after provisioning.\n"]
    pub fn microsoft_security_center_alert_settings(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_MicrosoftSecurityCenterAlertSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.microsoft_security_center_alert_settings", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `mimecast_mail_settings` after provisioning.\n"]
    pub fn mimecast_mail_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_MimecastMailSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mimecast_mail_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `mimecast_mail_v2_settings` after provisioning.\n"]
    pub fn mimecast_mail_v2_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_MimecastMailV2SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mimecast_mail_v2_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `netskope_alert_settings` after provisioning.\n"]
    pub fn netskope_alert_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_NetskopeAlertSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.netskope_alert_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `netskope_alert_v2_settings` after provisioning.\n"]
    pub fn netskope_alert_v2_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_NetskopeAlertV2SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.netskope_alert_v2_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `office365_settings` after provisioning.\n"]
    pub fn office365_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_Office365SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.office365_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `okta_settings` after provisioning.\n"]
    pub fn okta_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.okta_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `okta_user_context_settings` after provisioning.\n"]
    pub fn okta_user_context_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_OktaUserContextSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.okta_user_context_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `pan_ioc_settings` after provisioning.\n"]
    pub fn pan_ioc_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_PanIocSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pan_ioc_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `pan_prisma_cloud_settings` after provisioning.\n"]
    pub fn pan_prisma_cloud_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_PanPrismaCloudSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pan_prisma_cloud_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `proofpoint_mail_settings` after provisioning.\n"]
    pub fn proofpoint_mail_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_ProofpointMailSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proofpoint_mail_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `proofpoint_on_demand_settings` after provisioning.\n"]
    pub fn proofpoint_on_demand_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_ProofpointOnDemandSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proofpoint_on_demand_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `pubsub_settings` after provisioning.\n"]
    pub fn pubsub_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_PubsubSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `qualys_scan_settings` after provisioning.\n"]
    pub fn qualys_scan_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_QualysScanSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qualys_scan_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `qualys_vm_settings` after provisioning.\n"]
    pub fn qualys_vm_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_QualysVmSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.qualys_vm_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `rapid7_insight_settings` after provisioning.\n"]
    pub fn rapid7_insight_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_Rapid7InsightSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rapid7_insight_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `recorded_future_ioc_settings` after provisioning.\n"]
    pub fn recorded_future_ioc_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_RecordedFutureIocSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recorded_future_ioc_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `rh_isac_ioc_settings` after provisioning.\n"]
    pub fn rh_isac_ioc_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_RhIsacIocSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rh_isac_ioc_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `salesforce_settings` after provisioning.\n"]
    pub fn salesforce_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_SalesforceSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.salesforce_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `sentinelone_alert_settings` after provisioning.\n"]
    pub fn sentinelone_alert_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_SentineloneAlertSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sentinelone_alert_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now_cmdb_settings` after provisioning.\n"]
    pub fn service_now_cmdb_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_ServiceNowCmdbSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_now_cmdb_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `sftp_settings` after provisioning.\n"]
    pub fn sftp_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_SftpSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sftp_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `symantec_event_export_settings` after provisioning.\n"]
    pub fn symantec_event_export_settings(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_SymantecEventExportSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.symantec_event_export_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `thinkst_canary_settings` after provisioning.\n"]
    pub fn thinkst_canary_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_ThinkstCanarySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.thinkst_canary_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `threat_connect_ioc_settings` after provisioning.\n"]
    pub fn threat_connect_ioc_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_ThreatConnectIocSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threat_connect_ioc_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `threat_connect_ioc_v3_settings` after provisioning.\n"]
    pub fn threat_connect_ioc_v3_settings(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_ThreatConnectIocV3SettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threat_connect_ioc_v3_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `trellix_hx_alerts_settings` after provisioning.\n"]
    pub fn trellix_hx_alerts_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_TrellixHxAlertsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trellix_hx_alerts_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `trellix_hx_bulk_acqs_settings` after provisioning.\n"]
    pub fn trellix_hx_bulk_acqs_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_TrellixHxBulkAcqsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trellix_hx_bulk_acqs_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `trellix_hx_hosts_settings` after provisioning.\n"]
    pub fn trellix_hx_hosts_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_TrellixHxHostsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trellix_hx_hosts_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `webhook_settings` after provisioning.\n"]
    pub fn webhook_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_WebhookSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.webhook_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workday_settings` after provisioning.\n"]
    pub fn workday_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkdaySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workday_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_activity_settings` after provisioning.\n"]
    pub fn workspace_activity_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceActivitySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_activity_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_alerts_settings` after provisioning.\n"]
    pub fn workspace_alerts_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceAlertsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_alerts_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_chrome_os_settings` after provisioning.\n"]
    pub fn workspace_chrome_os_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceChromeOsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_chrome_os_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_groups_settings` after provisioning.\n"]
    pub fn workspace_groups_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceGroupsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_groups_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_mobile_settings` after provisioning.\n"]
    pub fn workspace_mobile_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceMobileSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_mobile_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_privileges_settings` after provisioning.\n"]
    pub fn workspace_privileges_settings(
        &self,
    ) -> terrars::ListRef<ChronicleFeed_Details_WorkspacePrivilegesSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_privileges_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `workspace_users_settings` after provisioning.\n"]
    pub fn workspace_users_settings(&self) -> terrars::ListRef<ChronicleFeed_Details_WorkspaceUsersSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workspace_users_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_FailureDetails {}

impl ChronicleFeed_FailureDetails { }

impl terrars::ToListMappable for ChronicleFeed_FailureDetails {
    type O = terrars::BlockAssignable<ChronicleFeed_FailureDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_FailureDetails {}

impl BuildChronicleFeed_FailureDetails {
    pub fn build(self) -> ChronicleFeed_FailureDetails {
        ChronicleFeed_FailureDetails {}
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_FailureDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_FailureDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_FailureDetailsRef {
        ChronicleFeed_FailureDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_FailureDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `error_action` after provisioning.\nerror_action contains the user action prescribed for remediation of feed\nerror."]
    pub fn error_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `error_cause` after provisioning.\nerror_cause contains the information regarding the failure cause."]
    pub fn error_cause(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_cause", self.base))
    }

    #[doc =
        "Get a reference to the value of field `error_code` after provisioning.\nerror_code contains the error code for the feed. The field is populated for\nthe feeds with failed status."]
    pub fn error_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_code", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_error_code` after provisioning.\nhttp_error_code contains the HTTP error code for the feed failure.\nfeed transfer failure may or may not result in http error code."]
    pub fn http_error_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_error_code", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleFeed_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleFeed_Timeouts {
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

impl terrars::ToListMappable for ChronicleFeed_Timeouts {
    type O = terrars::BlockAssignable<ChronicleFeed_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleFeed_Timeouts {}

impl BuildChronicleFeed_Timeouts {
    pub fn build(self) -> ChronicleFeed_Timeouts {
        ChronicleFeed_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleFeed_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleFeed_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleFeed_TimeoutsRef {
        ChronicleFeed_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleFeed_TimeoutsRef {
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
