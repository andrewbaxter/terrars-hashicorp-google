#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryAnalyticsHubListingData_ {
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
    allow_only_metadata_sharing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    categories: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    data_exchange_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_commercial: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    discovery_type: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documentation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icon: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    listing_id: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_linked_dataset_query_user_email: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_contact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_dataset: std::option::Option<std::vec::Vec<BigqueryAnalyticsHubListing_BigqueryDataset>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_provider: std::option::Option<std::vec::Vec<BigqueryAnalyticsHubListing_DataProvider>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publisher: std::option::Option<std::vec::Vec<BigqueryAnalyticsHubListing_Publisher>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_topic: std::option::Option<std::vec::Vec<BigqueryAnalyticsHubListing_PubsubTopic>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restricted_export_config: std::option::Option<std::vec::Vec<BigqueryAnalyticsHubListing_RestrictedExportConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigqueryAnalyticsHubListing_Timeouts>,
    dynamic: BigqueryAnalyticsHubListingDynamic,
}

struct BigqueryAnalyticsHubListing_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryAnalyticsHubListingData_>,
}

#[derive(Clone)]
pub struct BigqueryAnalyticsHubListing(std::rc::Rc<BigqueryAnalyticsHubListing_>);

impl BigqueryAnalyticsHubListing {
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
        "Set the field `allow_only_metadata_sharing`.\nIf true, the listing is only available to get the resource metadata. Listing is non subscribable."]
    pub fn set_allow_only_metadata_sharing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().allow_only_metadata_sharing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `categories`.\nCategories of the listing. Up to two categories are allowed."]
    pub fn set_categories(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().categories = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete_commercial`.\nIf the listing is commercial then this field must be set to true, otherwise a failure is thrown. This acts as a safety guard to avoid deleting commercial listings accidentally."]
    pub fn set_delete_commercial(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_commercial = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `description`.\nShort description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF)."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `discovery_type`.\nSpecifies the type of discovery on the discovery page. Cannot be set for a restricted listing. Note that this does not control the visibility of the exchange/listing which is defined by IAM permission. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn set_discovery_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().discovery_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documentation`.\nDocumentation describing the listing."]
    pub fn set_documentation(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().documentation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icon`.\nBase64 encoded image representing the listing."]
    pub fn set_icon(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().icon = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `log_linked_dataset_query_user_email`.\nIf true, subscriber email logging is enabled and all queries on the linked dataset will log the email address of the querying user. Once enabled, this setting cannot be turned off."]
    pub fn set_log_linked_dataset_query_user_email(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().log_linked_dataset_query_user_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_contact`.\nEmail or URL of the primary point of contact of the listing."]
    pub fn set_primary_contact(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().primary_contact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `request_access`.\nEmail or URL of the request access of the listing. Subscribers can use this reference to request access."]
    pub fn set_request_access(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().request_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigquery_dataset`.\n"]
    pub fn set_bigquery_dataset(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryAnalyticsHubListing_BigqueryDataset>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigquery_dataset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigquery_dataset = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_provider`.\n"]
    pub fn set_data_provider(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryAnalyticsHubListing_DataProvider>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_provider = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `publisher`.\n"]
    pub fn set_publisher(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryAnalyticsHubListing_Publisher>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().publisher = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.publisher = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pubsub_topic`.\n"]
    pub fn set_pubsub_topic(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryAnalyticsHubListing_PubsubTopic>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pubsub_topic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pubsub_topic = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `restricted_export_config`.\n"]
    pub fn set_restricted_export_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryAnalyticsHubListing_RestrictedExportConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().restricted_export_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.restricted_export_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigqueryAnalyticsHubListing_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_only_metadata_sharing` after provisioning.\nIf true, the listing is only available to get the resource metadata. Listing is non subscribable."]
    pub fn allow_only_metadata_sharing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_only_metadata_sharing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `categories` after provisioning.\nCategories of the listing. Up to two categories are allowed."]
    pub fn categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categories", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `commercial_info` after provisioning.\nCommercial info contains the information about the commercial data products associated with the listing."]
    pub fn commercial_info(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_CommercialInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.commercial_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_exchange_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_commercial` after provisioning.\nIf the listing is commercial then this field must be set to true, otherwise a failure is thrown. This acts as a safety guard to avoid deleting commercial listings accidentally."]
    pub fn delete_commercial(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_commercial", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nShort description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF)."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_type` after provisioning.\nSpecifies the type of discovery on the discovery page. Cannot be set for a restricted listing. Note that this does not control the visibility of the exchange/listing which is defined by IAM permission. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn discovery_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the listing."]
    pub fn documentation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the listing."]
    pub fn icon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icon", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listing_id` after provisioning.\nThe ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn listing_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listing_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange listing."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_linked_dataset_query_user_email` after provisioning.\nIf true, subscriber email logging is enabled and all queries on the linked dataset will log the email address of the querying user. Once enabled, this setting cannot be turned off."]
    pub fn log_linked_dataset_query_user_email(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_linked_dataset_query_user_email", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the listing. e.g. \"projects/myproject/locations/US/dataExchanges/123/listings/456\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the listing."]
    pub fn primary_contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `request_access` after provisioning.\nEmail or URL of the request access of the listing. Subscribers can use this reference to request access."]
    pub fn request_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the listing."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigquery_dataset` after provisioning.\n"]
    pub fn bigquery_dataset(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_BigqueryDatasetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_dataset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_provider` after provisioning.\n"]
    pub fn data_provider(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_DataProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publisher` after provisioning.\n"]
    pub fn publisher(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_PublisherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publisher", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\n"]
    pub fn pubsub_topic(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_PubsubTopicRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_topic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restricted_export_config` after provisioning.\n"]
    pub fn restricted_export_config(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_RestrictedExportConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restricted_export_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubListing_TimeoutsRef {
        BigqueryAnalyticsHubListing_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BigqueryAnalyticsHubListing {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigqueryAnalyticsHubListing { }

impl terrars::ToListMappable for BigqueryAnalyticsHubListing {
    type O = terrars::ListRef<BigqueryAnalyticsHubListingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigqueryAnalyticsHubListing_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigquery_analytics_hub_listing".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryAnalyticsHubListing {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub data_exchange_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Human-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub listing_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the location this data exchange listing."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryAnalyticsHubListing {
    pub fn build(self, stack: &mut terrars::Stack) -> BigqueryAnalyticsHubListing {
        let out = BigqueryAnalyticsHubListing(std::rc::Rc::new(BigqueryAnalyticsHubListing_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigqueryAnalyticsHubListingData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_only_metadata_sharing: core::default::Default::default(),
                categories: core::default::Default::default(),
                data_exchange_id: self.data_exchange_id,
                delete_commercial: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                discovery_type: core::default::Default::default(),
                display_name: self.display_name,
                documentation: core::default::Default::default(),
                icon: core::default::Default::default(),
                id: core::default::Default::default(),
                listing_id: self.listing_id,
                location: self.location,
                log_linked_dataset_query_user_email: core::default::Default::default(),
                primary_contact: core::default::Default::default(),
                project: core::default::Default::default(),
                request_access: core::default::Default::default(),
                bigquery_dataset: core::default::Default::default(),
                data_provider: core::default::Default::default(),
                publisher: core::default::Default::default(),
                pubsub_topic: core::default::Default::default(),
                restricted_export_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryAnalyticsHubListingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryAnalyticsHubListingRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_only_metadata_sharing` after provisioning.\nIf true, the listing is only available to get the resource metadata. Listing is non subscribable."]
    pub fn allow_only_metadata_sharing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_only_metadata_sharing", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `categories` after provisioning.\nCategories of the listing. Up to two categories are allowed."]
    pub fn categories(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categories", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `commercial_info` after provisioning.\nCommercial info contains the information about the commercial data products associated with the listing."]
    pub fn commercial_info(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_CommercialInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.commercial_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_exchange_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_commercial` after provisioning.\nIf the listing is commercial then this field must be set to true, otherwise a failure is thrown. This acts as a safety guard to avoid deleting commercial listings accidentally."]
    pub fn delete_commercial(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_commercial", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nShort description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF)."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_type` after provisioning.\nSpecifies the type of discovery on the discovery page. Cannot be set for a restricted listing. Note that this does not control the visibility of the exchange/listing which is defined by IAM permission. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn discovery_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the listing."]
    pub fn documentation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the listing."]
    pub fn icon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icon", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listing_id` after provisioning.\nThe ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn listing_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listing_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange listing."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_linked_dataset_query_user_email` after provisioning.\nIf true, subscriber email logging is enabled and all queries on the linked dataset will log the email address of the querying user. Once enabled, this setting cannot be turned off."]
    pub fn log_linked_dataset_query_user_email(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_linked_dataset_query_user_email", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the listing. e.g. \"projects/myproject/locations/US/dataExchanges/123/listings/456\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the listing."]
    pub fn primary_contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `request_access` after provisioning.\nEmail or URL of the request access of the listing. Subscribers can use this reference to request access."]
    pub fn request_access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_access", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the listing."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bigquery_dataset` after provisioning.\n"]
    pub fn bigquery_dataset(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_BigqueryDatasetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_dataset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_provider` after provisioning.\n"]
    pub fn data_provider(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_DataProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publisher` after provisioning.\n"]
    pub fn publisher(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_PublisherRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publisher", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pubsub_topic` after provisioning.\n"]
    pub fn pubsub_topic(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_PubsubTopicRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_topic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restricted_export_config` after provisioning.\n"]
    pub fn restricted_export_config(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_RestrictedExportConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.restricted_export_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubListing_TimeoutsRef {
        BigqueryAnalyticsHubListing_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubListingDynamic {
    bigquery_dataset: std::option::Option<terrars::DynamicBlock<BigqueryAnalyticsHubListing_BigqueryDataset>>,
    data_provider: std::option::Option<terrars::DynamicBlock<BigqueryAnalyticsHubListing_DataProvider>>,
    publisher: std::option::Option<terrars::DynamicBlock<BigqueryAnalyticsHubListing_Publisher>>,
    pubsub_topic: std::option::Option<terrars::DynamicBlock<BigqueryAnalyticsHubListing_PubsubTopic>>,
    restricted_export_config: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryAnalyticsHubListing_RestrictedExportConfig>>,
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplace {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commercial_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplace {
    #[doc = "Set the field `commercial_state`.\n"]
    pub fn set_commercial_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commercial_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplace {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplace>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplace {}

impl BuildBigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplace {
    pub fn build(self) -> BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplace {
        BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplace {
            commercial_state: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplaceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplaceRef {
        BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `commercial_state` after provisioning.\n"]
    pub fn commercial_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commercial_state", self.base))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListing_CommercialInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_marketplace: std
    ::option
    ::Option<terrars::ListField<BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplace>>,
}

impl BigqueryAnalyticsHubListing_CommercialInfo {
    #[doc = "Set the field `cloud_marketplace`.\n"]
    pub fn set_cloud_marketplace(
        mut self,
        v: impl std::convert::Into<terrars::ListField<BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplace>>,
    ) -> Self {
        self.cloud_marketplace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListing_CommercialInfo {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListing_CommercialInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListing_CommercialInfo {}

impl BuildBigqueryAnalyticsHubListing_CommercialInfo {
    pub fn build(self) -> BigqueryAnalyticsHubListing_CommercialInfo {
        BigqueryAnalyticsHubListing_CommercialInfo { cloud_marketplace: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListing_CommercialInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListing_CommercialInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryAnalyticsHubListing_CommercialInfoRef {
        BigqueryAnalyticsHubListing_CommercialInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListing_CommercialInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_marketplace` after provisioning.\n"]
    pub fn cloud_marketplace(&self) -> terrars::ListRef<BigqueryAnalyticsHubListing_CommercialInfo_CloudMarketplaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_marketplace", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicas {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicas {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_state`.\n"]
    pub fn set_primary_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.primary_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_state`.\n"]
    pub fn set_replica_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.replica_state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicas {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicas {}

impl BuildBigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicas {
    pub fn build(self) -> BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicas {
        BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicas {
            location: core::default::Default::default(),
            primary_state: core::default::Default::default(),
            replica_state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicasRef {
        BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_state` after provisioning.\n"]
    pub fn primary_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_state", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_state` after provisioning.\n"]
    pub fn replica_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_state", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResources {
    #[doc =
        "Set the field `routine`.\nFormat: For routine: projects/{projectId}/datasets/{datasetId}/routines/{routineId} Example:\"projects/test_project/datasets/test_dataset/routines/test_routine\""]
    pub fn set_routine(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.routine = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `table`.\nFormat: For table: projects/{projectId}/datasets/{datasetId}/tables/{tableId} Example:\"projects/test_project/datasets/test_dataset/tables/test_table\""]
    pub fn set_table(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResources {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListing_BigqueryDataset_SelectedResources {}

impl BuildBigqueryAnalyticsHubListing_BigqueryDataset_SelectedResources {
    pub fn build(self) -> BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResources {
        BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResources {
            routine: core::default::Default::default(),
            table: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResourcesRef {
        BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `routine` after provisioning.\nFormat: For routine: projects/{projectId}/datasets/{datasetId}/routines/{routineId} Example:\"projects/test_project/datasets/test_dataset/routines/test_routine\""]
    pub fn routine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routine", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table` after provisioning.\nFormat: For table: projects/{projectId}/datasets/{datasetId}/tables/{tableId} Example:\"projects/test_project/datasets/test_dataset/tables/test_table\""]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubListing_BigqueryDatasetDynamic {
    selected_resources: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResources>>,
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListing_BigqueryDataset {
    dataset: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selected_resources: std::option::Option<std::vec::Vec<BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResources>>,
    dynamic: BigqueryAnalyticsHubListing_BigqueryDatasetDynamic,
}

impl BigqueryAnalyticsHubListing_BigqueryDataset {
    #[doc =
        "Set the field `replica_locations`.\nA list of regions where the publisher has created shared dataset replicas."]
    pub fn set_replica_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.replica_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `selected_resources`.\n"]
    pub fn set_selected_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selected_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selected_resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListing_BigqueryDataset {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListing_BigqueryDataset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListing_BigqueryDataset {
    #[doc = "Resource name of the dataset source for this listing. e.g. projects/myproject/datasets/123"]
    pub dataset: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryAnalyticsHubListing_BigqueryDataset {
    pub fn build(self) -> BigqueryAnalyticsHubListing_BigqueryDataset {
        BigqueryAnalyticsHubListing_BigqueryDataset {
            dataset: self.dataset,
            replica_locations: core::default::Default::default(),
            selected_resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListing_BigqueryDatasetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListing_BigqueryDatasetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListing_BigqueryDatasetRef {
        BigqueryAnalyticsHubListing_BigqueryDatasetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListing_BigqueryDatasetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset` after provisioning.\nResource name of the dataset source for this listing. e.g. projects/myproject/datasets/123"]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc =
        "Get a reference to the value of field `effective_replicas` after provisioning.\nServer owned effective state of replicas. Contains both primary and secondary replicas.\nEach replica includes a system-computed (output-only) state and primary designation."]
    pub fn effective_replicas(
        &self,
    ) -> terrars::ListRef<BigqueryAnalyticsHubListing_BigqueryDataset_EffectiveReplicasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.effective_replicas", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replica_locations` after provisioning.\nA list of regions where the publisher has created shared dataset replicas."]
    pub fn replica_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.replica_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `selected_resources` after provisioning.\n"]
    pub fn selected_resources(
        &self,
    ) -> terrars::ListRef<BigqueryAnalyticsHubListing_BigqueryDataset_SelectedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selected_resources", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListing_DataProvider {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_contact: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListing_DataProvider {
    #[doc = "Set the field `primary_contact`.\nEmail or URL of the data provider."]
    pub fn set_primary_contact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.primary_contact = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListing_DataProvider {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListing_DataProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListing_DataProvider {
    #[doc = "Name of the data provider."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryAnalyticsHubListing_DataProvider {
    pub fn build(self) -> BigqueryAnalyticsHubListing_DataProvider {
        BigqueryAnalyticsHubListing_DataProvider {
            name: self.name,
            primary_contact: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListing_DataProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListing_DataProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryAnalyticsHubListing_DataProviderRef {
        BigqueryAnalyticsHubListing_DataProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListing_DataProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the data provider."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the data provider."]
    pub fn primary_contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListing_Publisher {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_contact: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListing_Publisher {
    #[doc = "Set the field `primary_contact`.\nEmail or URL of the listing publisher."]
    pub fn set_primary_contact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.primary_contact = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListing_Publisher {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListing_Publisher>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListing_Publisher {
    #[doc = "Name of the listing publisher."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryAnalyticsHubListing_Publisher {
    pub fn build(self) -> BigqueryAnalyticsHubListing_Publisher {
        BigqueryAnalyticsHubListing_Publisher {
            name: self.name,
            primary_contact: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListing_PublisherRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListing_PublisherRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryAnalyticsHubListing_PublisherRef {
        BigqueryAnalyticsHubListing_PublisherRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListing_PublisherRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the listing publisher."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the listing publisher."]
    pub fn primary_contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListing_PubsubTopic {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_affinity_regions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    topic: terrars::PrimField<std::string::String>,
}

impl BigqueryAnalyticsHubListing_PubsubTopic {
    #[doc =
        "Set the field `data_affinity_regions`.\nRegion hint on where the data might be published. Data affinity regions are modifiable.\nSee https://cloud.google.com/about/locations for full listing of possible Cloud regions."]
    pub fn set_data_affinity_regions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data_affinity_regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListing_PubsubTopic {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListing_PubsubTopic>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListing_PubsubTopic {
    #[doc = "Resource name of the Pub/Sub topic source for this listing. e.g. projects/myproject/topics/topicId"]
    pub topic: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryAnalyticsHubListing_PubsubTopic {
    pub fn build(self) -> BigqueryAnalyticsHubListing_PubsubTopic {
        BigqueryAnalyticsHubListing_PubsubTopic {
            data_affinity_regions: core::default::Default::default(),
            topic: self.topic,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListing_PubsubTopicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListing_PubsubTopicRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryAnalyticsHubListing_PubsubTopicRef {
        BigqueryAnalyticsHubListing_PubsubTopicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListing_PubsubTopicRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_affinity_regions` after provisioning.\nRegion hint on where the data might be published. Data affinity regions are modifiable.\nSee https://cloud.google.com/about/locations for full listing of possible Cloud regions."]
    pub fn data_affinity_regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.data_affinity_regions", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nResource name of the Pub/Sub topic source for this listing. e.g. projects/myproject/topics/topicId"]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListing_RestrictedExportConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restrict_query_result: std::option::Option<terrars::PrimField<bool>>,
}

impl BigqueryAnalyticsHubListing_RestrictedExportConfig {
    #[doc = "Set the field `enabled`.\nIf true, enable restricted export."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `restrict_query_result`.\nIf true, restrict export of query result derived from restricted linked dataset table."]
    pub fn set_restrict_query_result(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.restrict_query_result = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListing_RestrictedExportConfig {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListing_RestrictedExportConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListing_RestrictedExportConfig {}

impl BuildBigqueryAnalyticsHubListing_RestrictedExportConfig {
    pub fn build(self) -> BigqueryAnalyticsHubListing_RestrictedExportConfig {
        BigqueryAnalyticsHubListing_RestrictedExportConfig {
            enabled: core::default::Default::default(),
            restrict_query_result: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListing_RestrictedExportConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListing_RestrictedExportConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListing_RestrictedExportConfigRef {
        BigqueryAnalyticsHubListing_RestrictedExportConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListing_RestrictedExportConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\nIf true, enable restricted export."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restrict_direct_table_access` after provisioning.\nIf true, restrict direct table access(read api/tabledata.list) on linked table."]
    pub fn restrict_direct_table_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restrict_direct_table_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `restrict_query_result` after provisioning.\nIf true, restrict export of query result derived from restricted linked dataset table."]
    pub fn restrict_query_result(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restrict_query_result", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListing_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListing_Timeouts {
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

impl terrars::ToListMappable for BigqueryAnalyticsHubListing_Timeouts {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListing_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListing_Timeouts {}

impl BuildBigqueryAnalyticsHubListing_Timeouts {
    pub fn build(self) -> BigqueryAnalyticsHubListing_Timeouts {
        BigqueryAnalyticsHubListing_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListing_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListing_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryAnalyticsHubListing_TimeoutsRef {
        BigqueryAnalyticsHubListing_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListing_TimeoutsRef {
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
