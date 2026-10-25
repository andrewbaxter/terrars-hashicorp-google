#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApihubCurationData_ {
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
    curation_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<std::vec::Vec<ApihubCuration_Endpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApihubCuration_Timeouts>,
    dynamic: ApihubCurationDynamic,
}

struct ApihubCuration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApihubCurationData_>,
}

#[derive(Clone)]
pub struct ApihubCuration(std::rc::Rc<ApihubCuration_>);

impl ApihubCuration {
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

    #[doc = "Set the field `description`.\nThe description of the curation."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(self, v: impl std::convert::Into<terrars::BlockAssignable<ApihubCuration_Endpoint>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApihubCuration_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the curation was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `curation_id` after provisioning.\nThe ID to use for the curation resource, which will become the final\ncomponent of the curations's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified ID is already used by another curation resource in the API\nhub.\n* If not provided, a system generated ID will be used.\n\nThis value should be 4-500 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub fn curation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.curation_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the curation."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the curation."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_execution_error_code` after provisioning.\nThe error code of the last execution of the curation. The error code is\npopulated only when the last execution state is failed.\nPossible values:\nERROR_CODE_UNSPECIFIED\nINTERNAL_ERROR\nUNAUTHORIZED"]
    pub fn last_execution_error_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_execution_error_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_execution_error_message` after provisioning.\nError message describing the failure, if any, during the last execution of\nthe curation."]
    pub fn last_execution_error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_execution_error_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_execution_state` after provisioning.\nThe last execution state of the curation.\nPossible values:\nLAST_EXECUTION_STATE_UNSPECIFIED\nSUCCEEDED\nFAILED"]
    pub fn last_execution_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_execution_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the curation.\n\nFormat:\n'projects/{project}/locations/{location}/curations/{curation}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `plugin_instance_actions` after provisioning.\nThe plugin instances and associated actions that are using the curation.\nNote: A particular curation could be used by multiple plugin instances or\nmultiple actions in a plugin instance."]
    pub fn plugin_instance_actions(&self) -> terrars::ListRef<ApihubCuration_PluginInstanceActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.plugin_instance_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the curation was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<ApihubCuration_EndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubCuration_TimeoutsRef {
        ApihubCuration_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApihubCuration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApihubCuration { }

impl terrars::ToListMappable for ApihubCuration {
    type O = terrars::ListRef<ApihubCurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApihubCuration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apihub_curation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApihubCuration {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the curation resource, which will become the final\ncomponent of the curations's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified ID is already used by another curation resource in the API\nhub.\n* If not provided, a system generated ID will be used.\n\nThis value should be 4-500 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub curation_id: terrars::PrimField<std::string::String>,
    #[doc = "The display name of the curation."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildApihubCuration {
    pub fn build(self, stack: &mut terrars::Stack) -> ApihubCuration {
        let out = ApihubCuration(std::rc::Rc::new(ApihubCuration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApihubCurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                curation_id: self.curation_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                endpoint: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApihubCurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubCurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApihubCurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time at which the curation was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `curation_id` after provisioning.\nThe ID to use for the curation resource, which will become the final\ncomponent of the curations's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified ID is already used by another curation resource in the API\nhub.\n* If not provided, a system generated ID will be used.\n\nThis value should be 4-500 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub fn curation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.curation_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the curation."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the curation."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_execution_error_code` after provisioning.\nThe error code of the last execution of the curation. The error code is\npopulated only when the last execution state is failed.\nPossible values:\nERROR_CODE_UNSPECIFIED\nINTERNAL_ERROR\nUNAUTHORIZED"]
    pub fn last_execution_error_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_execution_error_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_execution_error_message` after provisioning.\nError message describing the failure, if any, during the last execution of\nthe curation."]
    pub fn last_execution_error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_execution_error_message", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_execution_state` after provisioning.\nThe last execution state of the curation.\nPossible values:\nLAST_EXECUTION_STATE_UNSPECIFIED\nSUCCEEDED\nFAILED"]
    pub fn last_execution_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_execution_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the curation.\n\nFormat:\n'projects/{project}/locations/{location}/curations/{curation}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `plugin_instance_actions` after provisioning.\nThe plugin instances and associated actions that are using the curation.\nNote: A particular curation could be used by multiple plugin instances or\nmultiple actions in a plugin instance."]
    pub fn plugin_instance_actions(&self) -> terrars::ListRef<ApihubCuration_PluginInstanceActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.plugin_instance_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe time at which the curation was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::ListRef<ApihubCuration_EndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubCuration_TimeoutsRef {
        ApihubCuration_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ApihubCurationDynamic {
    endpoint: std::option::Option<terrars::DynamicBlock<ApihubCuration_Endpoint>>,
}

#[derive(Serialize)]
pub struct ApihubCuration_PluginInstanceActions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plugin_instance: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApihubCuration_PluginInstanceActions {
    #[doc = "Set the field `action_id`.\n"]
    pub fn set_action_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `plugin_instance`.\n"]
    pub fn set_plugin_instance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.plugin_instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApihubCuration_PluginInstanceActions {
    type O = terrars::BlockAssignable<ApihubCuration_PluginInstanceActions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubCuration_PluginInstanceActions {}

impl BuildApihubCuration_PluginInstanceActions {
    pub fn build(self) -> ApihubCuration_PluginInstanceActions {
        ApihubCuration_PluginInstanceActions {
            action_id: core::default::Default::default(),
            plugin_instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubCuration_PluginInstanceActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubCuration_PluginInstanceActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApihubCuration_PluginInstanceActionsRef {
        ApihubCuration_PluginInstanceActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubCuration_PluginInstanceActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_id` after provisioning.\n"]
    pub fn action_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_id", self.base))
    }

    #[doc = "Get a reference to the value of field `plugin_instance` after provisioning.\n"]
    pub fn plugin_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin_instance", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetails {
    trigger_id: terrars::PrimField<std::string::String>,
    uri: terrars::PrimField<std::string::String>,
}

impl ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetails { }

impl terrars::ToListMappable for ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetails {
    type O = terrars::BlockAssignable<ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubCuration_Endpoint_ApplicationIntegrationEndpointDetails {
    #[doc = "The API trigger ID of the Application Integration workflow."]
    pub trigger_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The endpoint URI should be a valid REST URI for triggering an Application\nIntegration.\nFormat:\n'https://integrations.googleapis.com/v1/{name=projects/*/locations/*/integrations/*}:execute'\nor\n'https://{location}-integrations.googleapis.com/v1/{name=projects/*/locations/*/integrations/*}:execute'"]
    pub uri: terrars::PrimField<std::string::String>,
}

impl BuildApihubCuration_Endpoint_ApplicationIntegrationEndpointDetails {
    pub fn build(self) -> ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetails {
        ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetails {
            trigger_id: self.trigger_id,
            uri: self.uri,
        }
    }
}

#[allow(dead_code)]
pub struct ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetailsRef {
        ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `trigger_id` after provisioning.\nThe API trigger ID of the Application Integration workflow."]
    pub fn trigger_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uri` after provisioning.\nThe endpoint URI should be a valid REST URI for triggering an Application\nIntegration.\nFormat:\n'https://integrations.googleapis.com/v1/{name=projects/*/locations/*/integrations/*}:execute'\nor\n'https://{location}-integrations.googleapis.com/v1/{name=projects/*/locations/*/integrations/*}:execute'"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApihubCuration_EndpointDynamic {
    application_integration_endpoint_details: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetails>>,
}

#[derive(Serialize)]
pub struct ApihubCuration_Endpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_integration_endpoint_details: std
    ::option
    ::Option<std::vec::Vec<ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetails>>,
    dynamic: ApihubCuration_EndpointDynamic,
}

impl ApihubCuration_Endpoint {
    #[doc = "Set the field `application_integration_endpoint_details`.\n"]
    pub fn set_application_integration_endpoint_details(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetails>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.application_integration_endpoint_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.application_integration_endpoint_details = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApihubCuration_Endpoint {
    type O = terrars::BlockAssignable<ApihubCuration_Endpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubCuration_Endpoint {}

impl BuildApihubCuration_Endpoint {
    pub fn build(self) -> ApihubCuration_Endpoint {
        ApihubCuration_Endpoint {
            application_integration_endpoint_details: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubCuration_EndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubCuration_EndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApihubCuration_EndpointRef {
        ApihubCuration_EndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubCuration_EndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_integration_endpoint_details` after provisioning.\n"]
    pub fn application_integration_endpoint_details(
        &self,
    ) -> terrars::ListRef<ApihubCuration_Endpoint_ApplicationIntegrationEndpointDetailsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.application_integration_endpoint_details", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct ApihubCuration_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApihubCuration_Timeouts {
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

impl terrars::ToListMappable for ApihubCuration_Timeouts {
    type O = terrars::BlockAssignable<ApihubCuration_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubCuration_Timeouts {}

impl BuildApihubCuration_Timeouts {
    pub fn build(self) -> ApihubCuration_Timeouts {
        ApihubCuration_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubCuration_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubCuration_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApihubCuration_TimeoutsRef {
        ApihubCuration_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubCuration_TimeoutsRef {
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
