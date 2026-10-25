#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryAnalyticsHubListingSubscriptionData_ {
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
    data_exchange_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    listing_id: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_dataset: std::option::Option<std::vec::Vec<BigqueryAnalyticsHubListingSubscription_DestinationDataset>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_pubsub_subscription: std
    ::option
    ::Option<std::vec::Vec<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigqueryAnalyticsHubListingSubscription_Timeouts>,
    dynamic: BigqueryAnalyticsHubListingSubscriptionDynamic,
}

struct BigqueryAnalyticsHubListingSubscription_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryAnalyticsHubListingSubscriptionData_>,
}

#[derive(Clone)]
pub struct BigqueryAnalyticsHubListingSubscription(std::rc::Rc<BigqueryAnalyticsHubListingSubscription_>);

impl BigqueryAnalyticsHubListingSubscription {
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

    #[doc = "Set the field `destination_dataset`.\n"]
    pub fn set_destination_dataset(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BigqueryAnalyticsHubListingSubscription_DestinationDataset>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_dataset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_dataset = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_pubsub_subscription`.\n"]
    pub fn set_destination_pubsub_subscription(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().destination_pubsub_subscription = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.destination_pubsub_subscription = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigqueryAnalyticsHubListingSubscription_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `commercial_info` after provisioning.\nCommercial info metadata for this subscription. This is set if this is a commercial subscription i.e. if this subscription was created from subscribing to a commercial listing."]
    pub fn commercial_info(&self) -> terrars::ListRef<BigqueryAnalyticsHubListingSubscription_CommercialInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.commercial_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nTimestamp when the subscription was created."]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_exchange_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modify_time` after provisioning.\nTimestamp when the subscription was last modified."]
    pub fn last_modify_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modify_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `linked_dataset_map` after provisioning.\nOutput only. Map of listing resource names to associated linked resource,\ne.g. projects/123/locations/US/dataExchanges/456/listings/789 -> projects/123/datasets/my_dataset"]
    pub fn linked_dataset_map(&self) -> terrars::SetRef<BigqueryAnalyticsHubListingSubscription_LinkedDatasetMapRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.linked_dataset_map", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `linked_resources` after provisioning.\nOutput only. Linked resources created in the subscription. Only contains values if state = STATE_ACTIVE."]
    pub fn linked_resources(&self) -> terrars::ListRef<BigqueryAnalyticsHubListingSubscription_LinkedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_resources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listing_id` after provisioning.\nThe ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn listing_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listing_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the data exchange. Distinct from the location of the destination data set."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_linked_dataset_query_user_email` after provisioning.\nOutput only. By default, false. If true, the Subscriber agreed to the email sharing mandate that is enabled for Listing."]
    pub fn log_linked_dataset_query_user_email(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_linked_dataset_query_user_email", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the subscription. e.g. \"projects/myproject/locations/US/subscriptions/123\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_display_name` after provisioning.\nDisplay name of the project of this subscription."]
    pub fn organization_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_id` after provisioning.\nOrganization of the project this subscription belongs to."]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nListing shared asset type."]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the subscription."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_contact` after provisioning.\nEmail of the subscriber."]
    pub fn subscriber_contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_contact", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_id` after provisioning.\nThe subscription id used to reference the subscription."]
    pub fn subscription_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_dataset` after provisioning.\n"]
    pub fn destination_dataset(&self) -> terrars::ListRef<BigqueryAnalyticsHubListingSubscription_DestinationDatasetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_dataset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_pubsub_subscription` after provisioning.\n"]
    pub fn destination_pubsub_subscription(
        &self,
    ) -> terrars::ListRef<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscriptionRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.destination_pubsub_subscription", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubListingSubscription_TimeoutsRef {
        BigqueryAnalyticsHubListingSubscription_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BigqueryAnalyticsHubListingSubscription {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigqueryAnalyticsHubListingSubscription { }

impl terrars::ToListMappable for BigqueryAnalyticsHubListingSubscription {
    type O = terrars::ListRef<BigqueryAnalyticsHubListingSubscriptionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigqueryAnalyticsHubListingSubscription_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigquery_analytics_hub_listing_subscription".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub data_exchange_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub listing_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the location of the data exchange. Distinct from the location of the destination data set."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryAnalyticsHubListingSubscription {
    pub fn build(self, stack: &mut terrars::Stack) -> BigqueryAnalyticsHubListingSubscription {
        let out = BigqueryAnalyticsHubListingSubscription(std::rc::Rc::new(BigqueryAnalyticsHubListingSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigqueryAnalyticsHubListingSubscriptionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_exchange_id: self.data_exchange_id,
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                listing_id: self.listing_id,
                location: self.location,
                project: core::default::Default::default(),
                destination_dataset: core::default::Default::default(),
                destination_pubsub_subscription: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryAnalyticsHubListingSubscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListingSubscriptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryAnalyticsHubListingSubscriptionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `commercial_info` after provisioning.\nCommercial info metadata for this subscription. This is set if this is a commercial subscription i.e. if this subscription was created from subscribing to a commercial listing."]
    pub fn commercial_info(&self) -> terrars::ListRef<BigqueryAnalyticsHubListingSubscription_CommercialInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.commercial_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nTimestamp when the subscription was created."]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_exchange_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modify_time` after provisioning.\nTimestamp when the subscription was last modified."]
    pub fn last_modify_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modify_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `linked_dataset_map` after provisioning.\nOutput only. Map of listing resource names to associated linked resource,\ne.g. projects/123/locations/US/dataExchanges/456/listings/789 -> projects/123/datasets/my_dataset"]
    pub fn linked_dataset_map(&self) -> terrars::SetRef<BigqueryAnalyticsHubListingSubscription_LinkedDatasetMapRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.linked_dataset_map", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `linked_resources` after provisioning.\nOutput only. Linked resources created in the subscription. Only contains values if state = STATE_ACTIVE."]
    pub fn linked_resources(&self) -> terrars::ListRef<BigqueryAnalyticsHubListingSubscription_LinkedResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linked_resources", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listing_id` after provisioning.\nThe ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn listing_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listing_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location of the data exchange. Distinct from the location of the destination data set."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_linked_dataset_query_user_email` after provisioning.\nOutput only. By default, false. If true, the Subscriber agreed to the email sharing mandate that is enabled for Listing."]
    pub fn log_linked_dataset_query_user_email(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_linked_dataset_query_user_email", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the subscription. e.g. \"projects/myproject/locations/US/subscriptions/123\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_display_name` after provisioning.\nDisplay name of the project of this subscription."]
    pub fn organization_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `organization_id` after provisioning.\nOrganization of the project this subscription belongs to."]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\nListing shared asset type."]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nCurrent state of the subscription."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_contact` after provisioning.\nEmail of the subscriber."]
    pub fn subscriber_contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_contact", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `subscription_id` after provisioning.\nThe subscription id used to reference the subscription."]
    pub fn subscription_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_dataset` after provisioning.\n"]
    pub fn destination_dataset(&self) -> terrars::ListRef<BigqueryAnalyticsHubListingSubscription_DestinationDatasetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_dataset", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_pubsub_subscription` after provisioning.\n"]
    pub fn destination_pubsub_subscription(
        &self,
    ) -> terrars::ListRef<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscriptionRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.destination_pubsub_subscription", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubListingSubscription_TimeoutsRef {
        BigqueryAnalyticsHubListingSubscription_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubListingSubscriptionDynamic {
    destination_dataset: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryAnalyticsHubListingSubscription_DestinationDataset>>,
    destination_pubsub_subscription: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription>>,
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplace {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplace {
    #[doc = "Set the field `order`.\n"]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplace {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplace>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplace {}

impl BuildBigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplace {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplace {
        BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplace {
            order: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplaceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplaceRef {
        BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_CommercialInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_marketplace: std
    ::option
    ::Option<terrars::ListField<BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplace>>,
}

impl BigqueryAnalyticsHubListingSubscription_CommercialInfo {
    #[doc = "Set the field `cloud_marketplace`.\n"]
    pub fn set_cloud_marketplace(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplace>,
                    >,
    ) -> Self {
        self.cloud_marketplace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListingSubscription_CommercialInfo {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListingSubscription_CommercialInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_CommercialInfo {}

impl BuildBigqueryAnalyticsHubListingSubscription_CommercialInfo {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscription_CommercialInfo {
        BigqueryAnalyticsHubListingSubscription_CommercialInfo {
            cloud_marketplace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_CommercialInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListingSubscription_CommercialInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_CommercialInfoRef {
        BigqueryAnalyticsHubListingSubscription_CommercialInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_CommercialInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_marketplace` after provisioning.\n"]
    pub fn cloud_marketplace(
        &self,
    ) -> terrars::ListRef<BigqueryAnalyticsHubListingSubscription_CommercialInfo_CloudMarketplaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_marketplace", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_LinkedDatasetMap {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_dataset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_pubsub_subscription: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    listing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListingSubscription_LinkedDatasetMap {
    #[doc = "Set the field `linked_dataset`.\n"]
    pub fn set_linked_dataset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.linked_dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked_pubsub_subscription`.\n"]
    pub fn set_linked_pubsub_subscription(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.linked_pubsub_subscription = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `listing`.\n"]
    pub fn set_listing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.listing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_name`.\n"]
    pub fn set_resource_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListingSubscription_LinkedDatasetMap {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListingSubscription_LinkedDatasetMap>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_LinkedDatasetMap {}

impl BuildBigqueryAnalyticsHubListingSubscription_LinkedDatasetMap {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscription_LinkedDatasetMap {
        BigqueryAnalyticsHubListingSubscription_LinkedDatasetMap {
            linked_dataset: core::default::Default::default(),
            linked_pubsub_subscription: core::default::Default::default(),
            listing: core::default::Default::default(),
            resource_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_LinkedDatasetMapRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListingSubscription_LinkedDatasetMapRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_LinkedDatasetMapRef {
        BigqueryAnalyticsHubListingSubscription_LinkedDatasetMapRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_LinkedDatasetMapRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `linked_dataset` after provisioning.\n"]
    pub fn linked_dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.linked_dataset", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_pubsub_subscription` after provisioning.\n"]
    pub fn linked_pubsub_subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.linked_pubsub_subscription", self.base))
    }

    #[doc = "Get a reference to the value of field `listing` after provisioning.\n"]
    pub fn listing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listing", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_name` after provisioning.\n"]
    pub fn resource_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_name", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_LinkedResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_dataset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linked_pubsub_subscription: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    listing: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListingSubscription_LinkedResources {
    #[doc = "Set the field `linked_dataset`.\n"]
    pub fn set_linked_dataset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.linked_dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linked_pubsub_subscription`.\n"]
    pub fn set_linked_pubsub_subscription(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.linked_pubsub_subscription = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `listing`.\n"]
    pub fn set_listing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.listing = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListingSubscription_LinkedResources {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListingSubscription_LinkedResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_LinkedResources {}

impl BuildBigqueryAnalyticsHubListingSubscription_LinkedResources {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscription_LinkedResources {
        BigqueryAnalyticsHubListingSubscription_LinkedResources {
            linked_dataset: core::default::Default::default(),
            linked_pubsub_subscription: core::default::Default::default(),
            listing: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_LinkedResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListingSubscription_LinkedResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_LinkedResourcesRef {
        BigqueryAnalyticsHubListingSubscription_LinkedResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_LinkedResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `linked_dataset` after provisioning.\n"]
    pub fn linked_dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.linked_dataset", self.base))
    }

    #[doc = "Get a reference to the value of field `linked_pubsub_subscription` after provisioning.\n"]
    pub fn linked_pubsub_subscription(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.linked_pubsub_subscription", self.base))
    }

    #[doc = "Get a reference to the value of field `listing` after provisioning.\n"]
    pub fn listing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listing", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReference {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReference { }

impl terrars::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReference {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReference {
    #[doc =
        "A unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the project containing this dataset."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReference {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReference {
        BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReference {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReferenceRef {
        BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nA unique ID for this dataset, without the project name. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 1,024 characters."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe ID of the project containing this dataset."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubListingSubscription_DestinationDatasetDynamic {
    dataset_reference: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReference>>,
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationDataset {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    friendly_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_locations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_reference: std
    ::option
    ::Option<std::vec::Vec<BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReference>>,
    dynamic: BigqueryAnalyticsHubListingSubscription_DestinationDatasetDynamic,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationDataset {
    #[doc = "Set the field `description`.\nA user-friendly description of the dataset."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `friendly_name`.\nA descriptive name for the dataset."]
    pub fn set_friendly_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.friendly_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nThe labels associated with this dataset. You can use these to\norganize and group your datasets."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_locations`.\nList of regions where the subscriber wants dataset replicas."]
    pub fn set_replica_locations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.replica_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dataset_reference`.\n"]
    pub fn set_dataset_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReference>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dataset_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dataset_reference = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationDataset {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListingSubscription_DestinationDataset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationDataset {
    #[doc =
        "The geographic location where the dataset should reside.\nSee https://cloud.google.com/bigquery/docs/locations for supported locations."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationDataset {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscription_DestinationDataset {
        BigqueryAnalyticsHubListingSubscription_DestinationDataset {
            description: core::default::Default::default(),
            friendly_name: core::default::Default::default(),
            labels: core::default::Default::default(),
            location: self.location,
            replica_locations: core::default::Default::default(),
            dataset_reference: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationDatasetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListingSubscription_DestinationDatasetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationDatasetRef {
        BigqueryAnalyticsHubListingSubscription_DestinationDatasetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationDatasetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA user-friendly description of the dataset."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `friendly_name` after provisioning.\nA descriptive name for the dataset."]
    pub fn friendly_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.friendly_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels associated with this dataset. You can use these to\norganize and group your datasets."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the dataset should reside.\nSee https://cloud.google.com/bigquery/docs/locations for supported locations."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `replica_locations` after provisioning.\nList of regions where the subscriber wants dataset replicas."]
    pub fn replica_locations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.replica_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `dataset_reference` after provisioning.\n"]
    pub fn dataset_reference(
        &self,
    ) -> terrars::ListRef<BigqueryAnalyticsHubListingSubscription_DestinationDataset_DatasetReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dataset_reference", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop_unknown_fields: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_table_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_topic_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfig {
    #[doc =
        "Set the field `drop_unknown_fields`.\nWhen true and 'useTopicSchema' is true, any fields that are a part of the topic schema that are\nnot part of the BigQuery table schema are dropped when writing to BigQuery. Otherwise, the schemas\nmust be kept in sync and any messages with extra fields are not written and remain in the\nsubscription's backlog."]
    pub fn set_drop_unknown_fields(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.drop_unknown_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_email`.\nThe service account to use to write to BigQuery. The subscription creator or updater that\nspecifies this field must have 'iam.serviceAccounts.actAs' permission on the service account.\nIf not specified, the Pub/Sub service agent,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `table`.\nThe name of the table to which to write data, of the form\n{projectId}.{datasetId}.{tableId}"]
    pub fn set_table(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_table_schema`.\nWhen true, use the BigQuery table's schema as the columns to write to in BigQuery.\n'useTableSchema' and 'useTopicSchema' cannot be enabled at the same time."]
    pub fn set_use_table_schema(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_table_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `use_topic_schema`.\nWhen true, use the topic's schema as the columns to write to in BigQuery,\nif it exists. 'useTopicSchema' and 'useTableSchema' cannot be enabled at the same time."]
    pub fn set_use_topic_schema(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_topic_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `write_metadata`.\nWhen true, write the subscription name, message_id, publish_time, attributes, and ordering_key\nto additional columns in the table. The subscription name, message_id, and publish_time fields\nare put in their own columns while all other message properties (other than data) are written\nto a JSON object in the attributes column."]
    pub fn set_write_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.write_metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfig {
    type O =
        terrars
        ::BlockAssignable<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfig {}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfig {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfig {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfig {
            drop_unknown_fields: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            table: core::default::Default::default(),
            use_table_schema: core::default::Default::default(),
            use_topic_schema: core::default::Default::default(),
            write_metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfigRef {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `drop_unknown_fields` after provisioning.\nWhen true and 'useTopicSchema' is true, any fields that are a part of the topic schema that are\nnot part of the BigQuery table schema are dropped when writing to BigQuery. Otherwise, the schemas\nmust be kept in sync and any messages with extra fields are not written and remain in the\nsubscription's backlog."]
    pub fn drop_unknown_fields(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drop_unknown_fields", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe service account to use to write to BigQuery. The subscription creator or updater that\nspecifies this field must have 'iam.serviceAccounts.actAs' permission on the service account.\nIf not specified, the Pub/Sub service agent,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table` after provisioning.\nThe name of the table to which to write data, of the form\n{projectId}.{datasetId}.{tableId}"]
    pub fn table(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_table_schema` after provisioning.\nWhen true, use the BigQuery table's schema as the columns to write to in BigQuery.\n'useTableSchema' and 'useTopicSchema' cannot be enabled at the same time."]
    pub fn use_table_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_table_schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_topic_schema` after provisioning.\nWhen true, use the topic's schema as the columns to write to in BigQuery,\nif it exists. 'useTopicSchema' and 'useTableSchema' cannot be enabled at the same time."]
    pub fn use_topic_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_topic_schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `write_metadata` after provisioning.\nWhen true, write the subscription name, message_id, publish_time, attributes, and ordering_key\nto additional columns in the table. The subscription name, message_id, and publish_time fields\nare put in their own columns while all other message properties (other than data) are written\nto a JSON object in the attributes column."]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_topic_schema: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfig {
    #[doc =
        "Set the field `use_topic_schema`.\nWhen true, the output Cloud Storage file will be serialized using\nthe topic schema, if it exists."]
    pub fn set_use_topic_schema(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_topic_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `write_metadata`.\nWhen true, write the subscription name, message_id, publish_time, attributes, and ordering_key\nas additional fields in the output. The subscription name, message_id, and publish_time fields\nare put in their own fields while all other message properties other than data (for example,\nan ordering_key, if present) are added as entries in the attributes map."]
    pub fn set_write_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.write_metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfig {
    type O =
        terrars
        ::BlockAssignable<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfig {}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfig {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfig {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfig {
            use_topic_schema: core::default::Default::default(),
            write_metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfigRef {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `use_topic_schema` after provisioning.\nWhen true, the output Cloud Storage file will be serialized using\nthe topic schema, if it exists."]
    pub fn use_topic_schema(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_topic_schema", self.base))
    }

    #[doc =
        "Get a reference to the value of field `write_metadata` after provisioning.\nWhen true, write the subscription name, message_id, publish_time, attributes, and ordering_key\nas additional fields in the output. The subscription name, message_id, and publish_time fields\nare put in their own fields while all other message properties other than data (for example,\nan ordering_key, if present) are added as entries in the attributes map."]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfigDynamic {
    avro_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filename_datetime_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filename_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filename_suffix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_bytes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_messages: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    avro_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfig,
        >,
    >,
    dynamic: BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfigDynamic,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig {
    #[doc =
        "Set the field `bucket`.\nUser-provided name for the Cloud Storage bucket. The bucket must be created by the user.\nThe bucket name must be without any prefix like \"gs://\". See the\n[bucket naming requirements](https://cloud.google.com/storage/docs/buckets#naming)."]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filename_datetime_format`.\nUser-provided format string specifying how to represent datetimes in Cloud Storage filenames.\nSee the [datetime format guidance](https://cloud.google.com/pubsub/docs/create-cloudstorage-subscription#file_names)."]
    pub fn set_filename_datetime_format(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filename_datetime_format = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filename_prefix`.\nUser-provided prefix for Cloud Storage filename. See the\n[object naming requirements](https://cloud.google.com/storage/docs/objects#naming)."]
    pub fn set_filename_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filename_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filename_suffix`.\nUser-provided suffix for Cloud Storage filename. See the\n[object naming requirements](https://cloud.google.com/storage/docs/objects#naming).\nMust not end in \"/\"."]
    pub fn set_filename_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filename_suffix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_bytes`.\nThe maximum bytes that can be written to a Cloud Storage file before a new file is created.\nMin 1 KB, max 10 GiB. The maxBytes limit may be exceeded in cases where messages are larger\nthan the limit."]
    pub fn set_max_bytes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_bytes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_duration`.\nThe maximum duration that can elapse before a new Cloud Storage file is created.\nMin 1 minute, max 10 minutes, default 5 minutes. May not exceed the subscription's\nacknowledgement deadline."]
    pub fn set_max_duration(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_messages`.\nThe maximum number of messages that can be written to a Cloud Storage file before a new file\nis created. Min 1000 messages."]
    pub fn set_max_messages(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_email`.\nThe service account to use to write to Cloud Storage. The subscription creator or updater that\nspecifies this field must have 'iam.serviceAccounts.actAs' permission on the service account.\nIf not specified, the Pub/Sub service agent,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `avro_config`.\n"]
    pub fn set_avro_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.avro_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.avro_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig {
    type O =
        terrars
        ::BlockAssignable<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig {}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig {
            bucket: core::default::Default::default(),
            filename_datetime_format: core::default::Default::default(),
            filename_prefix: core::default::Default::default(),
            filename_suffix: core::default::Default::default(),
            max_bytes: core::default::Default::default(),
            max_duration: core::default::Default::default(),
            max_messages: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            avro_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfigRef {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nUser-provided name for the Cloud Storage bucket. The bucket must be created by the user.\nThe bucket name must be without any prefix like \"gs://\". See the\n[bucket naming requirements](https://cloud.google.com/storage/docs/buckets#naming)."]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filename_datetime_format` after provisioning.\nUser-provided format string specifying how to represent datetimes in Cloud Storage filenames.\nSee the [datetime format guidance](https://cloud.google.com/pubsub/docs/create-cloudstorage-subscription#file_names)."]
    pub fn filename_datetime_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename_datetime_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filename_prefix` after provisioning.\nUser-provided prefix for Cloud Storage filename. See the\n[object naming requirements](https://cloud.google.com/storage/docs/objects#naming)."]
    pub fn filename_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filename_suffix` after provisioning.\nUser-provided suffix for Cloud Storage filename. See the\n[object naming requirements](https://cloud.google.com/storage/docs/objects#naming).\nMust not end in \"/\"."]
    pub fn filename_suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename_suffix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_bytes` after provisioning.\nThe maximum bytes that can be written to a Cloud Storage file before a new file is created.\nMin 1 KB, max 10 GiB. The maxBytes limit may be exceeded in cases where messages are larger\nthan the limit."]
    pub fn max_bytes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_bytes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_duration` after provisioning.\nThe maximum duration that can elapse before a new Cloud Storage file is created.\nMin 1 minute, max 10 minutes, default 5 minutes. May not exceed the subscription's\nacknowledgement deadline."]
    pub fn max_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_messages` after provisioning.\nThe maximum number of messages that can be written to a Cloud Storage file before a new file\nis created. Min 1000 messages."]
    pub fn max_messages(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_messages", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe service account to use to write to Cloud Storage. The subscription creator or updater that\nspecifies this field must have 'iam.serviceAccounts.actAs' permission on the service account.\nIf not specified, the Pub/Sub service agent,\nservice-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `avro_config` after provisioning.\n"]
    pub fn avro_config(
        &self,
    ) -> terrars
    ::ListRef<
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig_AvroConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.avro_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_delivery_attempts: std::option::Option<terrars::PrimField<f64>>,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicy {
    #[doc =
        "Set the field `dead_letter_topic`.\nThe name of the topic to which dead letter messages should be published. Format is\n'projects/{project}/topics/{topic}'. The Pub/Sub service account associated with the enclosing\nsubscription's parent project (i.e., service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com)\nmust have permission to Publish() to this topic. The operation will fail if the topic does not exist.\nUsers should ensure that there is a subscription attached to this topic since messages published to\na topic with no subscriptions are lost."]
    pub fn set_dead_letter_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dead_letter_topic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_delivery_attempts`.\nThe maximum number of delivery attempts for any message. The value must be between 5 and 100.\nThe number of delivery attempts is defined as 1 + (the sum of number of NACKs and number of times\nthe acknowledgement deadline has been exceeded for the message). A NACK is any call to\nModifyAckDeadline with a 0 deadline. Note that client libraries may automatically extend\nack_deadlines. This field will be honored on a best effort basis. If this parameter is 0, a\ndefault value of 5 is used."]
    pub fn set_max_delivery_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_delivery_attempts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicy {
    type O =
        terrars
        ::BlockAssignable<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicy,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicy {}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicy {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicy {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicy {
            dead_letter_topic: core::default::Default::default(),
            max_delivery_attempts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicyRef {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dead_letter_topic` after provisioning.\nThe name of the topic to which dead letter messages should be published. Format is\n'projects/{project}/topics/{topic}'. The Pub/Sub service account associated with the enclosing\nsubscription's parent project (i.e., service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com)\nmust have permission to Publish() to this topic. The operation will fail if the topic does not exist.\nUsers should ensure that there is a subscription attached to this topic since messages published to\na topic with no subscriptions are lost."]
    pub fn dead_letter_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dead_letter_topic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_delivery_attempts` after provisioning.\nThe maximum number of delivery attempts for any message. The value must be between 5 and 100.\nThe number of delivery attempts is defined as 1 + (the sum of number of NACKs and number of times\nthe acknowledgement deadline has been exceeded for the message). A NACK is any call to\nModifyAckDeadline with a 0 deadline. Note that client libraries may automatically extend\nack_deadlines. This field will be honored on a best effort basis. If this parameter is 0, a\ndefault value of 5 is used."]
    pub fn max_delivery_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_delivery_attempts", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicy {
    #[doc =
        "Set the field `ttl`.\nSpecifies the \"time-to-live\" duration for an associated resource. The resource expires if it\nis not active for a period of 'ttl'. The definition of \"activity\" depends on the type of the\nassociated resource. The minimum and maximum allowed values for 'ttl' depend on the type of\nthe associated resource, as well. If 'ttl' is not set, the associated resource never expires."]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicy {
    type O =
        terrars
        ::BlockAssignable<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicy,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicy {}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicy {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicy {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicy {
            ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicyRef {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ttl` after provisioning.\nSpecifies the \"time-to-live\" duration for an associated resource. The resource expires if it\nis not active for a period of 'ttl'. The definition of \"activity\" depends on the type of the\nassociated resource. The minimum and maximum allowed values for 'ttl' depend on the type of\nthe associated resource, as well. If 'ttl' is not set, the associated resource never expires."]
    pub fn ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapper {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_metadata: std::option::Option<terrars::PrimField<bool>>,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapper {
    #[doc =
        "Set the field `write_metadata`.\nWhen true, writes the Pub/Sub message metadata to 'x-goog-pubsub-<KEY>:<VAL>' headers of the\nHTTP request. Writes the Pub/Sub message attributes to '<KEY>:<VAL>' headers of the HTTP request."]
    pub fn set_write_metadata(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.write_metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapper {
    type O =
        terrars
        ::BlockAssignable<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapper,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapper {}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapper {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapper {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapper {
            write_metadata: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapperRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapperRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapperRef {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapperRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapperRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `write_metadata` after provisioning.\nWhen true, writes the Pub/Sub message metadata to 'x-goog-pubsub-<KEY>:<VAL>' headers of the\nHTTP request. Writes the Pub/Sub message attributes to '<KEY>:<VAL>' headers of the HTTP request."]
    pub fn write_metadata(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.write_metadata", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcToken {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audience: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcToken {
    #[doc =
        "Set the field `audience`.\nAudience to be used when generating OIDC token. The audience claim identifies the recipients\nthat the JWT is intended for. The audience value is a single case-sensitive string. Having\nmultiple values (array) for the audience field is not supported. More info about the OIDC JWT\ntoken audience here: https://tools.ietf.org/html/rfc7519#section-4.1.3 Note: if not specified,\nthe Push endpoint URL will be used."]
    pub fn set_audience(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audience = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_email`.\nService account email used for generating the OIDC token. For more information\non setting up authentication, see Push subscriptions."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcToken {
    type O =
        terrars
        ::BlockAssignable<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcToken,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcToken {}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcToken {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcToken {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcToken {
            audience: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcTokenRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcTokenRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcTokenRef {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcTokenRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcTokenRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `audience` after provisioning.\nAudience to be used when generating OIDC token. The audience claim identifies the recipients\nthat the JWT is intended for. The audience value is a single case-sensitive string. Having\nmultiple values (array) for the audience field is not supported. More info about the OIDC JWT\ntoken audience here: https://tools.ietf.org/html/rfc7519#section-4.1.3 Note: if not specified,\nthe Push endpoint URL will be used."]
    pub fn audience(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audience", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nService account email used for generating the OIDC token. For more information\non setting up authentication, see Push subscriptions."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfigDynamic {
    no_wrapper: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapper,
        >,
    >,
    oidc_token: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcToken,
        >,
    >,
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_wrapper: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapper,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oidc_token: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcToken,
        >,
    >,
    dynamic: BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfigDynamic,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig {
    #[doc =
        "Set the field `attributes`.\nEndpoint configuration attributes that can be used to control different aspects of the message delivery.\nThe only currently supported attribute is 'x-goog-version', which you can use to change the format of the\npushed message. This attribute indicates the version of the data expected by the endpoint. This controls\nthe shape of the pushed message (i.e., its fields and metadata). If not present during the\n'CreateSubscription' call, it will default to the version of the Pub/Sub API used to make such call.\nIf not present in a 'ModifyPushConfig' call, its value will not be changed. 'GetSubscription' calls\nwill always return a valid version, even if the subscription was created without this attribute.\nThe only supported values for the 'x-goog-version' attribute are: 'v1beta1': uses the push format\ndefined in the v1beta1 Pub/Sub API. 'v1' or 'v1beta2': uses the push format defined in the v1 Pub/Sub API."]
    pub fn set_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `push_endpoint`.\nA URL locating the endpoint to which messages should be pushed.\nFor example, a Webhook endpoint might use 'https://example.com/push'."]
    pub fn set_push_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.push_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_wrapper`.\n"]
    pub fn set_no_wrapper(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapper,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.no_wrapper = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.no_wrapper = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oidc_token`.\n"]
    pub fn set_oidc_token(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcToken,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oidc_token = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oidc_token = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig {
    type O =
        terrars
        ::BlockAssignable<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig {}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig {
            attributes: core::default::Default::default(),
            push_endpoint: core::default::Default::default(),
            no_wrapper: core::default::Default::default(),
            oidc_token: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfigRef {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attributes` after provisioning.\nEndpoint configuration attributes that can be used to control different aspects of the message delivery.\nThe only currently supported attribute is 'x-goog-version', which you can use to change the format of the\npushed message. This attribute indicates the version of the data expected by the endpoint. This controls\nthe shape of the pushed message (i.e., its fields and metadata). If not present during the\n'CreateSubscription' call, it will default to the version of the Pub/Sub API used to make such call.\nIf not present in a 'ModifyPushConfig' call, its value will not be changed. 'GetSubscription' calls\nwill always return a valid version, even if the subscription was created without this attribute.\nThe only supported values for the 'x-goog-version' attribute are: 'v1beta1': uses the push format\ndefined in the v1beta1 Pub/Sub API. 'v1' or 'v1beta2': uses the push format defined in the v1 Pub/Sub API."]
    pub fn attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `push_endpoint` after provisioning.\nA URL locating the endpoint to which messages should be pushed.\nFor example, a Webhook endpoint might use 'https://example.com/push'."]
    pub fn push_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.push_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `no_wrapper` after provisioning.\n"]
    pub fn no_wrapper(
        &self,
    ) -> terrars
    ::ListRef<
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_NoWrapperRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.no_wrapper", self.base))
    }

    #[doc = "Get a reference to the value of field `oidc_token` after provisioning.\n"]
    pub fn oidc_token(
        &self,
    ) -> terrars
    ::ListRef<
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig_OidcTokenRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oidc_token", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_backoff: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_backoff: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicy {
    #[doc =
        "Set the field `maximum_backoff`.\nThe maximum delay between consecutive deliveries of a given message.\nValue should be between 0 and 600 seconds. Defaults to 600 seconds."]
    pub fn set_maximum_backoff(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.maximum_backoff = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minimum_backoff`.\nThe minimum delay between consecutive deliveries of a given message.\nValue should be between 0 and 600 seconds. Defaults to 10 seconds."]
    pub fn set_minimum_backoff(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.minimum_backoff = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicy {
    type O =
        terrars
        ::BlockAssignable<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicy,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicy {}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicy {
    pub fn build(
        self,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicy {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicy {
            maximum_backoff: core::default::Default::default(),
            minimum_backoff: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicyRef {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `maximum_backoff` after provisioning.\nThe maximum delay between consecutive deliveries of a given message.\nValue should be between 0 and 600 seconds. Defaults to 600 seconds."]
    pub fn maximum_backoff(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_backoff", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum_backoff` after provisioning.\nThe minimum delay between consecutive deliveries of a given message.\nValue should be between 0 and 600 seconds. Defaults to 10 seconds."]
    pub fn minimum_backoff(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_backoff", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscriptionDynamic {
    bigquery_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfig,
        >,
    >,
    cloud_storage_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig,
        >,
    >,
    dead_letter_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicy,
        >,
    >,
    expiration_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicy,
        >,
    >,
    push_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig,
        >,
    >,
    retry_policy: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicy,
        >,
    >,
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ack_deadline_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detached: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_exactly_once_delivery: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_message_ordering: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_retention_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_acked_messages: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dead_letter_policy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicy,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_policy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicy,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    push_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicy>,
    >,
    dynamic: BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscriptionDynamic,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription {
    #[doc =
        "Set the field `ack_deadline_seconds`.\nThe approximate amount of time (on a best-effort basis) Pub/Sub waits for the subscriber to\nacknowledge receipt before resending the message. In the interval after the message is delivered\nand before it is acknowledged, it is considered to be outstanding. During that time period, the\nmessage will not be redelivered (on a best-effort basis). For pull subscriptions, this value is\nused as the initial value for the ack deadline. To override this value for a given message, call\n'ModifyAckDeadline' with the corresponding 'ack_id' if using non-streaming pull or send the\n'ack_id' in a 'StreamingModifyAckDeadlineRequest' if using streaming pull. The minimum custom\ndeadline you can specify is 10 seconds. The maximum custom deadline you can specify is 600\nseconds (10 minutes). If this parameter is 0, a default value of 10 seconds is used. For push\ndelivery, this value is also used to set the request timeout for the call to the push endpoint.\nIf the subscriber never acknowledges the message, the Pub/Sub system will eventually redeliver\nthe message."]
    pub fn set_ack_deadline_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ack_deadline_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `detached`.\nIndicates whether the subscription is detached from its topic. Detached subscriptions don't\nreceive messages from their topic and don't retain any backlog. 'Pull' and 'StreamingPull'\nrequests will return FAILED_PRECONDITION. If the subscription is a push subscription, pushes\nto the endpoint will not be made."]
    pub fn set_detached(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.detached = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_exactly_once_delivery`.\nIf true, Pub/Sub provides the following guarantees for the delivery of a message with a given\nvalue of 'message_id' on this subscription: The message sent to a subscriber is guaranteed not\nto be resent before the message's acknowledgement deadline expires. An acknowledged message will\nnot be resent to a subscriber. Note that subscribers may still receive multiple copies of a\nmessage when 'enableExactlyOnceDelivery' is true if the message was published multiple times by\na publisher client. These copies are considered distinct by Pub/Sub and have distinct 'message_id'\nvalues."]
    pub fn set_enable_exactly_once_delivery(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_exactly_once_delivery = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_message_ordering`.\nIf true, messages published with the same 'ordering_key' in 'PubsubMessage'\nwill be delivered to the subscribers in the order in which they are received\nby the Pub/Sub system. Otherwise, they may be delivered in any order."]
    pub fn set_enable_message_ordering(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_message_ordering = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter`.\nAn expression written in the Pub/Sub filter language. If non-empty, then only 'PubsubMessage's\nwhose 'attributes' field matches the filter are delivered on this subscription. If empty, then\nno messages are filtered out."]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\nSee [Creating and managing labels](https://cloud.google.com/pubsub/docs/labels)."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `message_retention_duration`.\nHow long to retain unacknowledged messages in the subscription's backlog, from the moment a\nmessage is published. If 'retainAckedMessages' is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a Seek can be done. Defaults\nto 7 days. Cannot be more than 31 days or less than 10 minutes."]
    pub fn set_message_retention_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.message_retention_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retain_acked_messages`.\nIndicates whether to retain acknowledged messages. If true, then messages are not expunged from\nthe subscription's backlog, even if they are acknowledged, until they fall out of the\n'messageRetentionDuration' window. This must be true if you would like to Seek to a timestamp\nin the past to replay previously-acknowledged messages."]
    pub fn set_retain_acked_messages(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.retain_acked_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigquery_config`.\n"]
    pub fn set_bigquery_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_storage_config`.\n"]
    pub fn set_cloud_storage_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dead_letter_policy`.\n"]
    pub fn set_dead_letter_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dead_letter_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dead_letter_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `expiration_policy`.\n"]
    pub fn set_expiration_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expiration_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expiration_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `push_config`.\n"]
    pub fn set_push_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.push_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.push_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicy,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retry_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retry_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription {
    type O =
        terrars
        ::BlockAssignable<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription {
    #[doc = "Name of the subscription. Format is 'projects/{project}/subscriptions/{sub}'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription {
            ack_deadline_seconds: core::default::Default::default(),
            detached: core::default::Default::default(),
            enable_exactly_once_delivery: core::default::Default::default(),
            enable_message_ordering: core::default::Default::default(),
            filter: core::default::Default::default(),
            labels: core::default::Default::default(),
            message_retention_duration: core::default::Default::default(),
            name: self.name,
            retain_acked_messages: core::default::Default::default(),
            bigquery_config: core::default::Default::default(),
            cloud_storage_config: core::default::Default::default(),
            dead_letter_policy: core::default::Default::default(),
            expiration_policy: core::default::Default::default(),
            push_config: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscriptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscriptionRef {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscriptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscriptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ack_deadline_seconds` after provisioning.\nThe approximate amount of time (on a best-effort basis) Pub/Sub waits for the subscriber to\nacknowledge receipt before resending the message. In the interval after the message is delivered\nand before it is acknowledged, it is considered to be outstanding. During that time period, the\nmessage will not be redelivered (on a best-effort basis). For pull subscriptions, this value is\nused as the initial value for the ack deadline. To override this value for a given message, call\n'ModifyAckDeadline' with the corresponding 'ack_id' if using non-streaming pull or send the\n'ack_id' in a 'StreamingModifyAckDeadlineRequest' if using streaming pull. The minimum custom\ndeadline you can specify is 10 seconds. The maximum custom deadline you can specify is 600\nseconds (10 minutes). If this parameter is 0, a default value of 10 seconds is used. For push\ndelivery, this value is also used to set the request timeout for the call to the push endpoint.\nIf the subscriber never acknowledges the message, the Pub/Sub system will eventually redeliver\nthe message."]
    pub fn ack_deadline_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ack_deadline_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `detached` after provisioning.\nIndicates whether the subscription is detached from its topic. Detached subscriptions don't\nreceive messages from their topic and don't retain any backlog. 'Pull' and 'StreamingPull'\nrequests will return FAILED_PRECONDITION. If the subscription is a push subscription, pushes\nto the endpoint will not be made."]
    pub fn detached(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detached", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_exactly_once_delivery` after provisioning.\nIf true, Pub/Sub provides the following guarantees for the delivery of a message with a given\nvalue of 'message_id' on this subscription: The message sent to a subscriber is guaranteed not\nto be resent before the message's acknowledgement deadline expires. An acknowledged message will\nnot be resent to a subscriber. Note that subscribers may still receive multiple copies of a\nmessage when 'enableExactlyOnceDelivery' is true if the message was published multiple times by\na publisher client. These copies are considered distinct by Pub/Sub and have distinct 'message_id'\nvalues."]
    pub fn enable_exactly_once_delivery(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_exactly_once_delivery", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_message_ordering` after provisioning.\nIf true, messages published with the same 'ordering_key' in 'PubsubMessage'\nwill be delivered to the subscribers in the order in which they are received\nby the Pub/Sub system. Otherwise, they may be delivered in any order."]
    pub fn enable_message_ordering(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_message_ordering", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nAn expression written in the Pub/Sub filter language. If non-empty, then only 'PubsubMessage's\nwhose 'attributes' field matches the filter are delivered on this subscription. If empty, then\nno messages are filtered out."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nSee [Creating and managing labels](https://cloud.google.com/pubsub/docs/labels)."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `message_retention_duration` after provisioning.\nHow long to retain unacknowledged messages in the subscription's backlog, from the moment a\nmessage is published. If 'retainAckedMessages' is true, then this also configures the retention\nof acknowledged messages, and thus configures how far back in time a Seek can be done. Defaults\nto 7 days. Cannot be more than 31 days or less than 10 minutes."]
    pub fn message_retention_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_retention_duration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the subscription. Format is 'projects/{project}/subscriptions/{sub}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retain_acked_messages` after provisioning.\nIndicates whether to retain acknowledged messages. If true, then messages are not expunged from\nthe subscription's backlog, even if they are acknowledged, until they fall out of the\n'messageRetentionDuration' window. This must be true if you would like to Seek to a timestamp\nin the past to replay previously-acknowledged messages."]
    pub fn retain_acked_messages(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_acked_messages", self.base))
    }

    #[doc = "Get a reference to the value of field `bigquery_config` after provisioning.\n"]
    pub fn bigquery_config(
        &self,
    ) -> terrars
    ::ListRef<
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_BigqueryConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_config", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_storage_config` after provisioning.\n"]
    pub fn cloud_storage_config(
        &self,
    ) -> terrars
    ::ListRef<
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_CloudStorageConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_config", self.base))
    }

    #[doc = "Get a reference to the value of field `dead_letter_policy` after provisioning.\n"]
    pub fn dead_letter_policy(
        &self,
    ) -> terrars
    ::ListRef<
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_DeadLetterPolicyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dead_letter_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `expiration_policy` after provisioning.\n"]
    pub fn expiration_policy(
        &self,
    ) -> terrars
    ::ListRef<
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_ExpirationPolicyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiration_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `push_config` after provisioning.\n"]
    pub fn push_config(
        &self,
    ) -> terrars
    ::ListRef<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_PushConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.push_config", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(
        &self,
    ) -> terrars
    ::ListRef<
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription_RetryPolicyRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscriptionDynamic {
    pubsub_subscription: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription>,
    >,
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_subscription: std
    ::option
    ::Option<std::vec::Vec<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription>>,
    dynamic: BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscriptionDynamic,
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription {
    #[doc = "Set the field `pubsub_subscription`.\n"]
    pub fn set_pubsub_subscription(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscription,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pubsub_subscription = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub_subscription = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription {}

impl BuildBigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription {
            pubsub_subscription: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscriptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscriptionRef {
        BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscriptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscriptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pubsub_subscription` after provisioning.\n"]
    pub fn pubsub_subscription(
        &self,
    ) -> terrars
    ::ListRef<BigqueryAnalyticsHubListingSubscription_DestinationPubsubSubscription_PubsubSubscriptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_subscription", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingSubscription_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubListingSubscription_Timeouts {
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
}

impl terrars::ToListMappable for BigqueryAnalyticsHubListingSubscription_Timeouts {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubListingSubscription_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingSubscription_Timeouts {}

impl BuildBigqueryAnalyticsHubListingSubscription_Timeouts {
    pub fn build(self) -> BigqueryAnalyticsHubListingSubscription_Timeouts {
        BigqueryAnalyticsHubListingSubscription_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubListingSubscription_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubListingSubscription_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubListingSubscription_TimeoutsRef {
        BigqueryAnalyticsHubListingSubscription_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingSubscription_TimeoutsRef {
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
}
