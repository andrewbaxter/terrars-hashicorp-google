#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct FirebaseRemoteConfigRemoteConfigData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<std::vec::Vec<FirebaseRemoteConfigRemoteConfig_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_groups: std::option::Option<std::vec::Vec<FirebaseRemoteConfigRemoteConfig_ParameterGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<FirebaseRemoteConfigRemoteConfig_Parameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FirebaseRemoteConfigRemoteConfig_Timeouts>,
    dynamic: FirebaseRemoteConfigRemoteConfigDynamic,
}

struct FirebaseRemoteConfigRemoteConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FirebaseRemoteConfigRemoteConfigData_>,
}

#[derive(Clone)]
pub struct FirebaseRemoteConfigRemoteConfig(std::rc::Rc<FirebaseRemoteConfigRemoteConfig_>);

impl FirebaseRemoteConfigRemoteConfig {
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

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Conditions>>,
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

    #[doc = "Set the field `parameter_groups`.\n"]
    pub fn set_parameter_groups(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_ParameterGroups>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameter_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameter_groups = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Parameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FirebaseRemoteConfigRemoteConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the RemoteConfig.\nFormat: projects/{project}/namespaces/{namespace}/remoteConfig\nProject is a Firebase project ID or project number.\nNamespace is the namespace ID (e.g.: firebase or firebase-server)"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nContains all metadata about a particular version of the Remote Config\ntemplate.\n\nAll fields are set at the time the specified Remote Config template was\nwritten."]
    pub fn version(&self) -> terrars::ListRef<FirebaseRemoteConfigRemoteConfig_VersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<FirebaseRemoteConfigRemoteConfig_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseRemoteConfigRemoteConfig_TimeoutsRef {
        FirebaseRemoteConfigRemoteConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for FirebaseRemoteConfigRemoteConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FirebaseRemoteConfigRemoteConfig { }

impl terrars::ToListMappable for FirebaseRemoteConfigRemoteConfig {
    type O = terrars::ListRef<FirebaseRemoteConfigRemoteConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FirebaseRemoteConfigRemoteConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_firebase_remote_config_remote_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFirebaseRemoteConfigRemoteConfig {
    pub tf_id: std::string::String,
}

impl BuildFirebaseRemoteConfigRemoteConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> FirebaseRemoteConfigRemoteConfig {
        let out = FirebaseRemoteConfigRemoteConfig(std::rc::Rc::new(FirebaseRemoteConfigRemoteConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FirebaseRemoteConfigRemoteConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                conditions: core::default::Default::default(),
                parameter_groups: core::default::Default::default(),
                parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FirebaseRemoteConfigRemoteConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseRemoteConfigRemoteConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FirebaseRemoteConfigRemoteConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the RemoteConfig.\nFormat: projects/{project}/namespaces/{namespace}/remoteConfig\nProject is a Firebase project ID or project number.\nNamespace is the namespace ID (e.g.: firebase or firebase-server)"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nContains all metadata about a particular version of the Remote Config\ntemplate.\n\nAll fields are set at the time the specified Remote Config template was\nwritten."]
    pub fn version(&self) -> terrars::ListRef<FirebaseRemoteConfigRemoteConfig_VersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<FirebaseRemoteConfigRemoteConfig_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FirebaseRemoteConfigRemoteConfig_TimeoutsRef {
        FirebaseRemoteConfigRemoteConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct FirebaseRemoteConfigRemoteConfigDynamic {
    conditions: std::option::Option<terrars::DynamicBlock<FirebaseRemoteConfigRemoteConfig_Conditions>>,
    parameter_groups: std::option::Option<terrars::DynamicBlock<FirebaseRemoteConfigRemoteConfig_ParameterGroups>>,
    parameters: std::option::Option<terrars::DynamicBlock<FirebaseRemoteConfigRemoteConfig_Parameters>>,
}

#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfig_Version_UpdateUser {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseRemoteConfigRemoteConfig_Version_UpdateUser {
    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_url`.\n"]
    pub fn set_image_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseRemoteConfigRemoteConfig_Version_UpdateUser {
    type O = terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Version_UpdateUser>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseRemoteConfigRemoteConfig_Version_UpdateUser {}

impl BuildFirebaseRemoteConfigRemoteConfig_Version_UpdateUser {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfig_Version_UpdateUser {
        FirebaseRemoteConfigRemoteConfig_Version_UpdateUser {
            email: core::default::Default::default(),
            image_url: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseRemoteConfigRemoteConfig_Version_UpdateUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseRemoteConfigRemoteConfig_Version_UpdateUserRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseRemoteConfigRemoteConfig_Version_UpdateUserRef {
        FirebaseRemoteConfigRemoteConfig_Version_UpdateUserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseRemoteConfigRemoteConfig_Version_UpdateUserRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `image_url` after provisioning.\n"]
    pub fn image_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_url", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfig_Version {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_legacy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback_source: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_origin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_user: std::option::Option<terrars::ListField<FirebaseRemoteConfigRemoteConfig_Version_UpdateUser>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_number: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseRemoteConfigRemoteConfig_Version {
    #[doc = "Set the field `is_legacy`.\n"]
    pub fn set_is_legacy(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_legacy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollback_source`.\n"]
    pub fn set_rollback_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rollback_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_origin`.\n"]
    pub fn set_update_origin(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_type`.\n"]
    pub fn set_update_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_user`.\n"]
    pub fn set_update_user(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FirebaseRemoteConfigRemoteConfig_Version_UpdateUser>>,
    ) -> Self {
        self.update_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_number`.\n"]
    pub fn set_version_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseRemoteConfigRemoteConfig_Version {
    type O = terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Version>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseRemoteConfigRemoteConfig_Version {}

impl BuildFirebaseRemoteConfigRemoteConfig_Version {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfig_Version {
        FirebaseRemoteConfigRemoteConfig_Version {
            is_legacy: core::default::Default::default(),
            rollback_source: core::default::Default::default(),
            update_origin: core::default::Default::default(),
            update_time: core::default::Default::default(),
            update_type: core::default::Default::default(),
            update_user: core::default::Default::default(),
            version_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseRemoteConfigRemoteConfig_VersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseRemoteConfigRemoteConfig_VersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseRemoteConfigRemoteConfig_VersionRef {
        FirebaseRemoteConfigRemoteConfig_VersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseRemoteConfigRemoteConfig_VersionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_legacy` after provisioning.\n"]
    pub fn is_legacy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_legacy", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback_source` after provisioning.\n"]
    pub fn rollback_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollback_source", self.base))
    }

    #[doc = "Get a reference to the value of field `update_origin` after provisioning.\n"]
    pub fn update_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_origin", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `update_type` after provisioning.\n"]
    pub fn update_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_type", self.base))
    }

    #[doc = "Get a reference to the value of field `update_user` after provisioning.\n"]
    pub fn update_user(&self) -> terrars::ListRef<FirebaseRemoteConfigRemoteConfig_Version_UpdateUserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.update_user", self.base))
    }

    #[doc = "Get a reference to the value of field `version_number` after provisioning.\n"]
    pub fn version_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_number", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfig_Conditions {
    expression: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_color: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseRemoteConfigRemoteConfig_Conditions {
    #[doc =
        "Set the field `tag_color`.\nThe color associated with this condition for display purposes in the Firebase Console.\nNot specifying this value results in the Console picking an arbitrary color to associate with the condition. Possible values: [\"BLUE\", \"BROWN\", \"CYAN\", \"DEEP_ORANGE\", \"GREEN\", \"INDIGO\", \"LIME\", \"ORANGE\", \"PINK\", \"PURPLE\", \"TEAL\"]"]
    pub fn set_tag_color(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_color = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseRemoteConfigRemoteConfig_Conditions {
    type O = terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseRemoteConfigRemoteConfig_Conditions {
    #[doc =
        "The logic of this condition.\n\nSee the documentation regarding\n[Condition\nExpressions](https://firebase.google.com/docs/remote-config/condition-reference)\nfor the expected syntax of this field."]
    pub expression: terrars::PrimField<std::string::String>,
    #[doc = "A non-empty and unique name of this condition."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildFirebaseRemoteConfigRemoteConfig_Conditions {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfig_Conditions {
        FirebaseRemoteConfigRemoteConfig_Conditions {
            expression: self.expression,
            name: self.name,
            tag_color: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseRemoteConfigRemoteConfig_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseRemoteConfigRemoteConfig_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseRemoteConfigRemoteConfig_ConditionsRef {
        FirebaseRemoteConfigRemoteConfig_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseRemoteConfigRemoteConfig_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expression` after provisioning.\nThe logic of this condition.\n\nSee the documentation regarding\n[Condition\nExpressions](https://firebase.google.com/docs/remote-config/condition-reference)\nfor the expected syntax of this field."]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA non-empty and unique name of this condition."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag_color` after provisioning.\nThe color associated with this condition for display purposes in the Firebase Console.\nNot specifying this value results in the Console picking an arbitrary color to associate with the condition. Possible values: [\"BLUE\", \"BROWN\", \"CYAN\", \"DEEP_ORANGE\", \"GREEN\", \"INDIGO\", \"LIME\", \"ORANGE\", \"PINK\", \"PURPLE\", \"TEAL\"]"]
    pub fn tag_color(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_color", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValues {
    condition_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_in_app_default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValues {
    #[doc =
        "Set the field `use_in_app_default`.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn set_use_in_app_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_in_app_default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe string value that the parameter is set to."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValues {
    type O = terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValues>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValues {
    #[doc = ""]
    pub condition_name: terrars::PrimField<std::string::String>,
}

impl BuildFirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValues {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValues {
        FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValues {
            condition_name: self.condition_name,
            use_in_app_default: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValuesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValuesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValuesRef {
        FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValuesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValuesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition_name` after provisioning.\n"]
    pub fn condition_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `use_in_app_default` after provisioning.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn use_in_app_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_in_app_default", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe string value that the parameter is set to."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_in_app_default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValue {
    #[doc =
        "Set the field `use_in_app_default`.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn set_use_in_app_default(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_in_app_default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nThe string value that the parameter is set to."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValue {
    type O = terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseRemoteConfigRemoteConfig_Parameters_DefaultValue {}

impl BuildFirebaseRemoteConfigRemoteConfig_Parameters_DefaultValue {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValue {
        FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValue {
            use_in_app_default: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValueRef {
        FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `use_in_app_default` after provisioning.\nIf true, the parameter is omitted from the parameter values returned\nto a client."]
    pub fn use_in_app_default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_in_app_default", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nThe string value that the parameter is set to."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct FirebaseRemoteConfigRemoteConfig_ParametersDynamic {
    conditional_values: std
    ::option
    ::Option<terrars::DynamicBlock<FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValues>>,
    default_value: std
    ::option
    ::Option<terrars::DynamicBlock<FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValue>>,
}

#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfig_Parameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    parameter_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditional_values: std::option::Option<std::vec::Vec<FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValues>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_value: std::option::Option<std::vec::Vec<FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValue>>,
    dynamic: FirebaseRemoteConfigRemoteConfig_ParametersDynamic,
}

impl FirebaseRemoteConfigRemoteConfig_Parameters {
    #[doc =
        "Set the field `description`.\nA description for this Parameter. Its length must be less than or equal to\n256 characters . A description may contain any Unicode characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value_type`.\nThe data type for all values of this parameter in the current version of\nthe template. Default value: \"STRING\" Possible values: [\"STRING\", \"BOOLEAN\", \"NUMBER\", \"JSON\"]"]
    pub fn set_value_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditional_values`.\n"]
    pub fn set_conditional_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Parameters_ConditionalValues>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditional_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditional_values = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_value`.\n"]
    pub fn set_default_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FirebaseRemoteConfigRemoteConfig_Parameters {
    type O = terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Parameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseRemoteConfigRemoteConfig_Parameters {
    #[doc = ""]
    pub parameter_name: terrars::PrimField<std::string::String>,
}

impl BuildFirebaseRemoteConfigRemoteConfig_Parameters {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfig_Parameters {
        FirebaseRemoteConfigRemoteConfig_Parameters {
            description: core::default::Default::default(),
            parameter_name: self.parameter_name,
            value_type: core::default::Default::default(),
            conditional_values: core::default::Default::default(),
            default_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseRemoteConfigRemoteConfig_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseRemoteConfigRemoteConfig_ParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseRemoteConfigRemoteConfig_ParametersRef {
        FirebaseRemoteConfigRemoteConfig_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseRemoteConfigRemoteConfig_ParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description for this Parameter. Its length must be less than or equal to\n256 characters . A description may contain any Unicode characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter_name` after provisioning.\n"]
    pub fn parameter_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value_type` after provisioning.\nThe data type for all values of this parameter in the current version of\nthe template. Default value: \"STRING\" Possible values: [\"STRING\", \"BOOLEAN\", \"NUMBER\", \"JSON\"]"]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::ListRef<FirebaseRemoteConfigRemoteConfig_Parameters_DefaultValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct FirebaseRemoteConfigRemoteConfig_ParameterGroupsDynamic {
    parameters: std::option::Option<terrars::DynamicBlock<FirebaseRemoteConfigRemoteConfig_Parameters>>,
}

#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfig_ParameterGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    parameter_group_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<FirebaseRemoteConfigRemoteConfig_Parameters>>,
    dynamic: FirebaseRemoteConfigRemoteConfig_ParameterGroupsDynamic,
}

impl FirebaseRemoteConfigRemoteConfig_ParameterGroups {
    #[doc =
        "Set the field `description`.\nA description for the group. Its length must be less than or equal to 256\ncharacters. A description may contain any Unicode characters."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Parameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FirebaseRemoteConfigRemoteConfig_ParameterGroups {
    type O = terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_ParameterGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseRemoteConfigRemoteConfig_ParameterGroups {
    #[doc = ""]
    pub parameter_group_name: terrars::PrimField<std::string::String>,
}

impl BuildFirebaseRemoteConfigRemoteConfig_ParameterGroups {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfig_ParameterGroups {
        FirebaseRemoteConfigRemoteConfig_ParameterGroups {
            description: core::default::Default::default(),
            parameter_group_name: self.parameter_group_name,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseRemoteConfigRemoteConfig_ParameterGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseRemoteConfigRemoteConfig_ParameterGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FirebaseRemoteConfigRemoteConfig_ParameterGroupsRef {
        FirebaseRemoteConfigRemoteConfig_ParameterGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseRemoteConfigRemoteConfig_ParameterGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description for the group. Its length must be less than or equal to 256\ncharacters. A description may contain any Unicode characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `parameter_group_name` after provisioning.\n"]
    pub fn parameter_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameter_group_name", self.base))
    }
}

#[derive(Serialize)]
pub struct FirebaseRemoteConfigRemoteConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FirebaseRemoteConfigRemoteConfig_Timeouts {
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

impl terrars::ToListMappable for FirebaseRemoteConfigRemoteConfig_Timeouts {
    type O = terrars::BlockAssignable<FirebaseRemoteConfigRemoteConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFirebaseRemoteConfigRemoteConfig_Timeouts {}

impl BuildFirebaseRemoteConfigRemoteConfig_Timeouts {
    pub fn build(self) -> FirebaseRemoteConfigRemoteConfig_Timeouts {
        FirebaseRemoteConfigRemoteConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FirebaseRemoteConfigRemoteConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FirebaseRemoteConfigRemoteConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FirebaseRemoteConfigRemoteConfig_TimeoutsRef {
        FirebaseRemoteConfigRemoteConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FirebaseRemoteConfigRemoteConfig_TimeoutsRef {
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
