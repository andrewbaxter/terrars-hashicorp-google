#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleWatchlistData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiplying_factor: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    watchlist_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_population_mechanism: std::option::Option<std::vec::Vec<ChronicleWatchlist_EntityPopulationMechanism>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleWatchlist_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    watchlist_user_preferences: std::option::Option<std::vec::Vec<ChronicleWatchlist_WatchlistUserPreferences>>,
    dynamic: ChronicleWatchlistDynamic,
}

struct ChronicleWatchlist_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleWatchlistData_>,
}

#[derive(Clone)]
pub struct ChronicleWatchlist(std::rc::Rc<ChronicleWatchlist_>);

impl ChronicleWatchlist {
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

    #[doc = "Set the field `description`.\nOptional. Description of the watchlist."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `multiplying_factor`.\nOptional. Weight applied to the risk score for entities\nin this watchlist.\nThe default is 1.0 if it is not specified."]
    pub fn set_multiplying_factor(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().multiplying_factor = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `watchlist_id`.\nOptional. The ID to use for the watchlist,\nwhich will become the final component of the watchlist's resource name.\nThis value should be 4-63 characters, and valid characters\nare /a-z-/."]
    pub fn set_watchlist_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().watchlist_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entity_population_mechanism`.\n"]
    pub fn set_entity_population_mechanism(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleWatchlist_EntityPopulationMechanism>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entity_population_mechanism = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entity_population_mechanism = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleWatchlist_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `watchlist_user_preferences`.\n"]
    pub fn set_watchlist_user_preferences(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleWatchlist_WatchlistUserPreferences>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().watchlist_user_preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.watchlist_user_preferences = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time the watchlist was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the watchlist."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. Display name of the watchlist.\nNote that it must be at least one character and less than 63 characters\n(https://google.aip.dev/148)."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entity_count` after provisioning.\nCount of different types of entities in the watchlist."]
    pub fn entity_count(&self) -> terrars::ListRef<ChronicleWatchlist_EntityCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `multiplying_factor` after provisioning.\nOptional. Weight applied to the risk score for entities\nin this watchlist.\nThe default is 1.0 if it is not specified."]
    pub fn multiplying_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multiplying_factor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. Resource name of the watchlist. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/watchlists/{watchlist}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time the watchlist was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `watchlist_id` after provisioning.\nOptional. The ID to use for the watchlist,\nwhich will become the final component of the watchlist's resource name.\nThis value should be 4-63 characters, and valid characters\nare /a-z-/."]
    pub fn watchlist_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.watchlist_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entity_population_mechanism` after provisioning.\n"]
    pub fn entity_population_mechanism(&self) -> terrars::ListRef<ChronicleWatchlist_EntityPopulationMechanismRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_population_mechanism", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleWatchlist_TimeoutsRef {
        ChronicleWatchlist_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `watchlist_user_preferences` after provisioning.\n"]
    pub fn watchlist_user_preferences(&self) -> terrars::ListRef<ChronicleWatchlist_WatchlistUserPreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.watchlist_user_preferences", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleWatchlist {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleWatchlist { }

impl terrars::ToListMappable for ChronicleWatchlist {
    type O = terrars::ListRef<ChronicleWatchlistRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleWatchlist_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_watchlist".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleWatchlist {
    pub tf_id: std::string::String,
    #[doc =
        "Required. Display name of the watchlist.\nNote that it must be at least one character and less than 63 characters\n(https://google.aip.dev/148)."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "The location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildChronicleWatchlist {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleWatchlist {
        let out = ChronicleWatchlist(std::rc::Rc::new(ChronicleWatchlist_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleWatchlistData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                multiplying_factor: core::default::Default::default(),
                project: core::default::Default::default(),
                watchlist_id: core::default::Default::default(),
                entity_population_mechanism: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                watchlist_user_preferences: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleWatchlistRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleWatchlistRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleWatchlistRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Time the watchlist was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. Description of the watchlist."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. Display name of the watchlist.\nNote that it must be at least one character and less than 63 characters\n(https://google.aip.dev/148)."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entity_count` after provisioning.\nCount of different types of entities in the watchlist."]
    pub fn entity_count(&self) -> terrars::ListRef<ChronicleWatchlist_EntityCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe unique identifier for the Chronicle instance, which is the same as the customer ID."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the resource. This is the geographical region where the Chronicle instance resides, such as \"us\" or \"europe-west2\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `multiplying_factor` after provisioning.\nOptional. Weight applied to the risk score for entities\nin this watchlist.\nThe default is 1.0 if it is not specified."]
    pub fn multiplying_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multiplying_factor", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. Resource name of the watchlist. This unique identifier is generated using values provided for the URL parameters.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/watchlists/{watchlist}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Time the watchlist was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `watchlist_id` after provisioning.\nOptional. The ID to use for the watchlist,\nwhich will become the final component of the watchlist's resource name.\nThis value should be 4-63 characters, and valid characters\nare /a-z-/."]
    pub fn watchlist_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.watchlist_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entity_population_mechanism` after provisioning.\n"]
    pub fn entity_population_mechanism(&self) -> terrars::ListRef<ChronicleWatchlist_EntityPopulationMechanismRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_population_mechanism", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleWatchlist_TimeoutsRef {
        ChronicleWatchlist_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `watchlist_user_preferences` after provisioning.\n"]
    pub fn watchlist_user_preferences(&self) -> terrars::ListRef<ChronicleWatchlist_WatchlistUserPreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.watchlist_user_preferences", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleWatchlistDynamic {
    entity_population_mechanism: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleWatchlist_EntityPopulationMechanism>>,
    watchlist_user_preferences: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleWatchlist_WatchlistUserPreferences>>,
}

#[derive(Serialize)]
pub struct ChronicleWatchlist_EntityCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleWatchlist_EntityCount {
    #[doc = "Set the field `asset`.\n"]
    pub fn set_asset(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.asset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user`.\n"]
    pub fn set_user(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.user = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleWatchlist_EntityCount {
    type O = terrars::BlockAssignable<ChronicleWatchlist_EntityCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleWatchlist_EntityCount {}

impl BuildChronicleWatchlist_EntityCount {
    pub fn build(self) -> ChronicleWatchlist_EntityCount {
        ChronicleWatchlist_EntityCount {
            asset: core::default::Default::default(),
            user: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleWatchlist_EntityCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleWatchlist_EntityCountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleWatchlist_EntityCountRef {
        ChronicleWatchlist_EntityCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleWatchlist_EntityCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asset` after provisioning.\n"]
    pub fn asset(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset", self.base))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleWatchlist_EntityPopulationMechanism_Manual {}

impl ChronicleWatchlist_EntityPopulationMechanism_Manual { }

impl terrars::ToListMappable for ChronicleWatchlist_EntityPopulationMechanism_Manual {
    type O = terrars::BlockAssignable<ChronicleWatchlist_EntityPopulationMechanism_Manual>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleWatchlist_EntityPopulationMechanism_Manual {}

impl BuildChronicleWatchlist_EntityPopulationMechanism_Manual {
    pub fn build(self) -> ChronicleWatchlist_EntityPopulationMechanism_Manual {
        ChronicleWatchlist_EntityPopulationMechanism_Manual {}
    }
}

#[allow(dead_code)]
pub struct ChronicleWatchlist_EntityPopulationMechanism_ManualRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleWatchlist_EntityPopulationMechanism_ManualRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleWatchlist_EntityPopulationMechanism_ManualRef {
        ChronicleWatchlist_EntityPopulationMechanism_ManualRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleWatchlist_EntityPopulationMechanism_ManualRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct ChronicleWatchlist_EntityPopulationMechanismDynamic {
    manual: std::option::Option<terrars::DynamicBlock<ChronicleWatchlist_EntityPopulationMechanism_Manual>>,
}

#[derive(Serialize)]
pub struct ChronicleWatchlist_EntityPopulationMechanism {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manual: std::option::Option<std::vec::Vec<ChronicleWatchlist_EntityPopulationMechanism_Manual>>,
    dynamic: ChronicleWatchlist_EntityPopulationMechanismDynamic,
}

impl ChronicleWatchlist_EntityPopulationMechanism {
    #[doc = "Set the field `manual`.\n"]
    pub fn set_manual(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleWatchlist_EntityPopulationMechanism_Manual>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.manual = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.manual = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleWatchlist_EntityPopulationMechanism {
    type O = terrars::BlockAssignable<ChronicleWatchlist_EntityPopulationMechanism>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleWatchlist_EntityPopulationMechanism {}

impl BuildChronicleWatchlist_EntityPopulationMechanism {
    pub fn build(self) -> ChronicleWatchlist_EntityPopulationMechanism {
        ChronicleWatchlist_EntityPopulationMechanism {
            manual: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleWatchlist_EntityPopulationMechanismRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleWatchlist_EntityPopulationMechanismRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleWatchlist_EntityPopulationMechanismRef {
        ChronicleWatchlist_EntityPopulationMechanismRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleWatchlist_EntityPopulationMechanismRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `manual` after provisioning.\n"]
    pub fn manual(&self) -> terrars::ListRef<ChronicleWatchlist_EntityPopulationMechanism_ManualRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manual", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleWatchlist_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleWatchlist_Timeouts {
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

impl terrars::ToListMappable for ChronicleWatchlist_Timeouts {
    type O = terrars::BlockAssignable<ChronicleWatchlist_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleWatchlist_Timeouts {}

impl BuildChronicleWatchlist_Timeouts {
    pub fn build(self) -> ChronicleWatchlist_Timeouts {
        ChronicleWatchlist_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleWatchlist_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleWatchlist_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleWatchlist_TimeoutsRef {
        ChronicleWatchlist_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleWatchlist_TimeoutsRef {
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

#[derive(Serialize)]
pub struct ChronicleWatchlist_WatchlistUserPreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pinned: std::option::Option<terrars::PrimField<bool>>,
}

impl ChronicleWatchlist_WatchlistUserPreferences {
    #[doc = "Set the field `pinned`.\nOptional. Whether the watchlist is pinned on the dashboard."]
    pub fn set_pinned(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.pinned = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleWatchlist_WatchlistUserPreferences {
    type O = terrars::BlockAssignable<ChronicleWatchlist_WatchlistUserPreferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleWatchlist_WatchlistUserPreferences {}

impl BuildChronicleWatchlist_WatchlistUserPreferences {
    pub fn build(self) -> ChronicleWatchlist_WatchlistUserPreferences {
        ChronicleWatchlist_WatchlistUserPreferences { pinned: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ChronicleWatchlist_WatchlistUserPreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleWatchlist_WatchlistUserPreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleWatchlist_WatchlistUserPreferencesRef {
        ChronicleWatchlist_WatchlistUserPreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleWatchlist_WatchlistUserPreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pinned` after provisioning.\nOptional. Whether the watchlist is pinned on the dashboard."]
    pub fn pinned(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pinned", self.base))
    }
}
