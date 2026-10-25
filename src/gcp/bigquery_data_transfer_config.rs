#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryDataTransferConfigData_ {
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
    data_refresh_window_days: std::option::Option<terrars::PrimField<f64>>,
    data_source_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_pubsub_topic: std::option::Option<terrars::PrimField<std::string::String>>,
    params: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_preferences: std::option::Option<std::vec::Vec<BigqueryDataTransferConfig_EmailPreferences>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<BigqueryDataTransferConfig_EncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_options: std::option::Option<std::vec::Vec<BigqueryDataTransferConfig_ScheduleOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitive_params: std::option::Option<std::vec::Vec<BigqueryDataTransferConfig_SensitiveParams>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigqueryDataTransferConfig_Timeouts>,
    dynamic: BigqueryDataTransferConfigDynamic,
}

struct BigqueryDataTransferConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryDataTransferConfigData_>,
}

#[derive(Clone)]
pub struct BigqueryDataTransferConfig(std::rc::Rc<BigqueryDataTransferConfig_>);

impl BigqueryDataTransferConfig {
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
        "Set the field `data_refresh_window_days`.\nThe number of days to look back to automatically refresh the data.\nFor example, if dataRefreshWindowDays = 10, then every day BigQuery\nreingests data for [today-10, today-1], rather than ingesting data for\njust [today-1]. Only valid if the data source supports the feature.\nSet the value to 0 to use the default value."]
    pub fn set_data_refresh_window_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().data_refresh_window_days = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_dataset_id`.\nThe BigQuery target dataset id."]
    pub fn set_destination_dataset_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().destination_dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\nWhen set to true, no runs are scheduled for a given transfer."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `notification_pubsub_topic`.\nPub/Sub topic where notifications will be sent after transfer runs\nassociated with this transfer config finish."]
    pub fn set_notification_pubsub_topic(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().notification_pubsub_topic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schedule`.\nData transfer schedule. If the data source does not support a custom\nschedule, this should be empty. If it is empty, the default value for\nthe data source will be used. The specified times are in UTC. Examples\nof valid format: 1st,3rd monday of month 15:30, every wed,fri of jan,\njun 13:15, and first sunday of quarter 00:00. See more explanation\nabout the format here:\nhttps://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format\nNOTE: The minimum interval time between recurring transfers depends\non the data source; refer to the documentation for your data source."]
    pub fn set_schedule(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_name`.\nService account email. If this field is set, transfer config will\nbe created with this service account credentials. It requires that\nrequesting user calling this API has permissions to act as this service account."]
    pub fn set_service_account_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_account_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_preferences`.\n"]
    pub fn set_email_preferences(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryDataTransferConfig_EmailPreferences>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().email_preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.email_preferences = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryDataTransferConfig_EncryptionConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule_options`.\n"]
    pub fn set_schedule_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryDataTransferConfig_ScheduleOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sensitive_params`.\n"]
    pub fn set_sensitive_params(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryDataTransferConfig_SensitiveParams>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sensitive_params = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sensitive_params = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigqueryDataTransferConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `data_refresh_window_days` after provisioning.\nThe number of days to look back to automatically refresh the data.\nFor example, if dataRefreshWindowDays = 10, then every day BigQuery\nreingests data for [today-10, today-1], rather than ingesting data for\njust [today-1]. Only valid if the data source supports the feature.\nSet the value to 0 to use the default value."]
    pub fn data_refresh_window_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_refresh_window_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source_id` after provisioning.\nThe data source id. Cannot be changed once the transfer config is created."]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destination_dataset_id` after provisioning.\nThe BigQuery target dataset id."]
    pub fn destination_dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhen set to true, no runs are scheduled for a given transfer."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe user specified display name for the transfer config."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the transfer config. Transfer config names have the\nform projects/{projectId}/locations/{location}/transferConfigs/{configId}\nor projects/{projectId}/transferConfigs/{configId},\nwhere configId is usually a uuid, but this is not required.\nThe name is ignored when creating a transfer config."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_pubsub_topic` after provisioning.\nPub/Sub topic where notifications will be sent after transfer runs\nassociated with this transfer config finish."]
    pub fn notification_pubsub_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_pubsub_topic", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nParameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer'\nsection for each data source. For example the parameters for Cloud Storage transfers are listed here:\nhttps://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq\n\n**NOTE** : If you are attempting to update a parameter that cannot be updated (due to api limitations) [please force recreation of the resource](https://www.terraform.io/cli/state/taint#forcing-re-creation-of-resources)."]
    pub fn params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nData transfer schedule. If the data source does not support a custom\nschedule, this should be empty. If it is empty, the default value for\nthe data source will be used. The specified times are in UTC. Examples\nof valid format: 1st,3rd monday of month 15:30, every wed,fri of jan,\njun 13:15, and first sunday of quarter 00:00. See more explanation\nabout the format here:\nhttps://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format\nNOTE: The minimum interval time between recurring transfers depends\non the data source; refer to the documentation for your data source."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account_name` after provisioning.\nService account email. If this field is set, transfer config will\nbe created with this service account credentials. It requires that\nrequesting user calling this API has permissions to act as this service account."]
    pub fn service_account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_preferences` after provisioning.\n"]
    pub fn email_preferences(&self) -> terrars::ListRef<BigqueryDataTransferConfig_EmailPreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_preferences", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<BigqueryDataTransferConfig_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_options` after provisioning.\n"]
    pub fn schedule_options(&self) -> terrars::ListRef<BigqueryDataTransferConfig_ScheduleOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sensitive_params` after provisioning.\n"]
    pub fn sensitive_params(&self) -> terrars::ListRef<BigqueryDataTransferConfig_SensitiveParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitive_params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDataTransferConfig_TimeoutsRef {
        BigqueryDataTransferConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BigqueryDataTransferConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigqueryDataTransferConfig { }

impl terrars::ToListMappable for BigqueryDataTransferConfig {
    type O = terrars::ListRef<BigqueryDataTransferConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigqueryDataTransferConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigquery_data_transfer_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryDataTransferConfig {
    pub tf_id: std::string::String,
    #[doc = "The data source id. Cannot be changed once the transfer config is created."]
    pub data_source_id: terrars::PrimField<std::string::String>,
    #[doc = "The user specified display name for the transfer config."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Parameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer'\nsection for each data source. For example the parameters for Cloud Storage transfers are listed here:\nhttps://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq\n\n**NOTE** : If you are attempting to update a parameter that cannot be updated (due to api limitations) [please force recreation of the resource](https://www.terraform.io/cli/state/taint#forcing-re-creation-of-resources)."]
    pub params: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildBigqueryDataTransferConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> BigqueryDataTransferConfig {
        let out = BigqueryDataTransferConfig(std::rc::Rc::new(BigqueryDataTransferConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigqueryDataTransferConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_refresh_window_days: core::default::Default::default(),
                data_source_id: self.data_source_id,
                deletion_policy: core::default::Default::default(),
                destination_dataset_id: core::default::Default::default(),
                disabled: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                notification_pubsub_topic: core::default::Default::default(),
                params: self.params,
                project: core::default::Default::default(),
                schedule: core::default::Default::default(),
                service_account_name: core::default::Default::default(),
                email_preferences: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                schedule_options: core::default::Default::default(),
                sensitive_params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryDataTransferConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDataTransferConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryDataTransferConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_refresh_window_days` after provisioning.\nThe number of days to look back to automatically refresh the data.\nFor example, if dataRefreshWindowDays = 10, then every day BigQuery\nreingests data for [today-10, today-1], rather than ingesting data for\njust [today-1]. Only valid if the data source supports the feature.\nSet the value to 0 to use the default value."]
    pub fn data_refresh_window_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_refresh_window_days", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_source_id` after provisioning.\nThe data source id. Cannot be changed once the transfer config is created."]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `destination_dataset_id` after provisioning.\nThe BigQuery target dataset id."]
    pub fn destination_dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhen set to true, no runs are scheduled for a given transfer."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe user specified display name for the transfer config."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the transfer config should reside.\nExamples: US, EU, asia-northeast1. The default value is US."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the transfer config. Transfer config names have the\nform projects/{projectId}/locations/{location}/transferConfigs/{configId}\nor projects/{projectId}/transferConfigs/{configId},\nwhere configId is usually a uuid, but this is not required.\nThe name is ignored when creating a transfer config."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_pubsub_topic` after provisioning.\nPub/Sub topic where notifications will be sent after transfer runs\nassociated with this transfer config finish."]
    pub fn notification_pubsub_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_pubsub_topic", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `params` after provisioning.\nParameters specific to each data source. For more information see the bq tab in the 'Setting up a data transfer'\nsection for each data source. For example the parameters for Cloud Storage transfers are listed here:\nhttps://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq\n\n**NOTE** : If you are attempting to update a parameter that cannot be updated (due to api limitations) [please force recreation of the resource](https://www.terraform.io/cli/state/taint#forcing-re-creation-of-resources)."]
    pub fn params(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `schedule` after provisioning.\nData transfer schedule. If the data source does not support a custom\nschedule, this should be empty. If it is empty, the default value for\nthe data source will be used. The specified times are in UTC. Examples\nof valid format: 1st,3rd monday of month 15:30, every wed,fri of jan,\njun 13:15, and first sunday of quarter 00:00. See more explanation\nabout the format here:\nhttps://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format\nNOTE: The minimum interval time between recurring transfers depends\non the data source; refer to the documentation for your data source."]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_account_name` after provisioning.\nService account email. If this field is set, transfer config will\nbe created with this service account credentials. It requires that\nrequesting user calling this API has permissions to act as this service account."]
    pub fn service_account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_preferences` after provisioning.\n"]
    pub fn email_preferences(&self) -> terrars::ListRef<BigqueryDataTransferConfig_EmailPreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_preferences", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<BigqueryDataTransferConfig_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule_options` after provisioning.\n"]
    pub fn schedule_options(&self) -> terrars::ListRef<BigqueryDataTransferConfig_ScheduleOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sensitive_params` after provisioning.\n"]
    pub fn sensitive_params(&self) -> terrars::ListRef<BigqueryDataTransferConfig_SensitiveParamsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitive_params", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryDataTransferConfig_TimeoutsRef {
        BigqueryDataTransferConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BigqueryDataTransferConfigDynamic {
    email_preferences: std::option::Option<terrars::DynamicBlock<BigqueryDataTransferConfig_EmailPreferences>>,
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryDataTransferConfig_EncryptionConfiguration>>,
    schedule_options: std::option::Option<terrars::DynamicBlock<BigqueryDataTransferConfig_ScheduleOptions>>,
    sensitive_params: std::option::Option<terrars::DynamicBlock<BigqueryDataTransferConfig_SensitiveParams>>,
}

#[derive(Serialize)]
pub struct BigqueryDataTransferConfig_EmailPreferences {
    enable_failure_email: terrars::PrimField<bool>,
}

impl BigqueryDataTransferConfig_EmailPreferences { }

impl terrars::ToListMappable for BigqueryDataTransferConfig_EmailPreferences {
    type O = terrars::BlockAssignable<BigqueryDataTransferConfig_EmailPreferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDataTransferConfig_EmailPreferences {
    #[doc = "If true, email notifications will be sent on transfer run failures."]
    pub enable_failure_email: terrars::PrimField<bool>,
}

impl BuildBigqueryDataTransferConfig_EmailPreferences {
    pub fn build(self) -> BigqueryDataTransferConfig_EmailPreferences {
        BigqueryDataTransferConfig_EmailPreferences { enable_failure_email: self.enable_failure_email }
    }
}

#[allow(dead_code)]
pub struct BigqueryDataTransferConfig_EmailPreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDataTransferConfig_EmailPreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryDataTransferConfig_EmailPreferencesRef {
        BigqueryDataTransferConfig_EmailPreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDataTransferConfig_EmailPreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_failure_email` after provisioning.\nIf true, email notifications will be sent on transfer run failures."]
    pub fn enable_failure_email(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_failure_email", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryDataTransferConfig_EncryptionConfiguration {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl BigqueryDataTransferConfig_EncryptionConfiguration { }

impl terrars::ToListMappable for BigqueryDataTransferConfig_EncryptionConfiguration {
    type O = terrars::BlockAssignable<BigqueryDataTransferConfig_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDataTransferConfig_EncryptionConfiguration {
    #[doc = "The name of the KMS key used for encrypting BigQuery data."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryDataTransferConfig_EncryptionConfiguration {
    pub fn build(self) -> BigqueryDataTransferConfig_EncryptionConfiguration {
        BigqueryDataTransferConfig_EncryptionConfiguration { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct BigqueryDataTransferConfig_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDataTransferConfig_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryDataTransferConfig_EncryptionConfigurationRef {
        BigqueryDataTransferConfig_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDataTransferConfig_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe name of the KMS key used for encrypting BigQuery data."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryDataTransferConfig_ScheduleOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_auto_scheduling: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryDataTransferConfig_ScheduleOptions {
    #[doc =
        "Set the field `disable_auto_scheduling`.\nIf true, automatic scheduling of data transfer runs for this\nconfiguration will be disabled. The runs can be started on ad-hoc\nbasis using transferConfigs.startManualRuns API. When automatic\nscheduling is disabled, the TransferConfig.schedule field will\nbe ignored."]
    pub fn set_disable_auto_scheduling(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_auto_scheduling = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `end_time`.\nDefines time to stop scheduling transfer runs. A transfer run cannot be\nscheduled at or after the end time. The end time can be changed at any\nmoment. The time when a data transfer can be triggered manually is not\nlimited by this option."]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `start_time`.\nSpecifies time to start scheduling transfer runs. The first run will be\nscheduled at or after the start time according to a recurrence pattern\ndefined in the schedule string. The start time can be changed at any\nmoment. The time when a data transfer can be triggered manually is not\nlimited by this option."]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryDataTransferConfig_ScheduleOptions {
    type O = terrars::BlockAssignable<BigqueryDataTransferConfig_ScheduleOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDataTransferConfig_ScheduleOptions {}

impl BuildBigqueryDataTransferConfig_ScheduleOptions {
    pub fn build(self) -> BigqueryDataTransferConfig_ScheduleOptions {
        BigqueryDataTransferConfig_ScheduleOptions {
            disable_auto_scheduling: core::default::Default::default(),
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryDataTransferConfig_ScheduleOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDataTransferConfig_ScheduleOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryDataTransferConfig_ScheduleOptionsRef {
        BigqueryDataTransferConfig_ScheduleOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDataTransferConfig_ScheduleOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_auto_scheduling` after provisioning.\nIf true, automatic scheduling of data transfer runs for this\nconfiguration will be disabled. The runs can be started on ad-hoc\nbasis using transferConfigs.startManualRuns API. When automatic\nscheduling is disabled, the TransferConfig.schedule field will\nbe ignored."]
    pub fn disable_auto_scheduling(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_auto_scheduling", self.base))
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nDefines time to stop scheduling transfer runs. A transfer run cannot be\nscheduled at or after the end time. The end time can be changed at any\nmoment. The time when a data transfer can be triggered manually is not\nlimited by this option."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nSpecifies time to start scheduling transfer runs. The first run will be\nscheduled at or after the start time according to a recurrence pattern\ndefined in the schedule string. The start time can be changed at any\nmoment. The time when a data transfer can be triggered manually is not\nlimited by this option."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryDataTransferConfig_SensitiveParams {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_access_key_wo_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryDataTransferConfig_SensitiveParams {
    #[doc = "Set the field `secret_access_key`.\nThe Secret Access Key of the AWS account transferring data from."]
    pub fn set_secret_access_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_access_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_access_key_wo`.\nThe Secret Access Key of the AWS account transferring data from."]
    pub fn set_secret_access_key_wo(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_access_key_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `secret_access_key_wo_version`.\nTriggers update of 'secret_access_key_wo' write-only. Increment this value when an update to 'secret_access_key_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn set_secret_access_key_wo_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secret_access_key_wo_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryDataTransferConfig_SensitiveParams {
    type O = terrars::BlockAssignable<BigqueryDataTransferConfig_SensitiveParams>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDataTransferConfig_SensitiveParams {}

impl BuildBigqueryDataTransferConfig_SensitiveParams {
    pub fn build(self) -> BigqueryDataTransferConfig_SensitiveParams {
        BigqueryDataTransferConfig_SensitiveParams {
            secret_access_key: core::default::Default::default(),
            secret_access_key_wo: core::default::Default::default(),
            secret_access_key_wo_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryDataTransferConfig_SensitiveParamsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDataTransferConfig_SensitiveParamsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryDataTransferConfig_SensitiveParamsRef {
        BigqueryDataTransferConfig_SensitiveParamsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDataTransferConfig_SensitiveParamsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key` after provisioning.\nThe Secret Access Key of the AWS account transferring data from."]
    pub fn secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key_wo` after provisioning.\nThe Secret Access Key of the AWS account transferring data from."]
    pub fn secret_access_key_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key_wo", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_access_key_wo_version` after provisioning.\nTriggers update of 'secret_access_key_wo' write-only. Increment this value when an update to 'secret_access_key_wo' is needed. For more info see [updating write-only arguments](/docs/providers/google/guides/using_write_only_arguments.html#updating-write-only-arguments)"]
    pub fn secret_access_key_wo_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_access_key_wo_version", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryDataTransferConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryDataTransferConfig_Timeouts {
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

impl terrars::ToListMappable for BigqueryDataTransferConfig_Timeouts {
    type O = terrars::BlockAssignable<BigqueryDataTransferConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryDataTransferConfig_Timeouts {}

impl BuildBigqueryDataTransferConfig_Timeouts {
    pub fn build(self) -> BigqueryDataTransferConfig_Timeouts {
        BigqueryDataTransferConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryDataTransferConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryDataTransferConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryDataTransferConfig_TimeoutsRef {
        BigqueryDataTransferConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryDataTransferConfig_TimeoutsRef {
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
