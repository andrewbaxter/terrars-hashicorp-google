#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApihubPluginData_ {
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
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plugin_category: std::option::Option<terrars::PrimField<std::string::String>>,
    plugin_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions_config: std::option::Option<std::vec::Vec<ApihubPlugin_ActionsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_template: std::option::Option<std::vec::Vec<ApihubPlugin_ConfigTemplate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    documentation: std::option::Option<std::vec::Vec<ApihubPlugin_Documentation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosting_service: std::option::Option<std::vec::Vec<ApihubPlugin_HostingService>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApihubPlugin_Timeouts>,
    dynamic: ApihubPluginDynamic,
}

struct ApihubPlugin_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApihubPluginData_>,
}

#[derive(Clone)]
pub struct ApihubPlugin(std::rc::Rc<ApihubPlugin_>);

impl ApihubPlugin {
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

    #[doc =
        "Set the field `description`.\nThe plugin description. Max length is 2000 characters (Unicode code\npoints)."]
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
        "Set the field `plugin_category`.\nPossible values:\nPLUGIN_CATEGORY_UNSPECIFIED\nAPI_GATEWAY\nAPI_PRODUCER"]
    pub fn set_plugin_category(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().plugin_category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `actions_config`.\n"]
    pub fn set_actions_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPlugin_ActionsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `config_template`.\n"]
    pub fn set_config_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPlugin_ConfigTemplate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().config_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.config_template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `documentation`.\n"]
    pub fn set_documentation(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPlugin_Documentation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().documentation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.documentation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hosting_service`.\n"]
    pub fn set_hosting_service(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPlugin_HostingService>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hosting_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hosting_service = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApihubPlugin_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp indicating when the plugin was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe plugin description. Max length is 2000 characters (Unicode code\npoints)."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the plugin. Max length is 50 characters (Unicode code\npoints)."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the plugin.\nFormat: 'projects/{project}/locations/{location}/plugins/{plugin}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_type` after provisioning.\nThe type of the plugin, indicating whether it is 'SYSTEM_OWNED' or\n'USER_OWNED'.\nPossible values:\nOWNERSHIP_TYPE_UNSPECIFIED\nSYSTEM_OWNED\nUSER_OWNED"]
    pub fn ownership_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ownership_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `plugin_category` after provisioning.\nPossible values:\nPLUGIN_CATEGORY_UNSPECIFIED\nAPI_GATEWAY\nAPI_PRODUCER"]
    pub fn plugin_category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin_category", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `plugin_id` after provisioning.\nThe ID to use for the Plugin resource, which will become the final\ncomponent of the Plugin's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another Plugin resource in the API hub\ninstance.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, overall resource name which will be\nof format\n'projects/{project}/locations/{location}/plugins/{plugin}',\nits length is limited to 1000 characters and valid characters are\n/a-z[0-9]-_/."]
    pub fn plugin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nRepresents the state of the plugin.\nNote this field will not be set for plugins developed via plugin\nframework as the state will be managed at plugin instance level.\nPossible values:\nSTATE_UNSPECIFIED\nENABLED\nDISABLED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp indicating when the plugin was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `actions_config` after provisioning.\n"]
    pub fn actions_config(&self) -> terrars::ListRef<ApihubPlugin_ActionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_template` after provisioning.\n"]
    pub fn config_template(&self) -> terrars::ListRef<ApihubPlugin_ConfigTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `documentation` after provisioning.\n"]
    pub fn documentation(&self) -> terrars::ListRef<ApihubPlugin_DocumentationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosting_service` after provisioning.\n"]
    pub fn hosting_service(&self) -> terrars::ListRef<ApihubPlugin_HostingServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosting_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubPlugin_TimeoutsRef {
        ApihubPlugin_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApihubPlugin {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApihubPlugin { }

impl terrars::ToListMappable for ApihubPlugin {
    type O = terrars::ListRef<ApihubPluginRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApihubPlugin_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apihub_plugin".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApihubPlugin {
    pub tf_id: std::string::String,
    #[doc = "The display name of the plugin. Max length is 50 characters (Unicode code\npoints)."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the Plugin resource, which will become the final\ncomponent of the Plugin's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another Plugin resource in the API hub\ninstance.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, overall resource name which will be\nof format\n'projects/{project}/locations/{location}/plugins/{plugin}',\nits length is limited to 1000 characters and valid characters are\n/a-z[0-9]-_/."]
    pub plugin_id: terrars::PrimField<std::string::String>,
}

impl BuildApihubPlugin {
    pub fn build(self, stack: &mut terrars::Stack) -> ApihubPlugin {
        let out = ApihubPlugin(std::rc::Rc::new(ApihubPlugin_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApihubPluginData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                location: self.location,
                plugin_category: core::default::Default::default(),
                plugin_id: self.plugin_id,
                project: core::default::Default::default(),
                actions_config: core::default::Default::default(),
                config_template: core::default::Default::default(),
                documentation: core::default::Default::default(),
                hosting_service: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApihubPluginRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPluginRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApihubPluginRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp indicating when the plugin was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe plugin description. Max length is 2000 characters (Unicode code\npoints)."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the plugin. Max length is 50 characters (Unicode code\npoints)."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The name of the plugin.\nFormat: 'projects/{project}/locations/{location}/plugins/{plugin}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ownership_type` after provisioning.\nThe type of the plugin, indicating whether it is 'SYSTEM_OWNED' or\n'USER_OWNED'.\nPossible values:\nOWNERSHIP_TYPE_UNSPECIFIED\nSYSTEM_OWNED\nUSER_OWNED"]
    pub fn ownership_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ownership_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `plugin_category` after provisioning.\nPossible values:\nPLUGIN_CATEGORY_UNSPECIFIED\nAPI_GATEWAY\nAPI_PRODUCER"]
    pub fn plugin_category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin_category", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `plugin_id` after provisioning.\nThe ID to use for the Plugin resource, which will become the final\ncomponent of the Plugin's resource name. This field is optional.\n\n* If provided, the same will be used. The service will throw an error if\nthe specified id is already used by another Plugin resource in the API hub\ninstance.\n* If not provided, a system generated id will be used.\n\nThis value should be 4-63 characters, overall resource name which will be\nof format\n'projects/{project}/locations/{location}/plugins/{plugin}',\nits length is limited to 1000 characters and valid characters are\n/a-z[0-9]-_/."]
    pub fn plugin_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plugin_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nRepresents the state of the plugin.\nNote this field will not be set for plugins developed via plugin\nframework as the state will be managed at plugin instance level.\nPossible values:\nSTATE_UNSPECIFIED\nENABLED\nDISABLED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp indicating when the plugin was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `actions_config` after provisioning.\n"]
    pub fn actions_config(&self) -> terrars::ListRef<ApihubPlugin_ActionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_template` after provisioning.\n"]
    pub fn config_template(&self) -> terrars::ListRef<ApihubPlugin_ConfigTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `documentation` after provisioning.\n"]
    pub fn documentation(&self) -> terrars::ListRef<ApihubPlugin_DocumentationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.documentation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosting_service` after provisioning.\n"]
    pub fn hosting_service(&self) -> terrars::ListRef<ApihubPlugin_HostingServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosting_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApihubPlugin_TimeoutsRef {
        ApihubPlugin_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ApihubPluginDynamic {
    actions_config: std::option::Option<terrars::DynamicBlock<ApihubPlugin_ActionsConfig>>,
    config_template: std::option::Option<terrars::DynamicBlock<ApihubPlugin_ConfigTemplate>>,
    documentation: std::option::Option<terrars::DynamicBlock<ApihubPlugin_Documentation>>,
    hosting_service: std::option::Option<terrars::DynamicBlock<ApihubPlugin_HostingService>>,
}

#[derive(Serialize)]
pub struct ApihubPlugin_ActionsConfig {
    description: terrars::PrimField<std::string::String>,
    display_name: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    trigger_mode: terrars::PrimField<std::string::String>,
}

impl ApihubPlugin_ActionsConfig { }

impl terrars::ToListMappable for ApihubPlugin_ActionsConfig {
    type O = terrars::BlockAssignable<ApihubPlugin_ActionsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPlugin_ActionsConfig {
    #[doc = "The description of the operation performed by the action."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = "The display name of the action."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The id of the action."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc =
        "The trigger mode supported by the action.\nPossible values:\nTRIGGER_MODE_UNSPECIFIED\nAPI_HUB_ON_DEMAND_TRIGGER\nAPI_HUB_SCHEDULE_TRIGGER\nNON_API_HUB_MANAGED"]
    pub trigger_mode: terrars::PrimField<std::string::String>,
}

impl BuildApihubPlugin_ActionsConfig {
    pub fn build(self) -> ApihubPlugin_ActionsConfig {
        ApihubPlugin_ActionsConfig {
            description: self.description,
            display_name: self.display_name,
            id: self.id,
            trigger_mode: self.trigger_mode,
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPlugin_ActionsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPlugin_ActionsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApihubPlugin_ActionsConfigRef {
        ApihubPlugin_ActionsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPlugin_ActionsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the operation performed by the action."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the action."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nThe id of the action."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trigger_mode` after provisioning.\nThe trigger mode supported by the action.\nPossible values:\nTRIGGER_MODE_UNSPECIFIED\nAPI_HUB_ON_DEMAND_TRIGGER\nAPI_HUB_SCHEDULE_TRIGGER\nNON_API_HUB_MANAGED"]
    pub fn trigger_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
}

impl ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptions {
    #[doc = "Set the field `description`.\nDescription of the option."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptions {
    type O = terrars::BlockAssignable<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptions {
    #[doc = "Display name of the option."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "Id of the option."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptions {
    pub fn build(self) -> ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptions {
        ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptions {
            description: core::default::Default::default(),
            display_name: self.display_name,
            id: self.id,
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptionsRef {
        ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the option."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the option."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nId of the option."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    id: terrars::PrimField<std::string::String>,
}

impl ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptions {
    #[doc = "Set the field `description`.\nDescription of the option."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptions {
    type O = terrars::BlockAssignable<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptions {
    #[doc = "Display name of the option."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "Id of the option."]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptions {
    pub fn build(self) -> ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptions {
        ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptions {
            description: core::default::Default::default(),
            display_name: self.display_name,
            id: self.id,
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptionsRef {
        ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the option."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the option."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nId of the option."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApihubPlugin_ConfigTemplate_AdditionalConfigTemplateDynamic {
    enum_options: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptions>>,
    multi_select_options: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptions>>,
}

#[derive(Serialize)]
pub struct ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    value_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enum_options: std::option::Option<std::vec::Vec<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multi_select_options: std
    ::option
    ::Option<std::vec::Vec<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptions>>,
    dynamic: ApihubPlugin_ConfigTemplate_AdditionalConfigTemplateDynamic,
}

impl ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate {
    #[doc = "Set the field `description`.\nDescription."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `required`.\nFlag represents that this 'ConfigVariable' must be provided for a\nPluginInstance."]
    pub fn set_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `validation_regex`.\nRegular expression in RE2 syntax used for validating the 'value' of a\n'ConfigVariable'."]
    pub fn set_validation_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.validation_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enum_options`.\n"]
    pub fn set_enum_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.enum_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.enum_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `multi_select_options`.\n"]
    pub fn set_multi_select_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.multi_select_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.multi_select_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate {
    type O = terrars::BlockAssignable<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPlugin_ConfigTemplate_AdditionalConfigTemplate {
    #[doc = "ID of the config variable. Must be unique within the configuration."]
    pub id: terrars::PrimField<std::string::String>,
    #[doc =
        "Type of the parameter: string, int, bool etc.\nPossible values:\nVALUE_TYPE_UNSPECIFIED\nSTRING\nINT\nBOOL\nSECRET\nENUM\nMULTI_SELECT\nMULTI_STRING\nMULTI_INT"]
    pub value_type: terrars::PrimField<std::string::String>,
}

impl BuildApihubPlugin_ConfigTemplate_AdditionalConfigTemplate {
    pub fn build(self) -> ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate {
        ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate {
            description: core::default::Default::default(),
            id: self.id,
            required: core::default::Default::default(),
            validation_regex: core::default::Default::default(),
            value_type: self.value_type,
            enum_options: core::default::Default::default(),
            multi_select_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPlugin_ConfigTemplate_AdditionalConfigTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPlugin_ConfigTemplate_AdditionalConfigTemplateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPlugin_ConfigTemplate_AdditionalConfigTemplateRef {
        ApihubPlugin_ConfigTemplate_AdditionalConfigTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPlugin_ConfigTemplate_AdditionalConfigTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nID of the config variable. Must be unique within the configuration."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `required` after provisioning.\nFlag represents that this 'ConfigVariable' must be provided for a\nPluginInstance."]
    pub fn required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `validation_regex` after provisioning.\nRegular expression in RE2 syntax used for validating the 'value' of a\n'ConfigVariable'."]
    pub fn validation_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value_type` after provisioning.\nType of the parameter: string, int, bool etc.\nPossible values:\nVALUE_TYPE_UNSPECIFIED\nSTRING\nINT\nBOOL\nSECRET\nENUM\nMULTI_SELECT\nMULTI_STRING\nMULTI_INT"]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_options` after provisioning.\n"]
    pub fn enum_options(&self) -> terrars::ListRef<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_EnumOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum_options", self.base))
    }

    #[doc = "Get a reference to the value of field `multi_select_options` after provisioning.\n"]
    pub fn multi_select_options(
        &self,
    ) -> terrars::ListRef<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate_MultiSelectOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multi_select_options", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccount {
    service_account: terrars::PrimField<std::string::String>,
}

impl ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccount { }

impl terrars::ToListMappable for ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccount {
    type O = terrars::BlockAssignable<ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccount {
    #[doc =
        "The service account to be used for authenticating request.\n\nThe 'iam.serviceAccounts.getAccessToken' permission should be granted on\nthis service account to the impersonator service account."]
    pub service_account: terrars::PrimField<std::string::String>,
}

impl BuildApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccount {
    pub fn build(self) -> ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccount {
        ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccount { service_account: self.service_account }
    }
}

#[allow(dead_code)]
pub struct ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccountRef {
        ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccountRef {
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

#[derive(Serialize, Default)]
struct ApihubPlugin_ConfigTemplate_AuthConfigTemplateDynamic {
    service_account: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccount>>,
}

#[derive(Serialize)]
pub struct ApihubPlugin_ConfigTemplate_AuthConfigTemplate {
    supported_auth_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<std::vec::Vec<ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccount>>,
    dynamic: ApihubPlugin_ConfigTemplate_AuthConfigTemplateDynamic,
}

impl ApihubPlugin_ConfigTemplate_AuthConfigTemplate {
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccount>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_account = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_account = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApihubPlugin_ConfigTemplate_AuthConfigTemplate {
    type O = terrars::BlockAssignable<ApihubPlugin_ConfigTemplate_AuthConfigTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPlugin_ConfigTemplate_AuthConfigTemplate {
    #[doc = "The list of authentication types supported by the plugin."]
    pub supported_auth_types: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildApihubPlugin_ConfigTemplate_AuthConfigTemplate {
    pub fn build(self) -> ApihubPlugin_ConfigTemplate_AuthConfigTemplate {
        ApihubPlugin_ConfigTemplate_AuthConfigTemplate {
            supported_auth_types: self.supported_auth_types,
            service_account: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPlugin_ConfigTemplate_AuthConfigTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPlugin_ConfigTemplate_AuthConfigTemplateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApihubPlugin_ConfigTemplate_AuthConfigTemplateRef {
        ApihubPlugin_ConfigTemplate_AuthConfigTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPlugin_ConfigTemplate_AuthConfigTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `supported_auth_types` after provisioning.\nThe list of authentication types supported by the plugin."]
    pub fn supported_auth_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_auth_types", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::ListRef<ApihubPlugin_ConfigTemplate_AuthConfigTemplate_ServiceAccountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApihubPlugin_ConfigTemplateDynamic {
    additional_config_template: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate>>,
    auth_config_template: std
    ::option
    ::Option<terrars::DynamicBlock<ApihubPlugin_ConfigTemplate_AuthConfigTemplate>>,
}

#[derive(Serialize)]
pub struct ApihubPlugin_ConfigTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_config_template: std::option::Option<std::vec::Vec<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_config_template: std::option::Option<std::vec::Vec<ApihubPlugin_ConfigTemplate_AuthConfigTemplate>>,
    dynamic: ApihubPlugin_ConfigTemplateDynamic,
}

impl ApihubPlugin_ConfigTemplate {
    #[doc = "Set the field `additional_config_template`.\n"]
    pub fn set_additional_config_template(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.additional_config_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.additional_config_template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auth_config_template`.\n"]
    pub fn set_auth_config_template(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApihubPlugin_ConfigTemplate_AuthConfigTemplate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auth_config_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auth_config_template = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApihubPlugin_ConfigTemplate {
    type O = terrars::BlockAssignable<ApihubPlugin_ConfigTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPlugin_ConfigTemplate {}

impl BuildApihubPlugin_ConfigTemplate {
    pub fn build(self) -> ApihubPlugin_ConfigTemplate {
        ApihubPlugin_ConfigTemplate {
            additional_config_template: core::default::Default::default(),
            auth_config_template: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPlugin_ConfigTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPlugin_ConfigTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApihubPlugin_ConfigTemplateRef {
        ApihubPlugin_ConfigTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPlugin_ConfigTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_config_template` after provisioning.\n"]
    pub fn additional_config_template(
        &self,
    ) -> terrars::ListRef<ApihubPlugin_ConfigTemplate_AdditionalConfigTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_config_template", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_config_template` after provisioning.\n"]
    pub fn auth_config_template(&self) -> terrars::ListRef<ApihubPlugin_ConfigTemplate_AuthConfigTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_config_template", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPlugin_Documentation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApihubPlugin_Documentation {
    #[doc = "Set the field `external_uri`.\nThe uri of the externally hosted documentation."]
    pub fn set_external_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.external_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApihubPlugin_Documentation {
    type O = terrars::BlockAssignable<ApihubPlugin_Documentation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPlugin_Documentation {}

impl BuildApihubPlugin_Documentation {
    pub fn build(self) -> ApihubPlugin_Documentation {
        ApihubPlugin_Documentation { external_uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApihubPlugin_DocumentationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPlugin_DocumentationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApihubPlugin_DocumentationRef {
        ApihubPlugin_DocumentationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPlugin_DocumentationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external_uri` after provisioning.\nThe uri of the externally hosted documentation."]
    pub fn external_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPlugin_HostingService {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApihubPlugin_HostingService {
    #[doc =
        "Set the field `service_uri`.\nThe URI of the service implemented by the plugin developer, used to\ninvoke the plugin's functionality. This information is only required for\nuser defined plugins."]
    pub fn set_service_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApihubPlugin_HostingService {
    type O = terrars::BlockAssignable<ApihubPlugin_HostingService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPlugin_HostingService {}

impl BuildApihubPlugin_HostingService {
    pub fn build(self) -> ApihubPlugin_HostingService {
        ApihubPlugin_HostingService { service_uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApihubPlugin_HostingServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPlugin_HostingServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApihubPlugin_HostingServiceRef {
        ApihubPlugin_HostingServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPlugin_HostingServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `service_uri` after provisioning.\nThe URI of the service implemented by the plugin developer, used to\ninvoke the plugin's functionality. This information is only required for\nuser defined plugins."]
    pub fn service_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ApihubPlugin_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApihubPlugin_Timeouts {
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

impl terrars::ToListMappable for ApihubPlugin_Timeouts {
    type O = terrars::BlockAssignable<ApihubPlugin_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApihubPlugin_Timeouts {}

impl BuildApihubPlugin_Timeouts {
    pub fn build(self) -> ApihubPlugin_Timeouts {
        ApihubPlugin_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApihubPlugin_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApihubPlugin_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApihubPlugin_TimeoutsRef {
        ApihubPlugin_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApihubPlugin_TimeoutsRef {
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
