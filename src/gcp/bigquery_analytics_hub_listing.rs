use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryAnalyticsHubListingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    categories: Option<ListField<PrimField<String>>>,
    data_exchange_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentation: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    listing_id: PrimField<String>,
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_contact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_access: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bigquery_dataset: Option<Vec<BigqueryAnalyticsHubListingBigqueryDatasetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_provider: Option<Vec<BigqueryAnalyticsHubListingDataProviderEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publisher: Option<Vec<BigqueryAnalyticsHubListingPublisherEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryAnalyticsHubListingTimeoutsEl>,
    dynamic: BigqueryAnalyticsHubListingDynamic,
}

struct BigqueryAnalyticsHubListing_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryAnalyticsHubListingData>,
}

#[derive(Clone)]
pub struct BigqueryAnalyticsHubListing(Rc<BigqueryAnalyticsHubListing_>);

impl BigqueryAnalyticsHubListing {
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

    #[doc= "Set the field `categories`.\nCategories of the listing. Up to two categories are allowed."]
    pub fn set_categories(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().categories = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nShort description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF)."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `documentation`.\nDocumentation describing the listing."]
    pub fn set_documentation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().documentation = Some(v.into());
        self
    }

    #[doc= "Set the field `icon`.\nBase64 encoded image representing the listing."]
    pub fn set_icon(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().icon = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `primary_contact`.\nEmail or URL of the primary point of contact of the listing."]
    pub fn set_primary_contact(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().primary_contact = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `request_access`.\nEmail or URL of the request access of the listing. Subscribers can use this reference to request access."]
    pub fn set_request_access(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().request_access = Some(v.into());
        self
    }

    #[doc= "Set the field `bigquery_dataset`.\n"]
    pub fn set_bigquery_dataset(
        self,
        v: impl Into<BlockAssignable<BigqueryAnalyticsHubListingBigqueryDatasetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bigquery_dataset = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bigquery_dataset = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `data_provider`.\n"]
    pub fn set_data_provider(self, v: impl Into<BlockAssignable<BigqueryAnalyticsHubListingDataProviderEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_provider = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_provider = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `publisher`.\n"]
    pub fn set_publisher(self, v: impl Into<BlockAssignable<BigqueryAnalyticsHubListingPublisherEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().publisher = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.publisher = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryAnalyticsHubListingTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `categories` after provisioning.\nCategories of the listing. Up to two categories are allowed."]
    pub fn categories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.categories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_exchange_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nShort description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the listing."]
    pub fn documentation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the listing."]
    pub fn icon(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.icon", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `listing_id` after provisioning.\nThe ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn listing_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.listing_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange listing."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the listing. e.g. \"projects/myproject/locations/US/dataExchanges/123/listings/456\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the listing."]
    pub fn primary_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_access` after provisioning.\nEmail or URL of the request access of the listing. Subscribers can use this reference to request access."]
    pub fn request_access(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bigquery_dataset` after provisioning.\n"]
    pub fn bigquery_dataset(&self) -> ListRef<BigqueryAnalyticsHubListingBigqueryDatasetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bigquery_dataset", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_provider` after provisioning.\n"]
    pub fn data_provider(&self) -> ListRef<BigqueryAnalyticsHubListingDataProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_provider", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `publisher` after provisioning.\n"]
    pub fn publisher(&self) -> ListRef<BigqueryAnalyticsHubListingPublisherElRef> {
        ListRef::new(self.shared().clone(), format!("{}.publisher", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubListingTimeoutsElRef {
        BigqueryAnalyticsHubListingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for BigqueryAnalyticsHubListing {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BigqueryAnalyticsHubListing { }

impl ToListMappable for BigqueryAnalyticsHubListing {
    type O = ListRef<BigqueryAnalyticsHubListingRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BigqueryAnalyticsHubListing_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_analytics_hub_listing".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryAnalyticsHubListing {
    pub tf_id: String,
    #[doc= "The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub data_exchange_id: PrimField<String>,
    #[doc= "Human-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces."]
    pub display_name: PrimField<String>,
    #[doc= "The ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub listing_id: PrimField<String>,
    #[doc= "The name of the location this data exchange listing."]
    pub location: PrimField<String>,
}

impl BuildBigqueryAnalyticsHubListing {
    pub fn build(self, stack: &mut Stack) -> BigqueryAnalyticsHubListing {
        let out = BigqueryAnalyticsHubListing(Rc::new(BigqueryAnalyticsHubListing_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryAnalyticsHubListingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                categories: core::default::Default::default(),
                data_exchange_id: self.data_exchange_id,
                description: core::default::Default::default(),
                display_name: self.display_name,
                documentation: core::default::Default::default(),
                icon: core::default::Default::default(),
                id: core::default::Default::default(),
                listing_id: self.listing_id,
                location: self.location,
                primary_contact: core::default::Default::default(),
                project: core::default::Default::default(),
                request_access: core::default::Default::default(),
                bigquery_dataset: core::default::Default::default(),
                data_provider: core::default::Default::default(),
                publisher: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryAnalyticsHubListingRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryAnalyticsHubListingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryAnalyticsHubListingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `categories` after provisioning.\nCategories of the listing. Up to two categories are allowed."]
    pub fn categories(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.categories", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_exchange_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nShort description of the listing. The description must not contain Unicode non-characters and C0 and C1 control codes except tabs (HT), new lines (LF), carriage returns (CR), and page breaks (FF)."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the listing. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), ampersands (&) and can't start or end with spaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the listing."]
    pub fn documentation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the listing."]
    pub fn icon(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.icon", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `listing_id` after provisioning.\nThe ID of the listing. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn listing_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.listing_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange listing."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the listing. e.g. \"projects/myproject/locations/US/dataExchanges/123/listings/456\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the listing."]
    pub fn primary_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `request_access` after provisioning.\nEmail or URL of the request access of the listing. Subscribers can use this reference to request access."]
    pub fn request_access(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.request_access", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bigquery_dataset` after provisioning.\n"]
    pub fn bigquery_dataset(&self) -> ListRef<BigqueryAnalyticsHubListingBigqueryDatasetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.bigquery_dataset", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `data_provider` after provisioning.\n"]
    pub fn data_provider(&self) -> ListRef<BigqueryAnalyticsHubListingDataProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.data_provider", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `publisher` after provisioning.\n"]
    pub fn publisher(&self) -> ListRef<BigqueryAnalyticsHubListingPublisherElRef> {
        ListRef::new(self.shared().clone(), format!("{}.publisher", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubListingTimeoutsElRef {
        BigqueryAnalyticsHubListingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingBigqueryDatasetEl {
    dataset: PrimField<String>,
}

impl BigqueryAnalyticsHubListingBigqueryDatasetEl { }

impl ToListMappable for BigqueryAnalyticsHubListingBigqueryDatasetEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingBigqueryDatasetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingBigqueryDatasetEl {
    #[doc= "Resource name of the dataset source for this listing. e.g. projects/myproject/datasets/123"]
    pub dataset: PrimField<String>,
}

impl BuildBigqueryAnalyticsHubListingBigqueryDatasetEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingBigqueryDatasetEl {
        BigqueryAnalyticsHubListingBigqueryDatasetEl { dataset: self.dataset }
    }
}

pub struct BigqueryAnalyticsHubListingBigqueryDatasetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryAnalyticsHubListingBigqueryDatasetElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubListingBigqueryDatasetElRef {
        BigqueryAnalyticsHubListingBigqueryDatasetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingBigqueryDatasetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `dataset` after provisioning.\nResource name of the dataset source for this listing. e.g. projects/myproject/datasets/123"]
    pub fn dataset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingDataProviderEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_contact: Option<PrimField<String>>,
}

impl BigqueryAnalyticsHubListingDataProviderEl {
    #[doc= "Set the field `primary_contact`.\nEmail or URL of the data provider."]
    pub fn set_primary_contact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_contact = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryAnalyticsHubListingDataProviderEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingDataProviderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingDataProviderEl {
    #[doc= "Name of the data provider."]
    pub name: PrimField<String>,
}

impl BuildBigqueryAnalyticsHubListingDataProviderEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingDataProviderEl {
        BigqueryAnalyticsHubListingDataProviderEl {
            name: self.name,
            primary_contact: core::default::Default::default(),
        }
    }
}

pub struct BigqueryAnalyticsHubListingDataProviderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryAnalyticsHubListingDataProviderElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubListingDataProviderElRef {
        BigqueryAnalyticsHubListingDataProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingDataProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the data provider."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the data provider."]
    pub fn primary_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingPublisherEl {
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_contact: Option<PrimField<String>>,
}

impl BigqueryAnalyticsHubListingPublisherEl {
    #[doc= "Set the field `primary_contact`.\nEmail or URL of the listing publisher."]
    pub fn set_primary_contact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.primary_contact = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryAnalyticsHubListingPublisherEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingPublisherEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingPublisherEl {
    #[doc= "Name of the listing publisher."]
    pub name: PrimField<String>,
}

impl BuildBigqueryAnalyticsHubListingPublisherEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingPublisherEl {
        BigqueryAnalyticsHubListingPublisherEl {
            name: self.name,
            primary_contact: core::default::Default::default(),
        }
    }
}

pub struct BigqueryAnalyticsHubListingPublisherElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryAnalyticsHubListingPublisherElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubListingPublisherElRef {
        BigqueryAnalyticsHubListingPublisherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingPublisherElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nName of the listing publisher."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc= "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the listing publisher."]
    pub fn primary_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubListingTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BigqueryAnalyticsHubListingTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for BigqueryAnalyticsHubListingTimeoutsEl {
    type O = BlockAssignable<BigqueryAnalyticsHubListingTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubListingTimeoutsEl {}

impl BuildBigqueryAnalyticsHubListingTimeoutsEl {
    pub fn build(self) -> BigqueryAnalyticsHubListingTimeoutsEl {
        BigqueryAnalyticsHubListingTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BigqueryAnalyticsHubListingTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryAnalyticsHubListingTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubListingTimeoutsElRef {
        BigqueryAnalyticsHubListingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubListingTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubListingDynamic {
    bigquery_dataset: Option<DynamicBlock<BigqueryAnalyticsHubListingBigqueryDatasetEl>>,
    data_provider: Option<DynamicBlock<BigqueryAnalyticsHubListingDataProviderEl>>,
    publisher: Option<DynamicBlock<BigqueryAnalyticsHubListingPublisherEl>>,
}
