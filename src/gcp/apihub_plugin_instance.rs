#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApihubPluginInstanceData_ {
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
    disable: std::option::Option<terrars::PrimField<bool>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    plugin: terrars::PrimField<std::string::String>,
    plugin_instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<std::vec::Vec<ApihubPluginInstance_Actions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_config: std::option::Option<std::vec::Vec<ApihubPluginInstance_AuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApihubPluginInstance_Timeouts>,
    dynamic: ApihubPluginInstanceDynamic,
}

struct ApihubPluginInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApihubPluginInstanceData_>,
}

#[derive(Clone)]
pub struct ApihubPluginInstance(std::rc::Rc<ApihubPluginInstance_>);

impl ApihubPluginInstance {
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

    #[doc = "Set the field `disable`.\nThe display name for this plugin instance. Max length is 255 characters."]
    pub fn set_disable(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable = std::option::Option::Some(v.into());
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
        "Set the field `source_project_id`.\nOptional. The source project id of the plugin instance. This will be the id of runtime project in case of gcp based plugins and org id in case of non gcp based plugins. This field will be a required field for Google provided on-ramp plugins."]
    pub fn set_source_project_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().source_project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPluginInstance_Actions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auth_config`.\n"]
    pub fn set_auth_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPluginInstance_AuthConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApihubPluginInstance_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp indicating when the plugin instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable` after provisioning.\nThe display name for this plugin instance. Max length is 255 characters."]
    pub fn disable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for this plugin instance. Max length is 255 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error_message` after provisioning.\nError message describing the failure, if any, during Create, Delete or\nApplyConfig operation corresponding to the plugin instance.This field will\nonly be populated if the plugin instance is in the ERROR or FAILED state."]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique name of the plugin instance resource.\nFormat:\n'projects/{project}/locations/{location}/plugins/{plugin}/instances/{instance}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `plugin` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn plugin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `plugin_instance_id` after provisioning.\nThe ID to use for the plugin instance, which will become the final\ncomponent of the plugin instance's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another plugin instance in the plugin\nresource.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub fn plugin_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_project_id` after provisioning.\nOptional. The source project id of the plugin instance. This will be the id of runtime project in case of gcp based plugins and org id in case of non gcp based plugins. This field will be a required field for Google provided on-ramp plugins."]
    pub fn source_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_project_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the plugin instance (e.g., enabled, disabled,\nprovisioning).\nPossible values:\nSTATE_UNSPECIFIED\nCREATING\nACTIVE\nAPPLYING_CONFIG\nERROR\nFAILED\nDELETING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp indicating when the plugin instance was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<ApihubPluginInstance_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<ApihubPluginInstance_AuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubPluginInstance_TimeoutsRef {
        ApihubPluginInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApihubPluginInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApihubPluginInstance { }

impl terrars::ToListMappable for ApihubPluginInstance {
    type O = terrars::ListRef<ApihubPluginInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApihubPluginInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apihub_plugin_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApihubPluginInstance {
    pub tf_id: std::string::String,
    #[doc = "The display name for this plugin instance. Max length is 255 characters."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub plugin: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the plugin instance, which will become the final\ncomponent of the plugin instance's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another plugin instance in the plugin\nresource.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub plugin_instance_id: terrars::PrimField<std::string::String>,
}

impl BuildApihubPluginInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> ApihubPluginInstance {
        let out = ApihubPluginInstance(std::rc::Rc::new(ApihubPluginInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApihubPluginInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                disable: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: self.location,
                plugin: self.plugin,
                plugin_instance_id: self.plugin_instance_id,
                project: core::default::Default::default(),
                source_project_id: core::default::Default::default(),
                actions: core::default::Default::default(),
                auth_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApihubPluginInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApihubPluginInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp indicating when the plugin instance was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable` after provisioning.\nThe display name for this plugin instance. Max length is 255 characters."]
    pub fn disable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for this plugin instance. Max length is 255 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `error_message` after provisioning.\nError message describing the failure, if any, during Create, Delete or\nApplyConfig operation corresponding to the plugin instance.This field will\nonly be populated if the plugin instance is in the ERROR or FAILED state."]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique name of the plugin instance resource.\nFormat:\n'projects/{project}/locations/{location}/plugins/{plugin}/instances/{instance}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `plugin` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn plugin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `plugin_instance_id` after provisioning.\nThe ID to use for the plugin instance, which will become the final\ncomponent of the plugin instance's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another plugin instance in the plugin\nresource.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, and valid characters\nare /a-z[0-9]-_/."]
    pub fn plugin_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `source_project_id` after provisioning.\nOptional. The source project id of the plugin instance. This will be the id of runtime project in case of gcp based plugins and org id in case of non gcp based plugins. This field will be a required field for Google provided on-ramp plugins."]
    pub fn source_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_project_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the plugin instance (e.g., enabled, disabled,\nprovisioning).\nPossible values:\nSTATE_UNSPECIFIED\nCREATING\nACTIVE\nAPPLYING_CONFIG\nERROR\nFAILED\nDELETING"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp indicating when the plugin instance was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<ApihubPluginInstance_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_config` after provisioning.\n"]
    pub fn auth_config(&self) -> terrars::ListRef<ApihubPluginInstance_AuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubPluginInstance_TimeoutsRef {
        ApihubPluginInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ApihubPluginInstanceDynamic {
    actions: std::option::Option<terrars::DynamicBlock<ApihubPluginInstance_Actions>>,
    auth_config: std::option::Option<terrars::DynamicBlock<ApihubPluginInstance_AuthConfig>>,
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_Actions_HubInstanceAction_LastExecution {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    result: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApihubPluginInstance_Actions_HubInstanceAction_LastExecution {
    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_message`.\n"]
    pub fn set_error_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `result`.\n"]
    pub fn set_result(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.result = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApihubPluginInstance_Actions_HubInstanceAction_LastExecution {
    type O = terrars::BlockAssignable<ApihubPluginInstance_Actions_HubInstanceAction_LastExecution>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_Actions_HubInstanceAction_LastExecution {}

impl BuildApihubPluginInstance_Actions_HubInstanceAction_LastExecution {
    pub fn build(self) -> ApihubPluginInstance_Actions_HubInstanceAction_LastExecution {
        ApihubPluginInstance_Actions_HubInstanceAction_LastExecution {
            end_time: core::default::Default::default(),
            error_message: core::default::Default::default(),
            result: core::default::Default::default(),
            start_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_Actions_HubInstanceAction_LastExecutionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_Actions_HubInstanceAction_LastExecutionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPluginInstance_Actions_HubInstanceAction_LastExecutionRef {
        ApihubPluginInstance_Actions_HubInstanceAction_LastExecutionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_Actions_HubInstanceAction_LastExecutionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.base))
    }

    #[doc = "Get a reference to the value of field `error_message` after provisioning.\n"]
    pub fn error_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_message", self.base))
    }

    #[doc = "Get a reference to the value of field `result` after provisioning.\n"]
    pub fn result(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.result", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_Actions_HubInstanceAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_execution_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_execution: std
    ::option
    ::Option<terrars::ListField<ApihubPluginInstance_Actions_HubInstanceAction_LastExecution>>,
}

impl ApihubPluginInstance_Actions_HubInstanceAction {
    #[doc = "Set the field `current_execution_state`.\n"]
    pub fn set_current_execution_state(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.current_execution_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_execution`.\n"]
    pub fn set_last_execution(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ApihubPluginInstance_Actions_HubInstanceAction_LastExecution>>,
    ) -> Self {
        self.last_execution = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApihubPluginInstance_Actions_HubInstanceAction {
    type O = terrars::BlockAssignable<ApihubPluginInstance_Actions_HubInstanceAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_Actions_HubInstanceAction {}

impl BuildApihubPluginInstance_Actions_HubInstanceAction {
    pub fn build(self) -> ApihubPluginInstance_Actions_HubInstanceAction {
        ApihubPluginInstance_Actions_HubInstanceAction {
            current_execution_state: core::default::Default::default(),
            last_execution: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_Actions_HubInstanceActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_Actions_HubInstanceActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPluginInstance_Actions_HubInstanceActionRef {
        ApihubPluginInstance_Actions_HubInstanceActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_Actions_HubInstanceActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_execution_state` after provisioning.\n"]
    pub fn current_execution_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_execution_state", self.base))
    }

    #[doc = "Get a reference to the value of field `last_execution` after provisioning.\n"]
    pub fn last_execution(&self) -> terrars::ListRef<ApihubPluginInstance_Actions_HubInstanceAction_LastExecutionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_execution", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_Actions_CurationConfig_CustomCuration {
    curation: terrars::PrimField<std::string::String>,
}

impl ApihubPluginInstance_Actions_CurationConfig_CustomCuration { }

impl terrars::ToListMappable for ApihubPluginInstance_Actions_CurationConfig_CustomCuration {
    type O = terrars::BlockAssignable<ApihubPluginInstance_Actions_CurationConfig_CustomCuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_Actions_CurationConfig_CustomCuration {
    #[doc =
        "The unique name of the curation resource. This will be the name of the\ncuration resource in the format:\n'projects/{project}/locations/{location}/curations/{curation}'"]
    pub curation: terrars::PrimField<std::string::String>,
}

impl BuildApihubPluginInstance_Actions_CurationConfig_CustomCuration {
    pub fn build(self) -> ApihubPluginInstance_Actions_CurationConfig_CustomCuration {
        ApihubPluginInstance_Actions_CurationConfig_CustomCuration { curation: self.curation }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_Actions_CurationConfig_CustomCurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_Actions_CurationConfig_CustomCurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPluginInstance_Actions_CurationConfig_CustomCurationRef {
        ApihubPluginInstance_Actions_CurationConfig_CustomCurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_Actions_CurationConfig_CustomCurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `curation` after provisioning.\nThe unique name of the curation resource. This will be the name of the\ncuration resource in the format:\n'projects/{project}/locations/{location}/curations/{curation}'"]
    pub fn curation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.curation", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApihubPluginInstance_Actions_CurationConfigDynamic {
    custom_curation: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubPluginInstance_Actions_CurationConfig_CustomCuration>>,
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_Actions_CurationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    curation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_curation: std::option::Option<std::vec::Vec<ApihubPluginInstance_Actions_CurationConfig_CustomCuration>>,
    dynamic: ApihubPluginInstance_Actions_CurationConfigDynamic,
}

impl ApihubPluginInstance_Actions_CurationConfig {
    #[doc =
        "Set the field `curation_type`.\nPossible values:\nCURATION_TYPE_UNSPECIFIED\nDEFAULT_CURATION_FOR_API_METADATA\nCUSTOM_CURATION_FOR_API_METADATA"]
    pub fn set_curation_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.curation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_curation`.\n"]
    pub fn set_custom_curation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ApihubPluginInstance_Actions_CurationConfig_CustomCuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_curation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_curation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApihubPluginInstance_Actions_CurationConfig {
    type O = terrars::BlockAssignable<ApihubPluginInstance_Actions_CurationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_Actions_CurationConfig {}

impl BuildApihubPluginInstance_Actions_CurationConfig {
    pub fn build(self) -> ApihubPluginInstance_Actions_CurationConfig {
        ApihubPluginInstance_Actions_CurationConfig {
            curation_type: core::default::Default::default(),
            custom_curation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_Actions_CurationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_Actions_CurationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPluginInstance_Actions_CurationConfigRef {
        ApihubPluginInstance_Actions_CurationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_Actions_CurationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `curation_type` after provisioning.\nPossible values:\nCURATION_TYPE_UNSPECIFIED\nDEFAULT_CURATION_FOR_API_METADATA\nCUSTOM_CURATION_FOR_API_METADATA"]
    pub fn curation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.curation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_curation` after provisioning.\n"]
    pub fn custom_curation(&self) -> terrars::ListRef<ApihubPluginInstance_Actions_CurationConfig_CustomCurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_curation", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApihubPluginInstance_ActionsDynamic {
    curation_config: std::option::Option<terrars::DynamicBlock<ApihubPluginInstance_Actions_CurationConfig>>,
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_Actions {
    action_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_cron_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    curation_config: std::option::Option<std::vec::Vec<ApihubPluginInstance_Actions_CurationConfig>>,
    dynamic: ApihubPluginInstance_ActionsDynamic,
}

impl ApihubPluginInstance_Actions {
    #[doc =
        "Set the field `schedule_cron_expression`.\nThe schedule for this plugin instance action. This can only be set if the\nplugin supports API_HUB_SCHEDULE_TRIGGER mode for this action."]
    pub fn set_schedule_cron_expression(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.schedule_cron_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `schedule_time_zone`.\nThe time zone for the schedule cron expression. If not provided, UTC will\nbe used."]
    pub fn set_schedule_time_zone(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.schedule_time_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `curation_config`.\n"]
    pub fn set_curation_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPluginInstance_Actions_CurationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.curation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.curation_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApihubPluginInstance_Actions {
    type O = terrars::BlockAssignable<ApihubPluginInstance_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_Actions {
    #[doc = "This should map to one of the action id specified\nin actions_config in the plugin."]
    pub action_id: terrars::PrimField<std::string::String>,
}

impl BuildApihubPluginInstance_Actions {
    pub fn build(self) -> ApihubPluginInstance_Actions {
        ApihubPluginInstance_Actions {
            action_id: self.action_id,
            schedule_cron_expression: core::default::Default::default(),
            schedule_time_zone: core::default::Default::default(),
            curation_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_ActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApihubPluginInstance_ActionsRef {
        ApihubPluginInstance_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `action_id` after provisioning.\nThis should map to one of the action id specified\nin actions_config in the plugin."]
    pub fn action_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `hub_instance_action` after provisioning.\nThe execution status for the plugin instance."]
    pub fn hub_instance_action(&self) -> terrars::ListRef<ApihubPluginInstance_Actions_HubInstanceActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hub_instance_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schedule_cron_expression` after provisioning.\nThe schedule for this plugin instance action. This can only be set if the\nplugin supports API_HUB_SCHEDULE_TRIGGER mode for this action."]
    pub fn schedule_cron_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_cron_expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schedule_time_zone` after provisioning.\nThe time zone for the schedule cron expression. If not provided, UTC will\nbe used."]
    pub fn schedule_time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_time_zone", self.base))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the plugin action in the plugin instance.\nPossible values:\nSTATE_UNSPECIFIED\nENABLED\nDISABLED\nENABLING\nDISABLING\nERROR"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `curation_config` after provisioning.\n"]
    pub fn curation_config(&self) -> terrars::ListRef<ApihubPluginInstance_Actions_CurationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.curation_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKey {
    secret_version: terrars::PrimField<std::string::String>,
}

impl ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKey { }

impl terrars::ToListMappable for ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKey {
    type O = terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKey {
    #[doc = "The resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKey {
    pub fn build(self) -> ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKey {
        ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKey { secret_version: self.secret_version }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKeyRef {
        ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nThe resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApihubPluginInstance_AuthConfig_ApiKeyConfigDynamic {
    api_key: std::option::Option<terrars::DynamicBlock<ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKey>>,
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_AuthConfig_ApiKeyConfig {
    http_element_location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<std::vec::Vec<ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKey>>,
    dynamic: ApihubPluginInstance_AuthConfig_ApiKeyConfigDynamic,
}

impl ApihubPluginInstance_AuthConfig_ApiKeyConfig {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApihubPluginInstance_AuthConfig_ApiKeyConfig {
    type O = terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_ApiKeyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_AuthConfig_ApiKeyConfig {
    #[doc =
        "The location of the API key.\nThe default value is QUERY.\nPossible values:\nHTTP_ELEMENT_LOCATION_UNSPECIFIED\nQUERY\nHEADER\nPATH\nBODY\nCOOKIE"]
    pub http_element_location: terrars::PrimField<std::string::String>,
    #[doc =
        "The parameter name of the API key.\nE.g. If the API request is \"https://example.com/act?api_key=\",\n\"api_key\" would be the parameter name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildApihubPluginInstance_AuthConfig_ApiKeyConfig {
    pub fn build(self) -> ApihubPluginInstance_AuthConfig_ApiKeyConfig {
        ApihubPluginInstance_AuthConfig_ApiKeyConfig {
            http_element_location: self.http_element_location,
            name: self.name,
            api_key: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_AuthConfig_ApiKeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_AuthConfig_ApiKeyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPluginInstance_AuthConfig_ApiKeyConfigRef {
        ApihubPluginInstance_AuthConfig_ApiKeyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_AuthConfig_ApiKeyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `http_element_location` after provisioning.\nThe location of the API key.\nThe default value is QUERY.\nPossible values:\nHTTP_ELEMENT_LOCATION_UNSPECIFIED\nQUERY\nHEADER\nPATH\nBODY\nCOOKIE"]
    pub fn http_element_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_element_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe parameter name of the API key.\nE.g. If the API request is \"https://example.com/act?api_key=\",\n\"api_key\" would be the parameter name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::ListRef<ApihubPluginInstance_AuthConfig_ApiKeyConfig_ApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfig {
    service_account: terrars::PrimField<std::string::String>,
}

impl ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfig { }

impl terrars::ToListMappable for ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfig {
    type O = terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_AuthConfig_GoogleServiceAccountConfig {
    #[doc =
        "The service account to be used for authenticating request.\n\nThe 'iam.serviceAccounts.getAccessToken' permission should be granted on\nthis service account to the impersonator service account."]
    pub service_account: terrars::PrimField<std::string::String>,
}

impl BuildApihubPluginInstance_AuthConfig_GoogleServiceAccountConfig {
    pub fn build(self) -> ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfig {
        ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfig { service_account: self.service_account }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfigRef {
        ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe service account to be used for authenticating request.\n\nThe 'iam.serviceAccounts.getAccessToken' permission should be granted on\nthis service account to the impersonator service account."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecret {
    secret_version: terrars::PrimField<std::string::String>,
}

impl ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecret { }

impl terrars::ToListMappable for ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecret {
    type O = terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecret {
    #[doc = "The resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecret {
    pub fn build(self) -> ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecret {
        ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecret {
            secret_version: self.secret_version,
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecretRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecretRef {
        ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nThe resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfigDynamic {
    client_secret: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecret>>,
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig {
    client_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret: std
    ::option
    ::Option<std::vec::Vec<ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecret>>,
    dynamic: ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfigDynamic,
}

impl ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig {
    #[doc = "Set the field `client_secret`.\n"]
    pub fn set_client_secret(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecret>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_secret = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig {
    type O = terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig {
    #[doc = "The client identifier."]
    pub client_id: terrars::PrimField<std::string::String>,
}

impl BuildApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig {
    pub fn build(self) -> ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig {
        ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig {
            client_id: self.client_id,
            client_secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfigRef {
        ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\nThe client identifier."]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(
        &self,
    ) -> terrars::ListRef<ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig_ClientSecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_AuthConfig_UserPasswordConfig_Password {
    secret_version: terrars::PrimField<std::string::String>,
}

impl ApihubPluginInstance_AuthConfig_UserPasswordConfig_Password { }

impl terrars::ToListMappable for ApihubPluginInstance_AuthConfig_UserPasswordConfig_Password {
    type O = terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_UserPasswordConfig_Password>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_AuthConfig_UserPasswordConfig_Password {
    #[doc = "The resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub secret_version: terrars::PrimField<std::string::String>,
}

impl BuildApihubPluginInstance_AuthConfig_UserPasswordConfig_Password {
    pub fn build(self) -> ApihubPluginInstance_AuthConfig_UserPasswordConfig_Password {
        ApihubPluginInstance_AuthConfig_UserPasswordConfig_Password { secret_version: self.secret_version }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_AuthConfig_UserPasswordConfig_PasswordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_AuthConfig_UserPasswordConfig_PasswordRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPluginInstance_AuthConfig_UserPasswordConfig_PasswordRef {
        ApihubPluginInstance_AuthConfig_UserPasswordConfig_PasswordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_AuthConfig_UserPasswordConfig_PasswordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `secret_version` after provisioning.\nThe resource name of the secret version in the format,\nformat as: 'projects/*/secrets/*/versions/*'."]
    pub fn secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApihubPluginInstance_AuthConfig_UserPasswordConfigDynamic {
    password: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubPluginInstance_AuthConfig_UserPasswordConfig_Password>>,
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_AuthConfig_UserPasswordConfig {
    username: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<std::vec::Vec<ApihubPluginInstance_AuthConfig_UserPasswordConfig_Password>>,
    dynamic: ApihubPluginInstance_AuthConfig_UserPasswordConfigDynamic,
}

impl ApihubPluginInstance_AuthConfig_UserPasswordConfig {
    #[doc = "Set the field `password`.\n"]
    pub fn set_password(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_UserPasswordConfig_Password>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.password = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.password = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApihubPluginInstance_AuthConfig_UserPasswordConfig {
    type O = terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_UserPasswordConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_AuthConfig_UserPasswordConfig {
    #[doc = "Username."]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildApihubPluginInstance_AuthConfig_UserPasswordConfig {
    pub fn build(self) -> ApihubPluginInstance_AuthConfig_UserPasswordConfig {
        ApihubPluginInstance_AuthConfig_UserPasswordConfig {
            username: self.username,
            password: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_AuthConfig_UserPasswordConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_AuthConfig_UserPasswordConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPluginInstance_AuthConfig_UserPasswordConfigRef {
        ApihubPluginInstance_AuthConfig_UserPasswordConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_AuthConfig_UserPasswordConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\nUsername."]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::ListRef<ApihubPluginInstance_AuthConfig_UserPasswordConfig_PasswordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApihubPluginInstance_AuthConfigDynamic {
    api_key_config: std::option::Option<terrars::DynamicBlock<ApihubPluginInstance_AuthConfig_ApiKeyConfig>>,
    google_service_account_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfig>>,
    oauth2_client_credentials_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig>>,
    user_password_config: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubPluginInstance_AuthConfig_UserPasswordConfig>>,
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_AuthConfig {
    auth_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_config: std::option::Option<std::vec::Vec<ApihubPluginInstance_AuthConfig_ApiKeyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_service_account_config: std
    ::option
    ::Option<std::vec::Vec<ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_credentials_config: std
    ::option
    ::Option<std::vec::Vec<ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_password_config: std::option::Option<std::vec::Vec<ApihubPluginInstance_AuthConfig_UserPasswordConfig>>,
    dynamic: ApihubPluginInstance_AuthConfigDynamic,
}

impl ApihubPluginInstance_AuthConfig {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_ApiKeyConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_key_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_service_account_config`.\n"]
    pub fn set_google_service_account_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_service_account_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_service_account_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_client_credentials_config`.\n"]
    pub fn set_oauth2_client_credentials_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_client_credentials_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_client_credentials_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_password_config`.\n"]
    pub fn set_user_password_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPluginInstance_AuthConfig_UserPasswordConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_password_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_password_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApihubPluginInstance_AuthConfig {
    type O = terrars::BlockAssignable<ApihubPluginInstance_AuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_AuthConfig {
    #[doc =
        "Possible values:\nAUTH_TYPE_UNSPECIFIED\nNO_AUTH\nGOOGLE_SERVICE_ACCOUNT\nUSER_PASSWORD\nAPI_KEY\nOAUTH2_CLIENT_CREDENTIALS"]
    pub auth_type: terrars::PrimField<std::string::String>,
}

impl BuildApihubPluginInstance_AuthConfig {
    pub fn build(self) -> ApihubPluginInstance_AuthConfig {
        ApihubPluginInstance_AuthConfig {
            auth_type: self.auth_type,
            api_key_config: core::default::Default::default(),
            google_service_account_config: core::default::Default::default(),
            oauth2_client_credentials_config: core::default::Default::default(),
            user_password_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_AuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_AuthConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApihubPluginInstance_AuthConfigRef {
        ApihubPluginInstance_AuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_AuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auth_type` after provisioning.\nPossible values:\nAUTH_TYPE_UNSPECIFIED\nNO_AUTH\nGOOGLE_SERVICE_ACCOUNT\nUSER_PASSWORD\nAPI_KEY\nOAUTH2_CLIENT_CREDENTIALS"]
    pub fn auth_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_type", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(&self) -> terrars::ListRef<ApihubPluginInstance_AuthConfig_ApiKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_config", self.base))
    }

    #[doc = "Get a reference to the value of field `google_service_account_config` after provisioning.\n"]
    pub fn google_service_account_config(
        &self,
    ) -> terrars::ListRef<ApihubPluginInstance_AuthConfig_GoogleServiceAccountConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_service_account_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_credentials_config` after provisioning.\n"]
    pub fn oauth2_client_credentials_config(
        &self,
    ) -> terrars::ListRef<ApihubPluginInstance_AuthConfig_Oauth2ClientCredentialsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_client_credentials_config", self.base))
    }

    #[doc = "Get a reference to the value of field `user_password_config` after provisioning.\n"]
    pub fn user_password_config(&self) -> terrars::ListRef<ApihubPluginInstance_AuthConfig_UserPasswordConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_password_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPluginInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApihubPluginInstance_Timeouts {
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

impl terrars::ToListMappable for ApihubPluginInstance_Timeouts {
    type O = terrars::BlockAssignable<ApihubPluginInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPluginInstance_Timeouts {}

impl BuildApihubPluginInstance_Timeouts {
    pub fn build(self) -> ApihubPluginInstance_Timeouts {
        ApihubPluginInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPluginInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApihubPluginInstance_TimeoutsRef {
        ApihubPluginInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPluginInstance_TimeoutsRef {
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
