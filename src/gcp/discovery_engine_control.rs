#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DiscoveryEngineControlData_ {
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
    collection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    control_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    engine_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    solution_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_cases: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost_action: std::option::Option<std::vec::Vec<DiscoveryEngineControl_BoostAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<std::vec::Vec<DiscoveryEngineControl_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_action: std::option::Option<std::vec::Vec<DiscoveryEngineControl_FilterAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    promote_action: std::option::Option<std::vec::Vec<DiscoveryEngineControl_PromoteAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_action: std::option::Option<std::vec::Vec<DiscoveryEngineControl_RedirectAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    synonyms_action: std::option::Option<std::vec::Vec<DiscoveryEngineControl_SynonymsAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DiscoveryEngineControl_Timeouts>,
    dynamic: DiscoveryEngineControlDynamic,
}

struct DiscoveryEngineControl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DiscoveryEngineControlData_>,
}

#[derive(Clone)]
pub struct DiscoveryEngineControl(std::rc::Rc<DiscoveryEngineControl_>);

impl DiscoveryEngineControl {
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

    #[doc = "Set the field `collection_id`.\nThe collection ID. Currently only accepts \"default_collection\"."]
    pub fn set_collection_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().collection_id = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `use_cases`.\nThe use cases that the control is used for. Possible values: [\"SEARCH_USE_CASE_SEARCH\", \"SEARCH_USE_CASE_BROWSE\"]"]
    pub fn set_use_cases(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().use_cases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boost_action`.\n"]
    pub fn set_boost_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineControl_BoostAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().boost_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.boost_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineControl_Conditions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter_action`.\n"]
    pub fn set_filter_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineControl_FilterAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `promote_action`.\n"]
    pub fn set_promote_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineControl_PromoteAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().promote_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.promote_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect_action`.\n"]
    pub fn set_redirect_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineControl_RedirectAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redirect_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redirect_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `synonyms_action`.\n"]
    pub fn set_synonyms_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineControl_SynonymsAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().synonyms_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.synonyms_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DiscoveryEngineControl_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID. Currently only accepts \"default_collection\"."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_id` after provisioning.\nThe unique id of the control."]
    pub fn control_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the control. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe engine to add the control to."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the control. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/controls/{control_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `solution_type` after provisioning.\nThe solution type that the control belongs to. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub fn solution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.solution_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `use_cases` after provisioning.\nThe use cases that the control is used for. Possible values: [\"SEARCH_USE_CASE_SEARCH\", \"SEARCH_USE_CASE_BROWSE\"]"]
    pub fn use_cases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.use_cases", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boost_action` after provisioning.\n"]
    pub fn boost_action(&self) -> terrars::ListRef<DiscoveryEngineControl_BoostActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boost_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<DiscoveryEngineControl_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_action` after provisioning.\n"]
    pub fn filter_action(&self) -> terrars::ListRef<DiscoveryEngineControl_FilterActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `promote_action` after provisioning.\n"]
    pub fn promote_action(&self) -> terrars::ListRef<DiscoveryEngineControl_PromoteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.promote_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_action` after provisioning.\n"]
    pub fn redirect_action(&self) -> terrars::ListRef<DiscoveryEngineControl_RedirectActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `synonyms_action` after provisioning.\n"]
    pub fn synonyms_action(&self) -> terrars::ListRef<DiscoveryEngineControl_SynonymsActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.synonyms_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineControl_TimeoutsRef {
        DiscoveryEngineControl_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DiscoveryEngineControl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DiscoveryEngineControl { }

impl terrars::ToListMappable for DiscoveryEngineControl {
    type O = terrars::ListRef<DiscoveryEngineControlRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DiscoveryEngineControl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_discovery_engine_control".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDiscoveryEngineControl {
    pub tf_id: std::string::String,
    #[doc = "The unique id of the control."]
    pub control_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The display name of the control. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The engine to add the control to."]
    pub engine_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The solution type that the control belongs to. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub solution_type: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineControl {
    pub fn build(self, stack: &mut terrars::Stack) -> DiscoveryEngineControl {
        let out = DiscoveryEngineControl(std::rc::Rc::new(DiscoveryEngineControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DiscoveryEngineControlData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                collection_id: core::default::Default::default(),
                control_id: self.control_id,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                engine_id: self.engine_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                solution_type: self.solution_type,
                use_cases: core::default::Default::default(),
                boost_action: core::default::Default::default(),
                conditions: core::default::Default::default(),
                filter_action: core::default::Default::default(),
                promote_action: core::default::Default::default(),
                redirect_action: core::default::Default::default(),
                synonyms_action: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DiscoveryEngineControlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DiscoveryEngineControlRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nThe collection ID. Currently only accepts \"default_collection\"."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `control_id` after provisioning.\nThe unique id of the control."]
    pub fn control_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.control_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the control. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_id` after provisioning.\nThe engine to add the control to."]
    pub fn engine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the control. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/engines/{engine_id}/controls/{control_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `solution_type` after provisioning.\nThe solution type that the control belongs to. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub fn solution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.solution_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `use_cases` after provisioning.\nThe use cases that the control is used for. Possible values: [\"SEARCH_USE_CASE_SEARCH\", \"SEARCH_USE_CASE_BROWSE\"]"]
    pub fn use_cases(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.use_cases", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boost_action` after provisioning.\n"]
    pub fn boost_action(&self) -> terrars::ListRef<DiscoveryEngineControl_BoostActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boost_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<DiscoveryEngineControl_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_action` after provisioning.\n"]
    pub fn filter_action(&self) -> terrars::ListRef<DiscoveryEngineControl_FilterActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `promote_action` after provisioning.\n"]
    pub fn promote_action(&self) -> terrars::ListRef<DiscoveryEngineControl_PromoteActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.promote_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_action` after provisioning.\n"]
    pub fn redirect_action(&self) -> terrars::ListRef<DiscoveryEngineControl_RedirectActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `synonyms_action` after provisioning.\n"]
    pub fn synonyms_action(&self) -> terrars::ListRef<DiscoveryEngineControl_SynonymsActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.synonyms_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineControl_TimeoutsRef {
        DiscoveryEngineControl_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineControlDynamic {
    boost_action: std::option::Option<terrars::DynamicBlock<DiscoveryEngineControl_BoostAction>>,
    conditions: std::option::Option<terrars::DynamicBlock<DiscoveryEngineControl_Conditions>>,
    filter_action: std::option::Option<terrars::DynamicBlock<DiscoveryEngineControl_FilterAction>>,
    promote_action: std::option::Option<terrars::DynamicBlock<DiscoveryEngineControl_PromoteAction>>,
    redirect_action: std::option::Option<terrars::DynamicBlock<DiscoveryEngineControl_RedirectAction>>,
    synonyms_action: std::option::Option<terrars::DynamicBlock<DiscoveryEngineControl_SynonymsAction>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost_amount: std::option::Option<terrars::PrimField<f64>>,
}

impl DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPoint {
    #[doc = "Set the field `attribute_value`.\nThe attribute value of the control point."]
    pub fn set_attribute_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_value = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `boost_amount`.\nThe value between -1 to 1 by which to boost the score if the attributeValue\nevaluates to the value specified above."]
    pub fn set_boost_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boost_amount = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPoint {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPoint {}

impl BuildDiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPoint {
    pub fn build(self) -> DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPoint {
        DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPoint {
            attribute_value: core::default::Default::default(),
            boost_amount: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPointRef {
        DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_value` after provisioning.\nThe attribute value of the control point."]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `boost_amount` after provisioning.\nThe value between -1 to 1 by which to boost the score if the attributeValue\nevaluates to the value specified above."]
    pub fn boost_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boost_amount", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineControl_BoostAction_InterpolationBoostSpecDynamic {
    control_point: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPoint>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_BoostAction_InterpolationBoostSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interpolation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_point: std
    ::option
    ::Option<std::vec::Vec<DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPoint>>,
    dynamic: DiscoveryEngineControl_BoostAction_InterpolationBoostSpecDynamic,
}

impl DiscoveryEngineControl_BoostAction_InterpolationBoostSpec {
    #[doc =
        "Set the field `attribute_type`.\nThe attribute type to be used to determine the boost amount. Possible values: [\"NUMERICAL\", \"FRESHNESS\"]"]
    pub fn set_attribute_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `field_name`.\nThe name of the field whose value will be used to determine the boost amount."]
    pub fn set_field_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `interpolation_type`.\nThe interpolation type to be applied to connect the control points. Possible values: [\"LINEAR\"]"]
    pub fn set_interpolation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.interpolation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_point`.\n"]
    pub fn set_control_point(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPoint>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.control_point = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.control_point = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineControl_BoostAction_InterpolationBoostSpec {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_BoostAction_InterpolationBoostSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_BoostAction_InterpolationBoostSpec {}

impl BuildDiscoveryEngineControl_BoostAction_InterpolationBoostSpec {
    pub fn build(self) -> DiscoveryEngineControl_BoostAction_InterpolationBoostSpec {
        DiscoveryEngineControl_BoostAction_InterpolationBoostSpec {
            attribute_type: core::default::Default::default(),
            field_name: core::default::Default::default(),
            interpolation_type: core::default::Default::default(),
            control_point: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_BoostAction_InterpolationBoostSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_BoostAction_InterpolationBoostSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineControl_BoostAction_InterpolationBoostSpecRef {
        DiscoveryEngineControl_BoostAction_InterpolationBoostSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_BoostAction_InterpolationBoostSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute_type` after provisioning.\nThe attribute type to be used to determine the boost amount. Possible values: [\"NUMERICAL\", \"FRESHNESS\"]"]
    pub fn attribute_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field_name` after provisioning.\nThe name of the field whose value will be used to determine the boost amount."]
    pub fn field_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `interpolation_type` after provisioning.\nThe interpolation type to be applied to connect the control points. Possible values: [\"LINEAR\"]"]
    pub fn interpolation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpolation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `control_point` after provisioning.\n"]
    pub fn control_point(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineControl_BoostAction_InterpolationBoostSpec_ControlPointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_point", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineControl_BoostActionDynamic {
    interpolation_boost_spec: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineControl_BoostAction_InterpolationBoostSpec>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_BoostAction {
    data_store: terrars::PrimField<std::string::String>,
    filter: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_boost: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interpolation_boost_spec: std
    ::option
    ::Option<std::vec::Vec<DiscoveryEngineControl_BoostAction_InterpolationBoostSpec>>,
    dynamic: DiscoveryEngineControl_BoostActionDynamic,
}

impl DiscoveryEngineControl_BoostAction {
    #[doc =
        "Set the field `fixed_boost`.\nThe fixed boost value to apply to the search results. Positive values will increase the relevance of the results, while negative values will decrease the relevance. The value must be between -100 and 100."]
    pub fn set_fixed_boost(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.fixed_boost = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interpolation_boost_spec`.\n"]
    pub fn set_interpolation_boost_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DiscoveryEngineControl_BoostAction_InterpolationBoostSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.interpolation_boost_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.interpolation_boost_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineControl_BoostAction {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_BoostAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_BoostAction {
    #[doc = "The data store to boost."]
    pub data_store: terrars::PrimField<std::string::String>,
    #[doc = "The filter to apply to the search results."]
    pub filter: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineControl_BoostAction {
    pub fn build(self) -> DiscoveryEngineControl_BoostAction {
        DiscoveryEngineControl_BoostAction {
            data_store: self.data_store,
            filter: self.filter,
            fixed_boost: core::default::Default::default(),
            interpolation_boost_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_BoostActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_BoostActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineControl_BoostActionRef {
        DiscoveryEngineControl_BoostActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_BoostActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_store` after provisioning.\nThe data store to boost."]
    pub fn data_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply to the search results."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fixed_boost` after provisioning.\nThe fixed boost value to apply to the search results. Positive values will increase the relevance of the results, while negative values will decrease the relevance. The value must be between -100 and 100."]
    pub fn fixed_boost(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_boost", self.base))
    }

    #[doc = "Get a reference to the value of field `interpolation_boost_spec` after provisioning.\n"]
    pub fn interpolation_boost_spec(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineControl_BoostAction_InterpolationBoostSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.interpolation_boost_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_Conditions_ActiveTimeRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineControl_Conditions_ActiveTimeRange {
    #[doc = "Set the field `end_time`.\nThe end time of the active time range."]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\nThe start time of the active time range."]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineControl_Conditions_ActiveTimeRange {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_Conditions_ActiveTimeRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_Conditions_ActiveTimeRange {}

impl BuildDiscoveryEngineControl_Conditions_ActiveTimeRange {
    pub fn build(self) -> DiscoveryEngineControl_Conditions_ActiveTimeRange {
        DiscoveryEngineControl_Conditions_ActiveTimeRange {
            end_time: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_Conditions_ActiveTimeRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_Conditions_ActiveTimeRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineControl_Conditions_ActiveTimeRangeRef {
        DiscoveryEngineControl_Conditions_ActiveTimeRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_Conditions_ActiveTimeRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `end_time` after provisioning.\nThe end time of the active time range."]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `start_time` after provisioning.\nThe start time of the active time range."]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_Conditions_QueryTerms {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    full_match: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineControl_Conditions_QueryTerms {
    #[doc =
        "Set the field `full_match`.\nIf true, the query term must be an exact match. Otherwise, the query term can be a partial match."]
    pub fn set_full_match(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.full_match = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe value of the query term."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineControl_Conditions_QueryTerms {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_Conditions_QueryTerms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_Conditions_QueryTerms {}

impl BuildDiscoveryEngineControl_Conditions_QueryTerms {
    pub fn build(self) -> DiscoveryEngineControl_Conditions_QueryTerms {
        DiscoveryEngineControl_Conditions_QueryTerms {
            full_match: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_Conditions_QueryTermsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_Conditions_QueryTermsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineControl_Conditions_QueryTermsRef {
        DiscoveryEngineControl_Conditions_QueryTermsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_Conditions_QueryTermsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `full_match` after provisioning.\nIf true, the query term must be an exact match. Otherwise, the query term can be a partial match."]
    pub fn full_match(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.full_match", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the query term."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineControl_ConditionsDynamic {
    active_time_range: std::option::Option<terrars::DynamicBlock<DiscoveryEngineControl_Conditions_ActiveTimeRange>>,
    query_terms: std::option::Option<terrars::DynamicBlock<DiscoveryEngineControl_Conditions_QueryTerms>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_time_range: std::option::Option<std::vec::Vec<DiscoveryEngineControl_Conditions_ActiveTimeRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_terms: std::option::Option<std::vec::Vec<DiscoveryEngineControl_Conditions_QueryTerms>>,
    dynamic: DiscoveryEngineControl_ConditionsDynamic,
}

impl DiscoveryEngineControl_Conditions {
    #[doc =
        "Set the field `query_regex`.\nThe regular expression that the query must match for this condition to be met."]
    pub fn set_query_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.query_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `active_time_range`.\n"]
    pub fn set_active_time_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineControl_Conditions_ActiveTimeRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.active_time_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.active_time_range = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_terms`.\n"]
    pub fn set_query_terms(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineControl_Conditions_QueryTerms>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_terms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_terms = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineControl_Conditions {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_Conditions {}

impl BuildDiscoveryEngineControl_Conditions {
    pub fn build(self) -> DiscoveryEngineControl_Conditions {
        DiscoveryEngineControl_Conditions {
            query_regex: core::default::Default::default(),
            active_time_range: core::default::Default::default(),
            query_terms: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_ConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineControl_ConditionsRef {
        DiscoveryEngineControl_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `query_regex` after provisioning.\nThe regular expression that the query must match for this condition to be met."]
    pub fn query_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `active_time_range` after provisioning.\n"]
    pub fn active_time_range(&self) -> terrars::ListRef<DiscoveryEngineControl_Conditions_ActiveTimeRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.active_time_range", self.base))
    }

    #[doc = "Get a reference to the value of field `query_terms` after provisioning.\n"]
    pub fn query_terms(&self) -> terrars::ListRef<DiscoveryEngineControl_Conditions_QueryTermsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_terms", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_FilterAction {
    data_store: terrars::PrimField<std::string::String>,
    filter: terrars::PrimField<std::string::String>,
}

impl DiscoveryEngineControl_FilterAction { }

impl terrars::ToListMappable for DiscoveryEngineControl_FilterAction {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_FilterAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_FilterAction {
    #[doc = "The data store to filter."]
    pub data_store: terrars::PrimField<std::string::String>,
    #[doc = "The filter to apply to the search results."]
    pub filter: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineControl_FilterAction {
    pub fn build(self) -> DiscoveryEngineControl_FilterAction {
        DiscoveryEngineControl_FilterAction {
            data_store: self.data_store,
            filter: self.filter,
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_FilterActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_FilterActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineControl_FilterActionRef {
        DiscoveryEngineControl_FilterActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_FilterActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_store` after provisioning.\nThe data store to filter."]
    pub fn data_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nThe filter to apply to the search results."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_PromoteAction_SearchLinkPromotion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    title: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineControl_PromoteAction_SearchLinkPromotion {
    #[doc = "Set the field `description`.\nThe description of the promoted link."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `document`.\nThe document to promote."]
    pub fn set_document(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.document = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\nReturn promotions for basic site search."]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_uri`.\nThe image URI of the promoted link."]
    pub fn set_image_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uri`.\nThe URI to promote."]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineControl_PromoteAction_SearchLinkPromotion {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_PromoteAction_SearchLinkPromotion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_PromoteAction_SearchLinkPromotion {
    #[doc = "The title of the promoted link."]
    pub title: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineControl_PromoteAction_SearchLinkPromotion {
    pub fn build(self) -> DiscoveryEngineControl_PromoteAction_SearchLinkPromotion {
        DiscoveryEngineControl_PromoteAction_SearchLinkPromotion {
            description: core::default::Default::default(),
            document: core::default::Default::default(),
            enabled: core::default::Default::default(),
            image_uri: core::default::Default::default(),
            title: self.title,
            uri: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_PromoteAction_SearchLinkPromotionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_PromoteAction_SearchLinkPromotionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineControl_PromoteAction_SearchLinkPromotionRef {
        DiscoveryEngineControl_PromoteAction_SearchLinkPromotionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_PromoteAction_SearchLinkPromotionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the promoted link."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `document` after provisioning.\nThe document to promote."]
    pub fn document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nReturn promotions for basic site search."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image_uri` after provisioning.\nThe image URI of the promoted link."]
    pub fn image_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\nThe title of the promoted link."]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nThe URI to promote."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineControl_PromoteActionDynamic {
    search_link_promotion: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineControl_PromoteAction_SearchLinkPromotion>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_PromoteAction {
    data_store: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    search_link_promotion: std::option::Option<std::vec::Vec<DiscoveryEngineControl_PromoteAction_SearchLinkPromotion>>,
    dynamic: DiscoveryEngineControl_PromoteActionDynamic,
}

impl DiscoveryEngineControl_PromoteAction {
    #[doc = "Set the field `search_link_promotion`.\n"]
    pub fn set_search_link_promotion(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DiscoveryEngineControl_PromoteAction_SearchLinkPromotion>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.search_link_promotion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.search_link_promotion = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineControl_PromoteAction {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_PromoteAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_PromoteAction {
    #[doc = "The data store to promote."]
    pub data_store: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineControl_PromoteAction {
    pub fn build(self) -> DiscoveryEngineControl_PromoteAction {
        DiscoveryEngineControl_PromoteAction {
            data_store: self.data_store,
            search_link_promotion: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_PromoteActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_PromoteActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineControl_PromoteActionRef {
        DiscoveryEngineControl_PromoteActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_PromoteActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_store` after provisioning.\nThe data store to promote."]
    pub fn data_store(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store", self.base))
    }

    #[doc = "Get a reference to the value of field `search_link_promotion` after provisioning.\n"]
    pub fn search_link_promotion(&self) -> terrars::ListRef<DiscoveryEngineControl_PromoteAction_SearchLinkPromotionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.search_link_promotion", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_RedirectAction {
    redirect_uri: terrars::PrimField<std::string::String>,
}

impl DiscoveryEngineControl_RedirectAction { }

impl terrars::ToListMappable for DiscoveryEngineControl_RedirectAction {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_RedirectAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_RedirectAction {
    #[doc = "The URI to redirect to."]
    pub redirect_uri: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineControl_RedirectAction {
    pub fn build(self) -> DiscoveryEngineControl_RedirectAction {
        DiscoveryEngineControl_RedirectAction { redirect_uri: self.redirect_uri }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_RedirectActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_RedirectActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineControl_RedirectActionRef {
        DiscoveryEngineControl_RedirectActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_RedirectActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `redirect_uri` after provisioning.\nThe URI to redirect to."]
    pub fn redirect_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_SynonymsAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    synonyms: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DiscoveryEngineControl_SynonymsAction {
    #[doc = "Set the field `synonyms`.\nThe synonyms to apply to the search results."]
    pub fn set_synonyms(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.synonyms = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineControl_SynonymsAction {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_SynonymsAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_SynonymsAction {}

impl BuildDiscoveryEngineControl_SynonymsAction {
    pub fn build(self) -> DiscoveryEngineControl_SynonymsAction {
        DiscoveryEngineControl_SynonymsAction { synonyms: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_SynonymsActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_SynonymsActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineControl_SynonymsActionRef {
        DiscoveryEngineControl_SynonymsActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_SynonymsActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `synonyms` after provisioning.\nThe synonyms to apply to the search results."]
    pub fn synonyms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.synonyms", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineControl_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineControl_Timeouts {
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

impl terrars::ToListMappable for DiscoveryEngineControl_Timeouts {
    type O = terrars::BlockAssignable<DiscoveryEngineControl_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineControl_Timeouts {}

impl BuildDiscoveryEngineControl_Timeouts {
    pub fn build(self) -> DiscoveryEngineControl_Timeouts {
        DiscoveryEngineControl_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineControl_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineControl_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineControl_TimeoutsRef {
        DiscoveryEngineControl_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineControl_TimeoutsRef {
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
