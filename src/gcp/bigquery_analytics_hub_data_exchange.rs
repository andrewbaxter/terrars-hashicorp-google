use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryAnalyticsHubDataExchangeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
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
    location: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_contact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BigqueryAnalyticsHubDataExchangeTimeoutsEl>,
}

struct BigqueryAnalyticsHubDataExchange_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BigqueryAnalyticsHubDataExchangeData>,
}

#[derive(Clone)]
pub struct BigqueryAnalyticsHubDataExchange(Rc<BigqueryAnalyticsHubDataExchange_>);

impl BigqueryAnalyticsHubDataExchange {
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

    #[doc= "Set the field `description`.\nDescription of the data exchange."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `documentation`.\nDocumentation describing the data exchange."]
    pub fn set_documentation(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().documentation = Some(v.into());
        self
    }

    #[doc= "Set the field `icon`.\nBase64 encoded image representing the data exchange."]
    pub fn set_icon(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().icon = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `primary_contact`.\nEmail or URL of the primary point of contact of the data exchange."]
    pub fn set_primary_contact(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().primary_contact = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BigqueryAnalyticsHubDataExchangeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_exchange_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the data exchange."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and must not start or end with spaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the data exchange."]
    pub fn documentation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the data exchange."]
    pub fn icon(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.icon", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `listing_count` after provisioning.\nNumber of listings contained in the data exchange."]
    pub fn listing_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.listing_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the data exchange, for example:\n\"projects/myproject/locations/US/dataExchanges/123\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the data exchange."]
    pub fn primary_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
        BigqueryAnalyticsHubDataExchangeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for BigqueryAnalyticsHubDataExchange {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for BigqueryAnalyticsHubDataExchange { }

impl ToListMappable for BigqueryAnalyticsHubDataExchange {
    type O = ListRef<BigqueryAnalyticsHubDataExchangeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for BigqueryAnalyticsHubDataExchange_ {
    fn extract_resource_type(&self) -> String {
        "google_bigquery_analytics_hub_data_exchange".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryAnalyticsHubDataExchange {
    pub tf_id: String,
    #[doc= "The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub data_exchange_id: PrimField<String>,
    #[doc= "Human-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and must not start or end with spaces."]
    pub display_name: PrimField<String>,
    #[doc= "The name of the location this data exchange."]
    pub location: PrimField<String>,
}

impl BuildBigqueryAnalyticsHubDataExchange {
    pub fn build(self, stack: &mut Stack) -> BigqueryAnalyticsHubDataExchange {
        let out = BigqueryAnalyticsHubDataExchange(Rc::new(BigqueryAnalyticsHubDataExchange_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BigqueryAnalyticsHubDataExchangeData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                data_exchange_id: self.data_exchange_id,
                description: core::default::Default::default(),
                display_name: self.display_name,
                documentation: core::default::Default::default(),
                icon: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                primary_contact: core::default::Default::default(),
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryAnalyticsHubDataExchangeRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryAnalyticsHubDataExchangeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryAnalyticsHubDataExchangeRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `data_exchange_id` after provisioning.\nThe ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub fn data_exchange_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_exchange_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the data exchange."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and must not start or end with spaces."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the data exchange."]
    pub fn documentation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the data exchange."]
    pub fn icon(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.icon", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `listing_count` after provisioning.\nNumber of listings contained in the data exchange."]
    pub fn listing_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.listing_count", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource name of the data exchange, for example:\n\"projects/myproject/locations/US/dataExchanges/123\""]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the data exchange."]
    pub fn primary_contact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
        BigqueryAnalyticsHubDataExchangeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubDataExchangeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl BigqueryAnalyticsHubDataExchangeTimeoutsEl {
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

impl ToListMappable for BigqueryAnalyticsHubDataExchangeTimeoutsEl {
    type O = BlockAssignable<BigqueryAnalyticsHubDataExchangeTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubDataExchangeTimeoutsEl {}

impl BuildBigqueryAnalyticsHubDataExchangeTimeoutsEl {
    pub fn build(self) -> BigqueryAnalyticsHubDataExchangeTimeoutsEl {
        BigqueryAnalyticsHubDataExchangeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
        BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubDataExchangeTimeoutsElRef {
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
