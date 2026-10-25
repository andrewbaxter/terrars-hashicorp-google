#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ArtifactRegistryProjectConfigData_ {
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
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_logs_config: std::option::Option<std::vec::Vec<ArtifactRegistryProjectConfig_PlatformLogsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ArtifactRegistryProjectConfig_Timeouts>,
    dynamic: ArtifactRegistryProjectConfigDynamic,
}

struct ArtifactRegistryProjectConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ArtifactRegistryProjectConfigData_>,
}

#[derive(Clone)]
pub struct ArtifactRegistryProjectConfig(std::rc::Rc<ArtifactRegistryProjectConfig_>);

impl ArtifactRegistryProjectConfig {
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

    #[doc = "Set the field `location`.\nThe name of the location this config is located in."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform_logs_config`.\n"]
    pub fn set_platform_logs_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ArtifactRegistryProjectConfig_PlatformLogsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().platform_logs_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.platform_logs_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ArtifactRegistryProjectConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this config is located in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the project's config.\nAlways of the form: projects/{project}/locations/{location}/projectConfig"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_logs_config` after provisioning.\n"]
    pub fn platform_logs_config(&self) -> terrars::ListRef<ArtifactRegistryProjectConfig_PlatformLogsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.platform_logs_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ArtifactRegistryProjectConfig_TimeoutsRef {
        ArtifactRegistryProjectConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ArtifactRegistryProjectConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ArtifactRegistryProjectConfig { }

impl terrars::ToListMappable for ArtifactRegistryProjectConfig {
    type O = terrars::ListRef<ArtifactRegistryProjectConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ArtifactRegistryProjectConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_artifact_registry_project_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildArtifactRegistryProjectConfig {
    pub tf_id: std::string::String,
}

impl BuildArtifactRegistryProjectConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> ArtifactRegistryProjectConfig {
        let out = ArtifactRegistryProjectConfig(std::rc::Rc::new(ArtifactRegistryProjectConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ArtifactRegistryProjectConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                platform_logs_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ArtifactRegistryProjectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryProjectConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ArtifactRegistryProjectConfigRef {
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
        "Get a reference to the value of field `location` after provisioning.\nThe name of the location this config is located in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the project's config.\nAlways of the form: projects/{project}/locations/{location}/projectConfig"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_logs_config` after provisioning.\n"]
    pub fn platform_logs_config(&self) -> terrars::ListRef<ArtifactRegistryProjectConfig_PlatformLogsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.platform_logs_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ArtifactRegistryProjectConfig_TimeoutsRef {
        ArtifactRegistryProjectConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ArtifactRegistryProjectConfigDynamic {
    platform_logs_config: std
    ::option
    ::Option<terrars::DynamicBlock<ArtifactRegistryProjectConfig_PlatformLogsConfig>>,
}

#[derive(Serialize)]
pub struct ArtifactRegistryProjectConfig_PlatformLogsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity_level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArtifactRegistryProjectConfig_PlatformLogsConfig {
    #[doc =
        "Set the field `logging_state`.\nThe state of the platform logs: enabled or disabled. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn set_logging_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logging_state = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `severity_level`.\nThe severity level for the logs. Logs will be generated if their\nseverity level is >= than the value of the severity level mentioned here. Possible values: [\"DEBUG\", \"INFO\", \"NOTICE\", \"WARNING\", \"ERROR\", \"CRITICAL\", \"ALERT\", \"EMERGENCY\"]"]
    pub fn set_severity_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.severity_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ArtifactRegistryProjectConfig_PlatformLogsConfig {
    type O = terrars::BlockAssignable<ArtifactRegistryProjectConfig_PlatformLogsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryProjectConfig_PlatformLogsConfig {}

impl BuildArtifactRegistryProjectConfig_PlatformLogsConfig {
    pub fn build(self) -> ArtifactRegistryProjectConfig_PlatformLogsConfig {
        ArtifactRegistryProjectConfig_PlatformLogsConfig {
            logging_state: core::default::Default::default(),
            severity_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryProjectConfig_PlatformLogsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryProjectConfig_PlatformLogsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ArtifactRegistryProjectConfig_PlatformLogsConfigRef {
        ArtifactRegistryProjectConfig_PlatformLogsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryProjectConfig_PlatformLogsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `logging_state` after provisioning.\nThe state of the platform logs: enabled or disabled. Possible values: [\"ENABLED\", \"DISABLED\"]"]
    pub fn logging_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `severity_level` after provisioning.\nThe severity level for the logs. Logs will be generated if their\nseverity level is >= than the value of the severity level mentioned here. Possible values: [\"DEBUG\", \"INFO\", \"NOTICE\", \"WARNING\", \"ERROR\", \"CRITICAL\", \"ALERT\", \"EMERGENCY\"]"]
    pub fn severity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity_level", self.base))
    }
}

#[derive(Serialize)]
pub struct ArtifactRegistryProjectConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ArtifactRegistryProjectConfig_Timeouts {
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

impl terrars::ToListMappable for ArtifactRegistryProjectConfig_Timeouts {
    type O = terrars::BlockAssignable<ArtifactRegistryProjectConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildArtifactRegistryProjectConfig_Timeouts {}

impl BuildArtifactRegistryProjectConfig_Timeouts {
    pub fn build(self) -> ArtifactRegistryProjectConfig_Timeouts {
        ArtifactRegistryProjectConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ArtifactRegistryProjectConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ArtifactRegistryProjectConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ArtifactRegistryProjectConfig_TimeoutsRef {
        ArtifactRegistryProjectConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ArtifactRegistryProjectConfig_TimeoutsRef {
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
