#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleNativeDashboardData_ {
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
    access: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_pinned: std::option::Option<terrars::PrimField<bool>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    charts: std::option::Option<std::vec::Vec<ChronicleNativeDashboard_Charts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std::option::Option<std::vec::Vec<ChronicleNativeDashboard_Filters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleNativeDashboard_Timeouts>,
    dynamic: ChronicleNativeDashboardDynamic,
}

struct ChronicleNativeDashboard_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleNativeDashboardData_>,
}

#[derive(Clone)]
pub struct ChronicleNativeDashboard(std::rc::Rc<ChronicleNativeDashboard_>);

impl ChronicleNativeDashboard {
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
        "Set the field `access`.\nThe access level of the dashboard.\nPossible values:\nDASHBOARD_PRIVATE\nDASHBOARD_PUBLIC"]
    pub fn set_access(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nA description of the dashboard."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_pinned`.\nWhether the dashboard is pinned by the user."]
    pub fn set_is_pinned(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_pinned = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe type of dashboard.\nPossible values:\nCURATED, PRIVATE, PUBLIC, CUSTOM, MARKETPLACE"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `charts`.\n"]
    pub fn set_charts(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleNativeDashboard_Charts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().charts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.charts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleNativeDashboard_Filters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleNativeDashboard_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `access` after provisioning.\nThe access level of the dashboard.\nPossible values:\nDASHBOARD_PRIVATE\nDASHBOARD_PUBLIC"]
    pub fn access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe creation time of the dashboard."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_user_id` after provisioning.\nThe ID of the user who created the dashboard."]
    pub fn create_user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_id` after provisioning.\nThe unique ID of the Dashboard."]
    pub fn dashboard_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the dashboard."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name/title of the dashboard visible to users."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum for optimistic concurrency control,\nsent on update and delete requests."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nThe server-generated fingerprint of the dashboard definition."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe ID of the Chronicle instance."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_pinned` after provisioning.\nWhether the dashboard is pinned by the user."]
    pub fn is_pinned(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_pinned", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_viewed_time` after provisioning.\nThe time when this dashboard was last viewed."]
    pub fn last_viewed_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_viewed_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the Chronicle instance."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the dashboard."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of dashboard.\nPossible values:\nCURATED, PRIVATE, PUBLIC, CUSTOM, MARKETPLACE"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the dashboard was last edited."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_user_id` after provisioning.\nThe ID of the user who last edited the dashboard."]
    pub fn update_user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `charts` after provisioning.\n"]
    pub fn charts(&self) -> terrars::ListRef<ChronicleNativeDashboard_ChartsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.charts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<ChronicleNativeDashboard_FiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleNativeDashboard_TimeoutsRef {
        ChronicleNativeDashboard_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleNativeDashboard {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleNativeDashboard { }

impl terrars::ToListMappable for ChronicleNativeDashboard {
    type O = terrars::ListRef<ChronicleNativeDashboardRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleNativeDashboard_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_native_dashboard".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleNativeDashboard {
    pub tf_id: std::string::String,
    #[doc = "The display name/title of the dashboard visible to users."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The ID of the Chronicle instance."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc = "The location of the Chronicle instance."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildChronicleNativeDashboard {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleNativeDashboard {
        let out = ChronicleNativeDashboard(std::rc::Rc::new(ChronicleNativeDashboard_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleNativeDashboardData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                instance: self.instance,
                is_pinned: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                type_: core::default::Default::default(),
                charts: core::default::Default::default(),
                filters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleNativeDashboardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleNativeDashboardRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleNativeDashboardRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `access` after provisioning.\nThe access level of the dashboard.\nPossible values:\nDASHBOARD_PRIVATE\nDASHBOARD_PUBLIC"]
    pub fn access(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe creation time of the dashboard."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_user_id` after provisioning.\nThe ID of the user who created the dashboard."]
    pub fn create_user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dashboard_id` after provisioning.\nThe unique ID of the Dashboard."]
    pub fn dashboard_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the dashboard."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name/title of the dashboard visible to users."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nServer-computed checksum for optimistic concurrency control,\nsent on update and delete requests."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fingerprint` after provisioning.\nThe server-generated fingerprint of the dashboard definition."]
    pub fn fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance` after provisioning.\nThe ID of the Chronicle instance."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `is_pinned` after provisioning.\nWhether the dashboard is pinned by the user."]
    pub fn is_pinned(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_pinned", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_viewed_time` after provisioning.\nThe time when this dashboard was last viewed."]
    pub fn last_viewed_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_viewed_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the Chronicle instance."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the dashboard."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of dashboard.\nPossible values:\nCURATED, PRIVATE, PUBLIC, CUSTOM, MARKETPLACE"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time when the dashboard was last edited."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_user_id` after provisioning.\nThe ID of the user who last edited the dashboard."]
    pub fn update_user_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_user_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `charts` after provisioning.\n"]
    pub fn charts(&self) -> terrars::ListRef<ChronicleNativeDashboard_ChartsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.charts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<ChronicleNativeDashboard_FiltersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleNativeDashboard_TimeoutsRef {
        ChronicleNativeDashboard_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleNativeDashboardDynamic {
    charts: std::option::Option<terrars::DynamicBlock<ChronicleNativeDashboard_Charts>>,
    filters: std::option::Option<terrars::DynamicBlock<ChronicleNativeDashboard_Filters>>,
}

#[derive(Serialize)]
pub struct ChronicleNativeDashboard_Charts_ChartLayout {
    span_x: terrars::PrimField<f64>,
    span_y: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_x: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_y: std::option::Option<terrars::PrimField<f64>>,
}

impl ChronicleNativeDashboard_Charts_ChartLayout {
    #[doc = "Set the field `start_x`.\nThe starting X coordinate."]
    pub fn set_start_x(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_x = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_y`.\nThe starting Y coordinate."]
    pub fn set_start_y(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_y = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleNativeDashboard_Charts_ChartLayout {
    type O = terrars::BlockAssignable<ChronicleNativeDashboard_Charts_ChartLayout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleNativeDashboard_Charts_ChartLayout {
    #[doc = "The number of columns the chart spans."]
    pub span_x: terrars::PrimField<f64>,
    #[doc = "The number of rows the chart spans."]
    pub span_y: terrars::PrimField<f64>,
}

impl BuildChronicleNativeDashboard_Charts_ChartLayout {
    pub fn build(self) -> ChronicleNativeDashboard_Charts_ChartLayout {
        ChronicleNativeDashboard_Charts_ChartLayout {
            span_x: self.span_x,
            span_y: self.span_y,
            start_x: core::default::Default::default(),
            start_y: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleNativeDashboard_Charts_ChartLayoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleNativeDashboard_Charts_ChartLayoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleNativeDashboard_Charts_ChartLayoutRef {
        ChronicleNativeDashboard_Charts_ChartLayoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleNativeDashboard_Charts_ChartLayoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `span_x` after provisioning.\nThe number of columns the chart spans."]
    pub fn span_x(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.span_x", self.base))
    }

    #[doc = "Get a reference to the value of field `span_y` after provisioning.\nThe number of rows the chart spans."]
    pub fn span_y(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.span_y", self.base))
    }

    #[doc = "Get a reference to the value of field `start_x` after provisioning.\nThe starting X coordinate."]
    pub fn start_x(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_x", self.base))
    }

    #[doc = "Get a reference to the value of field `start_y` after provisioning.\nThe starting Y coordinate."]
    pub fn start_y(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_y", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleNativeDashboard_ChartsDynamic {
    chart_layout: std::option::Option<terrars::DynamicBlock<ChronicleNativeDashboard_Charts_ChartLayout>>,
}

#[derive(Serialize)]
pub struct ChronicleNativeDashboard_Charts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dashboard_chart: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chart_layout: std::option::Option<std::vec::Vec<ChronicleNativeDashboard_Charts_ChartLayout>>,
    dynamic: ChronicleNativeDashboard_ChartsDynamic,
}

impl ChronicleNativeDashboard_Charts {
    #[doc = "Set the field `dashboard_chart`.\nThe resource name of the associated DashboardChart."]
    pub fn set_dashboard_chart(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dashboard_chart = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filters_ids`.\nList of dashboard filter IDs applied to this chart."]
    pub fn set_filters_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.filters_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chart_layout`.\n"]
    pub fn set_chart_layout(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleNativeDashboard_Charts_ChartLayout>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.chart_layout = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.chart_layout = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleNativeDashboard_Charts {
    type O = terrars::BlockAssignable<ChronicleNativeDashboard_Charts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleNativeDashboard_Charts {}

impl BuildChronicleNativeDashboard_Charts {
    pub fn build(self) -> ChronicleNativeDashboard_Charts {
        ChronicleNativeDashboard_Charts {
            dashboard_chart: core::default::Default::default(),
            filters_ids: core::default::Default::default(),
            chart_layout: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleNativeDashboard_ChartsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleNativeDashboard_ChartsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleNativeDashboard_ChartsRef {
        ChronicleNativeDashboard_ChartsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleNativeDashboard_ChartsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dashboard_chart` after provisioning.\nThe resource name of the associated DashboardChart."]
    pub fn dashboard_chart(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dashboard_chart", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filters_ids` after provisioning.\nList of dashboard filter IDs applied to this chart."]
    pub fn filters_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `chart_layout` after provisioning.\n"]
    pub fn chart_layout(&self) -> terrars::ListRef<ChronicleNativeDashboard_Charts_ChartLayoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chart_layout", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValues {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_operator: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValues {
    #[doc =
        "Set the field `field_values`.\nThe values for the modifier. All operators should have a single\nvalue other than 'IN' and 'BETWEEN'."]
    pub fn set_field_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.field_values = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `filter_operator`.\nThe operator to apply to the field. Possible values: [\"EQUAL\", \"NOT_EQUAL\", \"IN\", \"GREATER_THAN\", \"GREATER_THAN_OR_EQUAL_TO\", \"LESS_THAN\", \"LESS_THAN_OR_EQUAL_TO\", \"BETWEEN\", \"PAST\", \"IS_NULL\", \"IS_NOT_NULL\", \"STARTS_WITH\", \"ENDS_WITH\", \"DOES_NOT_STARTS_WITH\", \"DOES_NOT_ENDS_WITH\", \"NOT_IN\", \"CONTAINS\", \"DOES_NOT_CONTAIN\"]"]
    pub fn set_filter_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_operator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValues {
    type O = terrars::BlockAssignable<ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleNativeDashboard_Filters_FilterOperatorAndFieldValues {}

impl BuildChronicleNativeDashboard_Filters_FilterOperatorAndFieldValues {
    pub fn build(self) -> ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValues {
        ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValues {
            field_values: core::default::Default::default(),
            filter_operator: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValuesRef {
        ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `field_values` after provisioning.\nThe values for the modifier. All operators should have a single\nvalue other than 'IN' and 'BETWEEN'."]
    pub fn field_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_values", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter_operator` after provisioning.\nThe operator to apply to the field. Possible values: [\"EQUAL\", \"NOT_EQUAL\", \"IN\", \"GREATER_THAN\", \"GREATER_THAN_OR_EQUAL_TO\", \"LESS_THAN\", \"LESS_THAN_OR_EQUAL_TO\", \"BETWEEN\", \"PAST\", \"IS_NULL\", \"IS_NOT_NULL\", \"STARTS_WITH\", \"ENDS_WITH\", \"DOES_NOT_STARTS_WITH\", \"DOES_NOT_ENDS_WITH\", \"NOT_IN\", \"CONTAINS\", \"DOES_NOT_CONTAIN\"]"]
    pub fn filter_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_operator", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleNativeDashboard_FiltersDynamic {
    filter_operator_and_field_values: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValues>>,
}

#[derive(Serialize)]
pub struct ChronicleNativeDashboard_Filters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chart_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_mandatory: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_standard_time_range_filter: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_standard_time_range_filter_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_operator_and_field_values: std
    ::option
    ::Option<std::vec::Vec<ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValues>>,
    dynamic: ChronicleNativeDashboard_FiltersDynamic,
}

impl ChronicleNativeDashboard_Filters {
    #[doc = "Set the field `chart_ids`.\nThe IDs of charts that this filter applies to."]
    pub fn set_chart_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.chart_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_source`.\nThe data source for the filter.\nPossible values:\nUDM, ENTITY, INGESTION_METRICS, RULE_DETECTIONS, RULESETS, GLOBAL,\nIOC_MATCHES, RULES, SOAR_CASES, SOAR_PLAYBOOKS, SOAR_CASE_HISTORY,\nDATA_TABLE, INVESTIGATION, INVESTIGATION_FEEDBACK"]
    pub fn set_data_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe display name of the filter."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_path`.\nThe UDM field path being filtered."]
    pub fn set_field_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\nThe unique ID of the filter."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_mandatory`.\nWhether the filter is mandatory for the dashboard consumer."]
    pub fn set_is_mandatory(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_mandatory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_standard_time_range_filter`.\nWhether the filter is a standard time range filter."]
    pub fn set_is_standard_time_range_filter(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_standard_time_range_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `is_standard_time_range_filter_enabled`.\nWhether the standard time range filter is currently enabled."]
    pub fn set_is_standard_time_range_filter_enabled(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.is_standard_time_range_filter_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_operator_and_field_values`.\n"]
    pub fn set_filter_operator_and_field_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValues>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter_operator_and_field_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter_operator_and_field_values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleNativeDashboard_Filters {
    type O = terrars::BlockAssignable<ChronicleNativeDashboard_Filters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleNativeDashboard_Filters {}

impl BuildChronicleNativeDashboard_Filters {
    pub fn build(self) -> ChronicleNativeDashboard_Filters {
        ChronicleNativeDashboard_Filters {
            chart_ids: core::default::Default::default(),
            data_source: core::default::Default::default(),
            display_name: core::default::Default::default(),
            field_path: core::default::Default::default(),
            id: core::default::Default::default(),
            is_mandatory: core::default::Default::default(),
            is_standard_time_range_filter: core::default::Default::default(),
            is_standard_time_range_filter_enabled: core::default::Default::default(),
            filter_operator_and_field_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleNativeDashboard_FiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleNativeDashboard_FiltersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleNativeDashboard_FiltersRef {
        ChronicleNativeDashboard_FiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleNativeDashboard_FiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `chart_ids` after provisioning.\nThe IDs of charts that this filter applies to."]
    pub fn chart_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chart_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_source` after provisioning.\nThe data source for the filter.\nPossible values:\nUDM, ENTITY, INGESTION_METRICS, RULE_DETECTIONS, RULESETS, GLOBAL,\nIOC_MATCHES, RULES, SOAR_CASES, SOAR_PLAYBOOKS, SOAR_CASE_HISTORY,\nDATA_TABLE, INVESTIGATION, INVESTIGATION_FEEDBACK"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the filter."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field_path` after provisioning.\nThe UDM field path being filtered."]
    pub fn field_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_path", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe unique ID of the filter."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_mandatory` after provisioning.\nWhether the filter is mandatory for the dashboard consumer."]
    pub fn is_mandatory(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_mandatory", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_standard_time_range_filter` after provisioning.\nWhether the filter is a standard time range filter."]
    pub fn is_standard_time_range_filter(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_standard_time_range_filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_standard_time_range_filter_enabled` after provisioning.\nWhether the standard time range filter is currently enabled."]
    pub fn is_standard_time_range_filter_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_standard_time_range_filter_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_operator_and_field_values` after provisioning.\n"]
    pub fn filter_operator_and_field_values(
        &self,
    ) -> terrars::ListRef<ChronicleNativeDashboard_Filters_FilterOperatorAndFieldValuesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_operator_and_field_values", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleNativeDashboard_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleNativeDashboard_Timeouts {
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

impl terrars::ToListMappable for ChronicleNativeDashboard_Timeouts {
    type O = terrars::BlockAssignable<ChronicleNativeDashboard_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleNativeDashboard_Timeouts {}

impl BuildChronicleNativeDashboard_Timeouts {
    pub fn build(self) -> ChronicleNativeDashboard_Timeouts {
        ChronicleNativeDashboard_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleNativeDashboard_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleNativeDashboard_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleNativeDashboard_TimeoutsRef {
        ChronicleNativeDashboard_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleNativeDashboard_TimeoutsRef {
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
