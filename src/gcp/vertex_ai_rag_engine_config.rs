#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiRagEngineConfigData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rag_managed_db_config: std::option::Option<std::vec::Vec<VertexAiRagEngineConfig_RagManagedDbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiRagEngineConfig_Timeouts>,
    dynamic: VertexAiRagEngineConfigDynamic,
}

struct VertexAiRagEngineConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiRagEngineConfigData_>,
}

#[derive(Clone)]
pub struct VertexAiRagEngineConfig(std::rc::Rc<VertexAiRagEngineConfig_>);

impl VertexAiRagEngineConfig {
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

    #[doc = "Set the field `region`.\nThe region of the RagEngineConfig. eg us-central1"]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rag_managed_db_config`.\n"]
    pub fn set_rag_managed_db_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagEngineConfig_RagManagedDbConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rag_managed_db_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rag_managed_db_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiRagEngineConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Dataset. This value is set by Google."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the RagEngineConfig. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rag_managed_db_config` after provisioning.\n"]
    pub fn rag_managed_db_config(&self) -> terrars::ListRef<VertexAiRagEngineConfig_RagManagedDbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rag_managed_db_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiRagEngineConfig_TimeoutsRef {
        VertexAiRagEngineConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VertexAiRagEngineConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiRagEngineConfig { }

impl terrars::ToListMappable for VertexAiRagEngineConfig {
    type O = terrars::ListRef<VertexAiRagEngineConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiRagEngineConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_rag_engine_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiRagEngineConfig {
    pub tf_id: std::string::String,
}

impl BuildVertexAiRagEngineConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiRagEngineConfig {
        let out = VertexAiRagEngineConfig(std::rc::Rc::new(VertexAiRagEngineConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiRagEngineConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                rag_managed_db_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiRagEngineConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagEngineConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiRagEngineConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Dataset. This value is set by Google."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the RagEngineConfig. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rag_managed_db_config` after provisioning.\n"]
    pub fn rag_managed_db_config(&self) -> terrars::ListRef<VertexAiRagEngineConfig_RagManagedDbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rag_managed_db_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiRagEngineConfig_TimeoutsRef {
        VertexAiRagEngineConfig_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VertexAiRagEngineConfigDynamic {
    rag_managed_db_config: std::option::Option<terrars::DynamicBlock<VertexAiRagEngineConfig_RagManagedDbConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiRagEngineConfig_RagManagedDbConfig_Basic {}

impl VertexAiRagEngineConfig_RagManagedDbConfig_Basic { }

impl terrars::ToListMappable for VertexAiRagEngineConfig_RagManagedDbConfig_Basic {
    type O = terrars::BlockAssignable<VertexAiRagEngineConfig_RagManagedDbConfig_Basic>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagEngineConfig_RagManagedDbConfig_Basic {}

impl BuildVertexAiRagEngineConfig_RagManagedDbConfig_Basic {
    pub fn build(self) -> VertexAiRagEngineConfig_RagManagedDbConfig_Basic {
        VertexAiRagEngineConfig_RagManagedDbConfig_Basic {}
    }
}

#[allow(dead_code)]
pub struct VertexAiRagEngineConfig_RagManagedDbConfig_BasicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagEngineConfig_RagManagedDbConfig_BasicRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiRagEngineConfig_RagManagedDbConfig_BasicRef {
        VertexAiRagEngineConfig_RagManagedDbConfig_BasicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagEngineConfig_RagManagedDbConfig_BasicRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct VertexAiRagEngineConfig_RagManagedDbConfig_Scaled {}

impl VertexAiRagEngineConfig_RagManagedDbConfig_Scaled { }

impl terrars::ToListMappable for VertexAiRagEngineConfig_RagManagedDbConfig_Scaled {
    type O = terrars::BlockAssignable<VertexAiRagEngineConfig_RagManagedDbConfig_Scaled>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagEngineConfig_RagManagedDbConfig_Scaled {}

impl BuildVertexAiRagEngineConfig_RagManagedDbConfig_Scaled {
    pub fn build(self) -> VertexAiRagEngineConfig_RagManagedDbConfig_Scaled {
        VertexAiRagEngineConfig_RagManagedDbConfig_Scaled {}
    }
}

#[allow(dead_code)]
pub struct VertexAiRagEngineConfig_RagManagedDbConfig_ScaledRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagEngineConfig_RagManagedDbConfig_ScaledRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiRagEngineConfig_RagManagedDbConfig_ScaledRef {
        VertexAiRagEngineConfig_RagManagedDbConfig_ScaledRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagEngineConfig_RagManagedDbConfig_ScaledRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct VertexAiRagEngineConfig_RagManagedDbConfig_Unprovisioned {}

impl VertexAiRagEngineConfig_RagManagedDbConfig_Unprovisioned { }

impl terrars::ToListMappable for VertexAiRagEngineConfig_RagManagedDbConfig_Unprovisioned {
    type O = terrars::BlockAssignable<VertexAiRagEngineConfig_RagManagedDbConfig_Unprovisioned>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagEngineConfig_RagManagedDbConfig_Unprovisioned {}

impl BuildVertexAiRagEngineConfig_RagManagedDbConfig_Unprovisioned {
    pub fn build(self) -> VertexAiRagEngineConfig_RagManagedDbConfig_Unprovisioned {
        VertexAiRagEngineConfig_RagManagedDbConfig_Unprovisioned {}
    }
}

#[allow(dead_code)]
pub struct VertexAiRagEngineConfig_RagManagedDbConfig_UnprovisionedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagEngineConfig_RagManagedDbConfig_UnprovisionedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiRagEngineConfig_RagManagedDbConfig_UnprovisionedRef {
        VertexAiRagEngineConfig_RagManagedDbConfig_UnprovisionedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagEngineConfig_RagManagedDbConfig_UnprovisionedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct VertexAiRagEngineConfig_RagManagedDbConfigDynamic {
    basic: std::option::Option<terrars::DynamicBlock<VertexAiRagEngineConfig_RagManagedDbConfig_Basic>>,
    scaled: std::option::Option<terrars::DynamicBlock<VertexAiRagEngineConfig_RagManagedDbConfig_Scaled>>,
    unprovisioned: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiRagEngineConfig_RagManagedDbConfig_Unprovisioned>>,
}

#[derive(Serialize)]
pub struct VertexAiRagEngineConfig_RagManagedDbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic: std::option::Option<std::vec::Vec<VertexAiRagEngineConfig_RagManagedDbConfig_Basic>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaled: std::option::Option<std::vec::Vec<VertexAiRagEngineConfig_RagManagedDbConfig_Scaled>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unprovisioned: std::option::Option<std::vec::Vec<VertexAiRagEngineConfig_RagManagedDbConfig_Unprovisioned>>,
    dynamic: VertexAiRagEngineConfig_RagManagedDbConfigDynamic,
}

impl VertexAiRagEngineConfig_RagManagedDbConfig {
    #[doc = "Set the field `basic`.\n"]
    pub fn set_basic(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagEngineConfig_RagManagedDbConfig_Basic>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scaled`.\n"]
    pub fn set_scaled(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagEngineConfig_RagManagedDbConfig_Scaled>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scaled = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scaled = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unprovisioned`.\n"]
    pub fn set_unprovisioned(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiRagEngineConfig_RagManagedDbConfig_Unprovisioned>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.unprovisioned = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.unprovisioned = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiRagEngineConfig_RagManagedDbConfig {
    type O = terrars::BlockAssignable<VertexAiRagEngineConfig_RagManagedDbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagEngineConfig_RagManagedDbConfig {}

impl BuildVertexAiRagEngineConfig_RagManagedDbConfig {
    pub fn build(self) -> VertexAiRagEngineConfig_RagManagedDbConfig {
        VertexAiRagEngineConfig_RagManagedDbConfig {
            basic: core::default::Default::default(),
            scaled: core::default::Default::default(),
            unprovisioned: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagEngineConfig_RagManagedDbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagEngineConfig_RagManagedDbConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiRagEngineConfig_RagManagedDbConfigRef {
        VertexAiRagEngineConfig_RagManagedDbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagEngineConfig_RagManagedDbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]
    pub fn basic(&self) -> terrars::ListRef<VertexAiRagEngineConfig_RagManagedDbConfig_BasicRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic", self.base))
    }

    #[doc = "Get a reference to the value of field `scaled` after provisioning.\n"]
    pub fn scaled(&self) -> terrars::ListRef<VertexAiRagEngineConfig_RagManagedDbConfig_ScaledRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaled", self.base))
    }

    #[doc = "Get a reference to the value of field `unprovisioned` after provisioning.\n"]
    pub fn unprovisioned(&self) -> terrars::ListRef<VertexAiRagEngineConfig_RagManagedDbConfig_UnprovisionedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unprovisioned", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiRagEngineConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiRagEngineConfig_Timeouts {
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

impl terrars::ToListMappable for VertexAiRagEngineConfig_Timeouts {
    type O = terrars::BlockAssignable<VertexAiRagEngineConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagEngineConfig_Timeouts {}

impl BuildVertexAiRagEngineConfig_Timeouts {
    pub fn build(self) -> VertexAiRagEngineConfig_Timeouts {
        VertexAiRagEngineConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagEngineConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagEngineConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiRagEngineConfig_TimeoutsRef {
        VertexAiRagEngineConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagEngineConfig_TimeoutsRef {
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
