#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VertexAiRagCorpusData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_spec: std::option::Option<std::vec::Vec<VertexAiRagCorpus_EncryptionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VertexAiRagCorpus_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vector_db_config: std::option::Option<std::vec::Vec<VertexAiRagCorpus_VectorDbConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vertex_ai_search_config: std::option::Option<std::vec::Vec<VertexAiRagCorpus_VertexAiSearchConfig>>,
    dynamic: VertexAiRagCorpusDynamic,
}

struct VertexAiRagCorpus_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VertexAiRagCorpusData_>,
}

#[derive(Clone)]
pub struct VertexAiRagCorpus(std::rc::Rc<VertexAiRagCorpus_>);

impl VertexAiRagCorpus {
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

    #[doc = "Set the field `description`.\nOptional. The description of the RagCorpus."]
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

    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagCorpus_EncryptionSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VertexAiRagCorpus_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vector_db_config`.\n"]
    pub fn set_vector_db_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vector_db_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vector_db_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vertex_ai_search_config`.\n"]
    pub fn set_vertex_ai_search_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagCorpus_VertexAiSearchConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vertex_ai_search_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vertex_ai_search_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `corpus_status` after provisioning.\nOutput only. RagCorpus state."]
    pub fn corpus_status(&self) -> terrars::ListRef<VertexAiRagCorpus_CorpusStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.corpus_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when this RagCorpus was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. The description of the RagCorpus."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the RagCorpus. The name can be up to 128\ncharacters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe generated name of the RagCorpus, in the format\nprojects/{project}/locations/{location}/ragCorpora/{rag_corpus}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the RagCorpus. eg europe-west4"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when this RagCorpus was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiRagCorpus_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiRagCorpus_TimeoutsRef {
        VertexAiRagCorpus_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vector_db_config` after provisioning.\n"]
    pub fn vector_db_config(&self) -> terrars::ListRef<VertexAiRagCorpus_VectorDbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vector_db_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vertex_ai_search_config` after provisioning.\n"]
    pub fn vertex_ai_search_config(&self) -> terrars::ListRef<VertexAiRagCorpus_VertexAiSearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vertex_ai_search_config", self.extract_ref()))
    }
}

