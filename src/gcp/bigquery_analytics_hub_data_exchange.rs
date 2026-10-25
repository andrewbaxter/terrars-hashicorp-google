#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryAnalyticsHubDataExchangeData_ {
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
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_linked_dataset_query_user_email: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_contact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sharing_environment_config: std
    ::option
    ::Option<std::vec::Vec<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigqueryAnalyticsHubDataExchange_Timeouts>,
    dynamic: BigqueryAnalyticsHubDataExchangeDynamic,
}

struct BigqueryAnalyticsHubDataExchange_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryAnalyticsHubDataExchangeData_>,
}

#[derive(Clone)]
pub struct BigqueryAnalyticsHubDataExchange(std::rc::Rc<BigqueryAnalyticsHubDataExchange_>);

impl BigqueryAnalyticsHubDataExchange {
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

    #[doc = "Set the field `description`.\nDescription of the data exchange."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `discovery_type`.\nType of discovery on the discovery page for all the listings under this exchange. Cannot be set for a Data Clean Room. Updating this field also updates (overwrites) the discoveryType field for all the listings under this exchange. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn set_discovery_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().discovery_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `documentation`.\nDocumentation describing the data exchange."]
    pub fn set_documentation(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().documentation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `icon`.\nBase64 encoded image representing the data exchange."]
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

    #[doc = "Set the field `primary_contact`.\nEmail or URL of the primary point of contact of the data exchange."]
    pub fn set_primary_contact(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().primary_contact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sharing_environment_config`.\n"]
    pub fn set_sharing_environment_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sharing_environment_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sharing_environment_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigqueryAnalyticsHubDataExchange_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
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

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the data exchange."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_type` after provisioning.\nType of discovery on the discovery page for all the listings under this exchange. Cannot be set for a Data Clean Room. Updating this field also updates (overwrites) the discoveryType field for all the listings under this exchange. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn discovery_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and must not start or end with spaces."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the data exchange."]
    pub fn documentation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the data exchange."]
    pub fn icon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icon", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listing_count` after provisioning.\nNumber of listings contained in the data exchange."]
    pub fn listing_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listing_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange."]
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
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the data exchange, for example:\n\"projects/myproject/locations/US/dataExchanges/123\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the data exchange."]
    pub fn primary_contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sharing_environment_config` after provisioning.\n"]
    pub fn sharing_environment_config(
        &self,
    ) -> terrars::ListRef<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sharing_environment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubDataExchange_TimeoutsRef {
        BigqueryAnalyticsHubDataExchange_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BigqueryAnalyticsHubDataExchange {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigqueryAnalyticsHubDataExchange { }

impl terrars::ToListMappable for BigqueryAnalyticsHubDataExchange {
    type O = terrars::ListRef<BigqueryAnalyticsHubDataExchangeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigqueryAnalyticsHubDataExchange_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigquery_analytics_hub_data_exchange".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryAnalyticsHubDataExchange {
    pub tf_id: std::string::String,
    #[doc =
        "The ID of the data exchange. Must contain only Unicode letters, numbers (0-9), underscores (_). Should not use characters that require URL-escaping, or characters outside of ASCII, spaces."]
    pub data_exchange_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Human-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and must not start or end with spaces."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The name of the location this data exchange."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryAnalyticsHubDataExchange {
    pub fn build(self, stack: &mut terrars::Stack) -> BigqueryAnalyticsHubDataExchange {
        let out = BigqueryAnalyticsHubDataExchange(std::rc::Rc::new(BigqueryAnalyticsHubDataExchange_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigqueryAnalyticsHubDataExchangeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_exchange_id: self.data_exchange_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                discovery_type: core::default::Default::default(),
                display_name: self.display_name,
                documentation: core::default::Default::default(),
                icon: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                log_linked_dataset_query_user_email: core::default::Default::default(),
                primary_contact: core::default::Default::default(),
                project: core::default::Default::default(),
                sharing_environment_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryAnalyticsHubDataExchangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubDataExchangeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryAnalyticsHubDataExchangeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the data exchange."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovery_type` after provisioning.\nType of discovery on the discovery page for all the listings under this exchange. Cannot be set for a Data Clean Room. Updating this field also updates (overwrites) the discoveryType field for all the listings under this exchange. Possible values: [\"DISCOVERY_TYPE_PRIVATE\", \"DISCOVERY_TYPE_PUBLIC\"]"]
    pub fn discovery_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of the data exchange. The display name must contain only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces ( ), and must not start or end with spaces."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `documentation` after provisioning.\nDocumentation describing the data exchange."]
    pub fn documentation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `icon` after provisioning.\nBase64 encoded image representing the data exchange."]
    pub fn icon(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.icon", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `listing_count` after provisioning.\nNumber of listings contained in the data exchange."]
    pub fn listing_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listing_count", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this data exchange."]
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
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the data exchange, for example:\n\"projects/myproject/locations/US/dataExchanges/123\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `primary_contact` after provisioning.\nEmail or URL of the primary point of contact of the data exchange."]
    pub fn primary_contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_contact", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sharing_environment_config` after provisioning.\n"]
    pub fn sharing_environment_config(
        &self,
    ) -> terrars::ListRef<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sharing_environment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryAnalyticsHubDataExchange_TimeoutsRef {
        BigqueryAnalyticsHubDataExchange_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubDataExchangeDynamic {
    sharing_environment_config: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig>>,
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfig {}

impl BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfig { }

impl terrars::ToListMappable for BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfig {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfig {}

impl BuildBigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfig {
    pub fn build(self) -> BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfig {
        BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfig {}
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfigRef {
        BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfig {}

impl BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfig { }

impl terrars::ToListMappable for BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfig {
    type O =
        terrars::BlockAssignable<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfig {}

impl BuildBigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfig {
    pub fn build(self) -> BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfig {
        BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfig {}
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfigRef {
        BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfigDynamic {
    dcr_exchange_config: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfig>>,
    default_exchange_config: std
    ::option
    ::Option<terrars::DynamicBlock<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfig>>,
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dcr_exchange_config: std
    ::option
    ::Option<std::vec::Vec<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_exchange_config: std
    ::option
    ::Option<std::vec::Vec<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfig>>,
    dynamic: BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfigDynamic,
}

impl BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig {
    #[doc = "Set the field `dcr_exchange_config`.\n"]
    pub fn set_dcr_exchange_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dcr_exchange_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dcr_exchange_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_exchange_config`.\n"]
    pub fn set_default_exchange_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_exchange_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_exchange_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig {}

impl BuildBigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig {
    pub fn build(self) -> BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig {
        BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig {
            dcr_exchange_config: core::default::Default::default(),
            default_exchange_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfigRef {
        BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dcr_exchange_config` after provisioning.\n"]
    pub fn dcr_exchange_config(
        &self,
    ) -> terrars::ListRef<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DcrExchangeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dcr_exchange_config", self.base))
    }

    #[doc = "Get a reference to the value of field `default_exchange_config` after provisioning.\n"]
    pub fn default_exchange_config(
        &self,
    ) -> terrars::ListRef<BigqueryAnalyticsHubDataExchange_SharingEnvironmentConfig_DefaultExchangeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_exchange_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryAnalyticsHubDataExchange_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryAnalyticsHubDataExchange_Timeouts {
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

impl terrars::ToListMappable for BigqueryAnalyticsHubDataExchange_Timeouts {
    type O = terrars::BlockAssignable<BigqueryAnalyticsHubDataExchange_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryAnalyticsHubDataExchange_Timeouts {}

impl BuildBigqueryAnalyticsHubDataExchange_Timeouts {
    pub fn build(self) -> BigqueryAnalyticsHubDataExchange_Timeouts {
        BigqueryAnalyticsHubDataExchange_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryAnalyticsHubDataExchange_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryAnalyticsHubDataExchange_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryAnalyticsHubDataExchange_TimeoutsRef {
        BigqueryAnalyticsHubDataExchange_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryAnalyticsHubDataExchange_TimeoutsRef {
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