impl terrars::Referable for VertexAiRagCorpus {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VertexAiRagCorpus { }

impl terrars::ToListMappable for VertexAiRagCorpus {
    type O = terrars::ListRef<VertexAiRagCorpusRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VertexAiRagCorpus_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vertex_ai_rag_corpus".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVertexAiRagCorpus {
    pub tf_id: std::string::String,
    #[doc =
        "Required. The display name of the RagCorpus. The name can be up to 128\ncharacters long and can consist of any UTF-8 characters."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The region of the RagCorpus. eg europe-west4"]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiRagCorpus {
    pub fn build(self, stack: &mut terrars::Stack) -> VertexAiRagCorpus {
        let out = VertexAiRagCorpus(std::rc::Rc::new(VertexAiRagCorpus_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VertexAiRagCorpusData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: self.region,
                encryption_spec: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vector_db_config: core::default::Default::default(),
                vertex_ai_search_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VertexAiRagCorpusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VertexAiRagCorpusRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `corpus_status` after provisioning.\nOutput only. RagCorpus state."]
    pub fn corpus_status(&self) -> terrars::ListRef<VertexAiRagCorpus_CorpusStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.corpus_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when this RagCorpus was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional. The description of the RagCorpus."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the RagCorpus. The name can be up to 128\ncharacters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe generated name of the RagCorpus, in the format\nprojects/{project}/locations/{location}/ragCorpora/{rag_corpus}."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the RagCorpus. eg europe-west4"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when this RagCorpus was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<VertexAiRagCorpus_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VertexAiRagCorpus_TimeoutsRef {
        VertexAiRagCorpus_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vector_db_config` after provisioning.\n"]
    pub fn vector_db_config(&self) -> terrars::ListRef<VertexAiRagCorpus_VectorDbConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vector_db_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vertex_ai_search_config` after provisioning.\n"]
    pub fn vertex_ai_search_config(&self) -> terrars::ListRef<VertexAiRagCorpus_VertexAiSearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vertex_ai_search_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VertexAiRagCorpusDynamic {
    encryption_spec: std::option::Option<terrars::DynamicBlock<VertexAiRagCorpus_EncryptionSpec>>,
    vector_db_config: std::option::Option<terrars::DynamicBlock<VertexAiRagCorpus_VectorDbConfig>>,
    vertex_ai_search_config: std::option::Option<terrars::DynamicBlock<VertexAiRagCorpus_VertexAiSearchConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_CorpusStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiRagCorpus_CorpusStatus {
    #[doc = "Set the field `error_status`.\n"]
    pub fn set_error_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.error_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiRagCorpus_CorpusStatus {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_CorpusStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_CorpusStatus {}

impl BuildVertexAiRagCorpus_CorpusStatus {
    pub fn build(self) -> VertexAiRagCorpus_CorpusStatus {
        VertexAiRagCorpus_CorpusStatus {
            error_status: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_CorpusStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_CorpusStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiRagCorpus_CorpusStatusRef {
        VertexAiRagCorpus_CorpusStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_CorpusStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `error_status` after provisioning.\n"]
    pub fn error_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_status", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_EncryptionSpec {
    kms_key_name: terrars::PrimField<std::string::String>,
}

impl VertexAiRagCorpus_EncryptionSpec { }

impl terrars::ToListMappable for VertexAiRagCorpus_EncryptionSpec {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_EncryptionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_EncryptionSpec {
    #[doc =
        "Required. The Cloud KMS resource identifier of the customer managed\nencryption key used to protect the resource. Has the form:\nprojects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key.\nThe key needs to be in the same region as where the resource is\ncreated."]
    pub kms_key_name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiRagCorpus_EncryptionSpec {
    pub fn build(self) -> VertexAiRagCorpus_EncryptionSpec {
        VertexAiRagCorpus_EncryptionSpec { kms_key_name: self.kms_key_name }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_EncryptionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_EncryptionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiRagCorpus_EncryptionSpecRef {
        VertexAiRagCorpus_EncryptionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_EncryptionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nRequired. The Cloud KMS resource identifier of the customer managed\nencryption key used to protect the resource. Has the form:\nprojects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key.\nThe key needs to be in the same region as where the resource is\ncreated."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiRagCorpus_Timeouts {
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

impl terrars::ToListMappable for VertexAiRagCorpus_Timeouts {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_Timeouts {}

impl BuildVertexAiRagCorpus_Timeouts {
    pub fn build(self) -> VertexAiRagCorpus_Timeouts {
        VertexAiRagCorpus_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiRagCorpus_TimeoutsRef {
        VertexAiRagCorpus_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_TimeoutsRef {
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
pub struct VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_string: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfig {
    #[doc =
        "Set the field `api_key_secret_version`.\nThe SecretManager secret version resource name storing API key.\ne.g. projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn set_api_key_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.api_key_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `api_key_string`.\nThe API key string."]
    pub fn set_api_key_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api_key_string = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfig {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfig {}

impl BuildVertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfig {
    pub fn build(self) -> VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfig {
        VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfig {
            api_key_secret_version: core::default::Default::default(),
            api_key_string: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfigRef {
        VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `api_key_secret_version` after provisioning.\nThe SecretManager secret version resource name storing API key.\ne.g. projects/{project}/secrets/{secret}/versions/{version}"]
    pub fn api_key_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `api_key_string` after provisioning.\nThe API key string."]
    pub fn api_key_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_string", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiRagCorpus_VectorDbConfig_ApiAuthDynamic {
    api_key_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfig>>,
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_VectorDbConfig_ApiAuth {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_config: std::option::Option<std::vec::Vec<VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfig>>,
    dynamic: VertexAiRagCorpus_VectorDbConfig_ApiAuthDynamic,
}

impl VertexAiRagCorpus_VectorDbConfig_ApiAuth {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfig>>,
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
}

impl terrars::ToListMappable for VertexAiRagCorpus_VectorDbConfig_ApiAuth {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_ApiAuth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_VectorDbConfig_ApiAuth {}

impl BuildVertexAiRagCorpus_VectorDbConfig_ApiAuth {
    pub fn build(self) -> VertexAiRagCorpus_VectorDbConfig_ApiAuth {
        VertexAiRagCorpus_VectorDbConfig_ApiAuth {
            api_key_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_VectorDbConfig_ApiAuthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_VectorDbConfig_ApiAuthRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiRagCorpus_VectorDbConfig_ApiAuthRef {
        VertexAiRagCorpus_VectorDbConfig_ApiAuthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_VectorDbConfig_ApiAuthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(&self) -> terrars::ListRef<VertexAiRagCorpus_VectorDbConfig_ApiAuth_ApiKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_config", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_VectorDbConfig_Pinecone {
    index_name: terrars::PrimField<std::string::String>,
}

impl VertexAiRagCorpus_VectorDbConfig_Pinecone { }

impl terrars::ToListMappable for VertexAiRagCorpus_VectorDbConfig_Pinecone {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_Pinecone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_VectorDbConfig_Pinecone {
    #[doc = "Pinecone index name. This value cannot be changed after it's set."]
    pub index_name: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiRagCorpus_VectorDbConfig_Pinecone {
    pub fn build(self) -> VertexAiRagCorpus_VectorDbConfig_Pinecone {
        VertexAiRagCorpus_VectorDbConfig_Pinecone { index_name: self.index_name }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_VectorDbConfig_PineconeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_VectorDbConfig_PineconeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiRagCorpus_VectorDbConfig_PineconeRef {
        VertexAiRagCorpus_VectorDbConfig_PineconeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_VectorDbConfig_PineconeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `index_name` after provisioning.\nPinecone index name. This value cannot be changed after it's set."]
    pub fn index_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_name", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpoint {
    endpoint: terrars::PrimField<std::string::String>,
}

impl VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpoint { }

impl terrars::ToListMappable for VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpoint {
    type O =
        terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpoint {
    #[doc =
        "Required. The endpoint resource name. Format:\nprojects/{project}/locations/{location}/publishers/{publisher}/models/{model}\nor projects/{project}/locations/{location}/endpoints/{endpoint}."]
    pub endpoint: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpoint {
    pub fn build(self) -> VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpoint {
        VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpoint { endpoint: self.endpoint }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpointRef {
        VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nRequired. The endpoint resource name. Format:\nprojects/{project}/locations/{location}/publishers/{publisher}/models/{model}\nor projects/{project}/locations/{location}/endpoints/{endpoint}."]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model` after provisioning.\nOutput only. The resource name of the model that is deployed on the endpoint."]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc =
        "Get a reference to the value of field `model_version_id` after provisioning.\nOutput only. Version ID of the model that is deployed on the endpoint."]
    pub fn model_version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_version_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfigDynamic {
    vertex_prediction_endpoint: std
    ::option
    ::Option<
        terrars::DynamicBlock<VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpoint>,
    >,
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vertex_prediction_endpoint: std
    ::option
    ::Option<std::vec::Vec<VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpoint>>,
    dynamic: VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfigDynamic,
}

impl VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig {
    #[doc = "Set the field `vertex_prediction_endpoint`.\n"]
    pub fn set_vertex_prediction_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vertex_prediction_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vertex_prediction_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig {}

impl BuildVertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig {
    pub fn build(self) -> VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig {
        VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig {
            vertex_prediction_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfigRef {
        VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vertex_prediction_endpoint` after provisioning.\n"]
    pub fn vertex_prediction_endpoint(
        &self,
    ) -> terrars::ListRef<VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig_VertexPredictionEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vertex_prediction_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Ann {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    leaf_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tree_depth: std::option::Option<terrars::PrimField<f64>>,
}

impl VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Ann {
    #[doc = "Set the field `leaf_count`.\nNumber of leaf nodes in the tree-based structure. Default value is 500."]
    pub fn set_leaf_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.leaf_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tree_depth`.\nThe depth of the tree-based structure. Only depth values of 2 and 3 are supported. Default value is 2."]
    pub fn set_tree_depth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tree_depth = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Ann {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Ann>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_VectorDbConfig_RagManagedDb_Ann {}

impl BuildVertexAiRagCorpus_VectorDbConfig_RagManagedDb_Ann {
    pub fn build(self) -> VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Ann {
        VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Ann {
            leaf_count: core::default::Default::default(),
            tree_depth: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_VectorDbConfig_RagManagedDb_AnnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_VectorDbConfig_RagManagedDb_AnnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiRagCorpus_VectorDbConfig_RagManagedDb_AnnRef {
        VertexAiRagCorpus_VectorDbConfig_RagManagedDb_AnnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_VectorDbConfig_RagManagedDb_AnnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `leaf_count` after provisioning.\nNumber of leaf nodes in the tree-based structure. Default value is 500."]
    pub fn leaf_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.leaf_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tree_depth` after provisioning.\nThe depth of the tree-based structure. Only depth values of 2 and 3 are supported. Default value is 2."]
    pub fn tree_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tree_depth", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Knn {}

impl VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Knn { }

impl terrars::ToListMappable for VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Knn {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Knn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_VectorDbConfig_RagManagedDb_Knn {}

impl BuildVertexAiRagCorpus_VectorDbConfig_RagManagedDb_Knn {
    pub fn build(self) -> VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Knn {
        VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Knn {}
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_VectorDbConfig_RagManagedDb_KnnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_VectorDbConfig_RagManagedDb_KnnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiRagCorpus_VectorDbConfig_RagManagedDb_KnnRef {
        VertexAiRagCorpus_VectorDbConfig_RagManagedDb_KnnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_VectorDbConfig_RagManagedDb_KnnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct VertexAiRagCorpus_VectorDbConfig_RagManagedDbDynamic {
    ann: std::option::Option<terrars::DynamicBlock<VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Ann>>,
    knn: std::option::Option<terrars::DynamicBlock<VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Knn>>,
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_VectorDbConfig_RagManagedDb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ann: std::option::Option<std::vec::Vec<VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Ann>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    knn: std::option::Option<std::vec::Vec<VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Knn>>,
    dynamic: VertexAiRagCorpus_VectorDbConfig_RagManagedDbDynamic,
}

impl VertexAiRagCorpus_VectorDbConfig_RagManagedDb {
    #[doc = "Set the field `ann`.\n"]
    pub fn set_ann(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Ann>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ann = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ann = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `knn`.\n"]
    pub fn set_knn(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_RagManagedDb_Knn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.knn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.knn = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiRagCorpus_VectorDbConfig_RagManagedDb {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_RagManagedDb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_VectorDbConfig_RagManagedDb {}

impl BuildVertexAiRagCorpus_VectorDbConfig_RagManagedDb {
    pub fn build(self) -> VertexAiRagCorpus_VectorDbConfig_RagManagedDb {
        VertexAiRagCorpus_VectorDbConfig_RagManagedDb {
            ann: core::default::Default::default(),
            knn: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_VectorDbConfig_RagManagedDbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_VectorDbConfig_RagManagedDbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiRagCorpus_VectorDbConfig_RagManagedDbRef {
        VertexAiRagCorpus_VectorDbConfig_RagManagedDbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_VectorDbConfig_RagManagedDbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ann` after provisioning.\n"]
    pub fn ann(&self) -> terrars::ListRef<VertexAiRagCorpus_VectorDbConfig_RagManagedDb_AnnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ann", self.base))
    }

    #[doc = "Get a reference to the value of field `knn` after provisioning.\n"]
    pub fn knn(&self) -> terrars::ListRef<VertexAiRagCorpus_VectorDbConfig_RagManagedDb_KnnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.knn", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_VectorDbConfig_VertexVectorSearch {
    index: terrars::PrimField<std::string::String>,
    index_endpoint: terrars::PrimField<std::string::String>,
}

impl VertexAiRagCorpus_VectorDbConfig_VertexVectorSearch { }

impl terrars::ToListMappable for VertexAiRagCorpus_VectorDbConfig_VertexVectorSearch {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_VertexVectorSearch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_VectorDbConfig_VertexVectorSearch {
    #[doc = "The resource name of the Index.\nFormat: projects/{project}/locations/{location}/indexes/{index}"]
    pub index: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource name of the Index Endpoint.\nFormat: projects/{project}/locations/{location}/indexEndpoints/{index_endpoint}"]
    pub index_endpoint: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiRagCorpus_VectorDbConfig_VertexVectorSearch {
    pub fn build(self) -> VertexAiRagCorpus_VectorDbConfig_VertexVectorSearch {
        VertexAiRagCorpus_VectorDbConfig_VertexVectorSearch {
            index: self.index,
            index_endpoint: self.index_endpoint,
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_VectorDbConfig_VertexVectorSearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_VectorDbConfig_VertexVectorSearchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VertexAiRagCorpus_VectorDbConfig_VertexVectorSearchRef {
        VertexAiRagCorpus_VectorDbConfig_VertexVectorSearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_VectorDbConfig_VertexVectorSearchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `index` after provisioning.\nThe resource name of the Index.\nFormat: projects/{project}/locations/{location}/indexes/{index}"]
    pub fn index(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index", self.base))
    }

    #[doc =
        "Get a reference to the value of field `index_endpoint` after provisioning.\nThe resource name of the Index Endpoint.\nFormat: projects/{project}/locations/{location}/indexEndpoints/{index_endpoint}"]
    pub fn index_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct VertexAiRagCorpus_VectorDbConfigDynamic {
    api_auth: std::option::Option<terrars::DynamicBlock<VertexAiRagCorpus_VectorDbConfig_ApiAuth>>,
    pinecone: std::option::Option<terrars::DynamicBlock<VertexAiRagCorpus_VectorDbConfig_Pinecone>>,
    rag_embedding_model_config: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig>>,
    rag_managed_db: std::option::Option<terrars::DynamicBlock<VertexAiRagCorpus_VectorDbConfig_RagManagedDb>>,
    vertex_vector_search: std
    ::option
    ::Option<terrars::DynamicBlock<VertexAiRagCorpus_VectorDbConfig_VertexVectorSearch>>,
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_VectorDbConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_auth: std::option::Option<std::vec::Vec<VertexAiRagCorpus_VectorDbConfig_ApiAuth>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pinecone: std::option::Option<std::vec::Vec<VertexAiRagCorpus_VectorDbConfig_Pinecone>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rag_embedding_model_config: std
    ::option
    ::Option<std::vec::Vec<VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rag_managed_db: std::option::Option<std::vec::Vec<VertexAiRagCorpus_VectorDbConfig_RagManagedDb>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vertex_vector_search: std::option::Option<std::vec::Vec<VertexAiRagCorpus_VectorDbConfig_VertexVectorSearch>>,
    dynamic: VertexAiRagCorpus_VectorDbConfigDynamic,
}

impl VertexAiRagCorpus_VectorDbConfig {
    #[doc = "Set the field `api_auth`.\n"]
    pub fn set_api_auth(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_ApiAuth>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_auth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_auth = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pinecone`.\n"]
    pub fn set_pinecone(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_Pinecone>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pinecone = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pinecone = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rag_embedding_model_config`.\n"]
    pub fn set_rag_embedding_model_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rag_embedding_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rag_embedding_model_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rag_managed_db`.\n"]
    pub fn set_rag_managed_db(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_RagManagedDb>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rag_managed_db = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rag_managed_db = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vertex_vector_search`.\n"]
    pub fn set_vertex_vector_search(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig_VertexVectorSearch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vertex_vector_search = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vertex_vector_search = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VertexAiRagCorpus_VectorDbConfig {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_VectorDbConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_VectorDbConfig {}

impl BuildVertexAiRagCorpus_VectorDbConfig {
    pub fn build(self) -> VertexAiRagCorpus_VectorDbConfig {
        VertexAiRagCorpus_VectorDbConfig {
            api_auth: core::default::Default::default(),
            pinecone: core::default::Default::default(),
            rag_embedding_model_config: core::default::Default::default(),
            rag_managed_db: core::default::Default::default(),
            vertex_vector_search: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_VectorDbConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_VectorDbConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiRagCorpus_VectorDbConfigRef {
        VertexAiRagCorpus_VectorDbConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_VectorDbConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_auth` after provisioning.\n"]
    pub fn api_auth(&self) -> terrars::ListRef<VertexAiRagCorpus_VectorDbConfig_ApiAuthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_auth", self.base))
    }

    #[doc = "Get a reference to the value of field `pinecone` after provisioning.\n"]
    pub fn pinecone(&self) -> terrars::ListRef<VertexAiRagCorpus_VectorDbConfig_PineconeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pinecone", self.base))
    }

    #[doc = "Get a reference to the value of field `rag_embedding_model_config` after provisioning.\n"]
    pub fn rag_embedding_model_config(
        &self,
    ) -> terrars::ListRef<VertexAiRagCorpus_VectorDbConfig_RagEmbeddingModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rag_embedding_model_config", self.base))
    }

    #[doc = "Get a reference to the value of field `rag_managed_db` after provisioning.\n"]
    pub fn rag_managed_db(&self) -> terrars::ListRef<VertexAiRagCorpus_VectorDbConfig_RagManagedDbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rag_managed_db", self.base))
    }

    #[doc = "Get a reference to the value of field `vertex_vector_search` after provisioning.\n"]
    pub fn vertex_vector_search(&self) -> terrars::ListRef<VertexAiRagCorpus_VectorDbConfig_VertexVectorSearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vertex_vector_search", self.base))
    }
}

#[derive(Serialize)]
pub struct VertexAiRagCorpus_VertexAiSearchConfig {
    serving_config: terrars::PrimField<std::string::String>,
}

impl VertexAiRagCorpus_VertexAiSearchConfig { }

impl terrars::ToListMappable for VertexAiRagCorpus_VertexAiSearchConfig {
    type O = terrars::BlockAssignable<VertexAiRagCorpus_VertexAiSearchConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVertexAiRagCorpus_VertexAiSearchConfig {
    #[doc =
        "Vertex AI Search Serving Config resource full name. For example,\nprojects/{project}/locations/{location}/collections/{collection}/engines/{engine}/servingConfigs/{serving_config}\nor\nprojects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/servingConfigs/{serving_config}."]
    pub serving_config: terrars::PrimField<std::string::String>,
}

impl BuildVertexAiRagCorpus_VertexAiSearchConfig {
    pub fn build(self) -> VertexAiRagCorpus_VertexAiSearchConfig {
        VertexAiRagCorpus_VertexAiSearchConfig { serving_config: self.serving_config }
    }
}

#[allow(dead_code)]
pub struct VertexAiRagCorpus_VertexAiSearchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VertexAiRagCorpus_VertexAiSearchConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VertexAiRagCorpus_VertexAiSearchConfigRef {
        VertexAiRagCorpus_VertexAiSearchConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VertexAiRagCorpus_VertexAiSearchConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `serving_config` after provisioning.\nVertex AI Search Serving Config resource full name. For example,\nprojects/{project}/locations/{location}/collections/{collection}/engines/{engine}/servingConfigs/{serving_config}\nor\nprojects/{project}/locations/{location}/collections/{collection}/dataStores/{data_store}/servingConfigs/{serving_config}."]
    pub fn serving_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serving_config", self.base))
    }
}
