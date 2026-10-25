#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CesExampleData_ {
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
    app: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    example_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<std::vec::Vec<CesExample_Messages>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CesExample_Timeouts>,
    dynamic: CesExampleDynamic,
}

struct CesExample_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CesExampleData_>,
}

#[derive(Clone)]
pub struct CesExample(std::rc::Rc<CesExample_>);

impl CesExample {
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

    #[doc = "Set the field `description`.\nHuman-readable description of the example."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `entry_agent`.\nThe agent that initially handles the conversation. If not specified, the\nexample represents a conversation that is handled by the root agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn set_entry_agent(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().entry_agent = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(self, v: impl std::convert::Into<terrars::BlockAssignable<CesExample_Messages>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().messages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.messages = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CesExample_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name', defining the app the example belongs to. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the example was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the example."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the example."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_agent` after provisioning.\nThe agent that initially handles the conversation. If not specified, the\nexample represents a conversation that is handled by the root agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn entry_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `example_id` after provisioning.\nThe ID to use for the example, which will become the final component of\nthe example's resource name. In Terraform, this field is required."]
    pub fn example_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.example_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invalid` after provisioning.\nThe example may become invalid if referencing resources are deleted.\nInvalid examples will not be used as few-shot examples."]
    pub fn invalid(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invalid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name', defining what region the parent app is in. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the example.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/examples/{example}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the example was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::ListRef<CesExample_MessagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesExample_TimeoutsRef {
        CesExample_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CesExample {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CesExample { }

impl terrars::ToListMappable for CesExample {
    type O = terrars::ListRef<CesExampleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CesExample_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_ces_example".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCesExample {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name', defining the app the example belongs to. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: terrars::PrimField<std::string::String>,
    #[doc = "Display name of the example."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the example, which will become the final component of\nthe example's resource name. In Terraform, this field is required."]
    pub example_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name', defining what region the parent app is in. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildCesExample {
    pub fn build(self, stack: &mut terrars::Stack) -> CesExample {
        let out = CesExample(std::rc::Rc::new(CesExample_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CesExampleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app: self.app,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                entry_agent: core::default::Default::default(),
                example_id: self.example_id,
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                messages: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CesExampleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesExampleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CesExampleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name', defining the app the example belongs to. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the example was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nHuman-readable description of the example."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the example."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `entry_agent` after provisioning.\nThe agent that initially handles the conversation. If not specified, the\nexample represents a conversation that is handled by the root agent.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn entry_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_agent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `example_id` after provisioning.\nThe ID to use for the example, which will become the final component of\nthe example's resource name. In Terraform, this field is required."]
    pub fn example_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.example_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `invalid` after provisioning.\nThe example may become invalid if referencing resources are deleted.\nInvalid examples will not be used as few-shot examples."]
    pub fn invalid(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invalid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name', defining what region the parent app is in. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the example.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/examples/{example}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTimestamp when the example was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::ListRef<CesExample_MessagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesExample_TimeoutsRef {
        CesExample_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CesExampleDynamic {
    messages: std::option::Option<terrars::DynamicBlock<CesExample_Messages>>,
}

#[derive(Serialize)]
pub struct CesExample_Messages_Chunks_AgentTransfer {
    target_agent: terrars::PrimField<std::string::String>,
}

impl CesExample_Messages_Chunks_AgentTransfer { }

impl terrars::ToListMappable for CesExample_Messages_Chunks_AgentTransfer {
    type O = terrars::BlockAssignable<CesExample_Messages_Chunks_AgentTransfer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesExample_Messages_Chunks_AgentTransfer {
    #[doc =
        "The agent to which the conversation is being transferred. The agent will\nhandle the conversation from this point forward.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub target_agent: terrars::PrimField<std::string::String>,
}

impl BuildCesExample_Messages_Chunks_AgentTransfer {
    pub fn build(self) -> CesExample_Messages_Chunks_AgentTransfer {
        CesExample_Messages_Chunks_AgentTransfer { target_agent: self.target_agent }
    }
}

#[allow(dead_code)]
pub struct CesExample_Messages_Chunks_AgentTransferRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesExample_Messages_Chunks_AgentTransferRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesExample_Messages_Chunks_AgentTransferRef {
        CesExample_Messages_Chunks_AgentTransferRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesExample_Messages_Chunks_AgentTransferRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the agent."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_agent` after provisioning.\nThe agent to which the conversation is being transferred. The agent will\nhandle the conversation from this point forward.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/agents/{agent}'"]
    pub fn target_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_agent", self.base))
    }
}

#[derive(Serialize)]
pub struct CesExample_Messages_Chunks_Blob {
    data: terrars::PrimField<std::string::String>,
    mime_type: terrars::PrimField<std::string::String>,
}

impl CesExample_Messages_Chunks_Blob { }

impl terrars::ToListMappable for CesExample_Messages_Chunks_Blob {
    type O = terrars::BlockAssignable<CesExample_Messages_Chunks_Blob>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesExample_Messages_Chunks_Blob {
    #[doc = "Raw bytes of the blob."]
    pub data: terrars::PrimField<std::string::String>,
    #[doc = "The IANA standard MIME type of the source data."]
    pub mime_type: terrars::PrimField<std::string::String>,
}

impl BuildCesExample_Messages_Chunks_Blob {
    pub fn build(self) -> CesExample_Messages_Chunks_Blob {
        CesExample_Messages_Chunks_Blob {
            data: self.data,
            mime_type: self.mime_type,
        }
    }
}

#[allow(dead_code)]
pub struct CesExample_Messages_Chunks_BlobRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesExample_Messages_Chunks_BlobRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesExample_Messages_Chunks_BlobRef {
        CesExample_Messages_Chunks_BlobRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesExample_Messages_Chunks_BlobRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\nRaw bytes of the blob."]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mime_type` after provisioning.\nThe IANA standard MIME type of the source data."]
    pub fn mime_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }
}

#[derive(Serialize)]
pub struct CesExample_Messages_Chunks_Image {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alt_text: std::option::Option<terrars::PrimField<std::string::String>>,
    data: terrars::PrimField<std::string::String>,
    mime_type: terrars::PrimField<std::string::String>,
}

impl CesExample_Messages_Chunks_Image {
    #[doc = "Set the field `alt_text`.\nThe alternative text for the image."]
    pub fn set_alt_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.alt_text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesExample_Messages_Chunks_Image {
    type O = terrars::BlockAssignable<CesExample_Messages_Chunks_Image>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesExample_Messages_Chunks_Image {
    #[doc = "Raw bytes of the image."]
    pub data: terrars::PrimField<std::string::String>,
    #[doc =
        "The IANA standard MIME type of the source data.\nSupported image types includes:\n* image/png\n* image/jpeg\n* image/webp"]
    pub mime_type: terrars::PrimField<std::string::String>,
}

impl BuildCesExample_Messages_Chunks_Image {
    pub fn build(self) -> CesExample_Messages_Chunks_Image {
        CesExample_Messages_Chunks_Image {
            alt_text: core::default::Default::default(),
            data: self.data,
            mime_type: self.mime_type,
        }
    }
}

#[allow(dead_code)]
pub struct CesExample_Messages_Chunks_ImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesExample_Messages_Chunks_ImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesExample_Messages_Chunks_ImageRef {
        CesExample_Messages_Chunks_ImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesExample_Messages_Chunks_ImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alt_text` after provisioning.\nThe alternative text for the image."]
    pub fn alt_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alt_text", self.base))
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\nRaw bytes of the image."]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mime_type` after provisioning.\nThe IANA standard MIME type of the source data.\nSupported image types includes:\n* image/png\n* image/jpeg\n* image/webp"]
    pub fn mime_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }
}

#[derive(Serialize)]
pub struct CesExample_Messages_Chunks_ToolCall_ToolsetTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    toolset: terrars::PrimField<std::string::String>,
}

impl CesExample_Messages_Chunks_ToolCall_ToolsetTool {
    #[doc = "Set the field `tool_id`.\nThe tool ID to filter the tools to retrieve the schema for."]
    pub fn set_tool_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tool_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesExample_Messages_Chunks_ToolCall_ToolsetTool {
    type O = terrars::BlockAssignable<CesExample_Messages_Chunks_ToolCall_ToolsetTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesExample_Messages_Chunks_ToolCall_ToolsetTool {
    #[doc =
        "The resource name of the Toolset from which this tool is derived.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub toolset: terrars::PrimField<std::string::String>,
}

impl BuildCesExample_Messages_Chunks_ToolCall_ToolsetTool {
    pub fn build(self) -> CesExample_Messages_Chunks_ToolCall_ToolsetTool {
        CesExample_Messages_Chunks_ToolCall_ToolsetTool {
            tool_id: core::default::Default::default(),
            toolset: self.toolset,
        }
    }
}

#[allow(dead_code)]
pub struct CesExample_Messages_Chunks_ToolCall_ToolsetToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesExample_Messages_Chunks_ToolCall_ToolsetToolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesExample_Messages_Chunks_ToolCall_ToolsetToolRef {
        CesExample_Messages_Chunks_ToolCall_ToolsetToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesExample_Messages_Chunks_ToolCall_ToolsetToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tool_id` after provisioning.\nThe tool ID to filter the tools to retrieve the schema for."]
    pub fn tool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `toolset` after provisioning.\nThe resource name of the Toolset from which this tool is derived.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/toolsets/{toolset}'"]
    pub fn toolset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.toolset", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesExample_Messages_Chunks_ToolCallDynamic {
    toolset_tool: std::option::Option<terrars::DynamicBlock<CesExample_Messages_Chunks_ToolCall_ToolsetTool>>,
}

#[derive(Serialize)]
pub struct CesExample_Messages_Chunks_ToolCall {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    toolset_tool: std::option::Option<std::vec::Vec<CesExample_Messages_Chunks_ToolCall_ToolsetTool>>,
    dynamic: CesExample_Messages_Chunks_ToolCallDynamic,
}

impl CesExample_Messages_Chunks_ToolCall {
    #[doc = "Set the field `args`.\nThe input parameters and values for the tool in JSON object format."]
    pub fn set_args(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `id`.\nThe unique identifier of the tool call. If populated, the client should\nreturn the execution result with the matching ID in\nToolResponse."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tool`.\nThe name of the tool to execute.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn set_tool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `toolset_tool`.\n"]
    pub fn set_toolset_tool(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesExample_Messages_Chunks_ToolCall_ToolsetTool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.toolset_tool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.toolset_tool = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesExample_Messages_Chunks_ToolCall {
    type O = terrars::BlockAssignable<CesExample_Messages_Chunks_ToolCall>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesExample_Messages_Chunks_ToolCall {}

impl BuildCesExample_Messages_Chunks_ToolCall {
    pub fn build(self) -> CesExample_Messages_Chunks_ToolCall {
        CesExample_Messages_Chunks_ToolCall {
            args: core::default::Default::default(),
            id: core::default::Default::default(),
            tool: core::default::Default::default(),
            toolset_tool: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesExample_Messages_Chunks_ToolCallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesExample_Messages_Chunks_ToolCallRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesExample_Messages_Chunks_ToolCallRef {
        CesExample_Messages_Chunks_ToolCallRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesExample_Messages_Chunks_ToolCallRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nThe input parameters and values for the tool in JSON object format."]
    pub fn args(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the tool."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe unique identifier of the tool call. If populated, the client should\nreturn the execution result with the matching ID in\nToolResponse."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tool` after provisioning.\nThe name of the tool to execute.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn tool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool", self.base))
    }

    #[doc = "Get a reference to the value of field `toolset_tool` after provisioning.\n"]
    pub fn toolset_tool(&self) -> terrars::ListRef<CesExample_Messages_Chunks_ToolCall_ToolsetToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.toolset_tool", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesExample_Messages_Chunks_ToolResponseDynamic {
    toolset_tool: std::option::Option<terrars::DynamicBlock<CesExample_Messages_Chunks_ToolCall_ToolsetTool>>,
}

#[derive(Serialize)]
pub struct CesExample_Messages_Chunks_ToolResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    response: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    toolset_tool: std::option::Option<std::vec::Vec<CesExample_Messages_Chunks_ToolCall_ToolsetTool>>,
    dynamic: CesExample_Messages_Chunks_ToolResponseDynamic,
}

impl CesExample_Messages_Chunks_ToolResponse {
    #[doc = "Set the field `id`.\nThe matching ID of the tool call the response is for."]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tool`.\nThe name of the tool to execute.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn set_tool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `toolset_tool`.\n"]
    pub fn set_toolset_tool(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesExample_Messages_Chunks_ToolCall_ToolsetTool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.toolset_tool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.toolset_tool = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesExample_Messages_Chunks_ToolResponse {
    type O = terrars::BlockAssignable<CesExample_Messages_Chunks_ToolResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesExample_Messages_Chunks_ToolResponse {
    #[doc =
        "The tool execution result in JSON object format.\nUse \"output\" key to specify tool response and \"error\" key to specify\nerror details (if any). If \"output\" and \"error\" keys are not specified,\nthen whole \"response\" is treated as tool execution result."]
    pub response: terrars::PrimField<std::string::String>,
}

impl BuildCesExample_Messages_Chunks_ToolResponse {
    pub fn build(self) -> CesExample_Messages_Chunks_ToolResponse {
        CesExample_Messages_Chunks_ToolResponse {
            id: core::default::Default::default(),
            response: self.response,
            tool: core::default::Default::default(),
            toolset_tool: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesExample_Messages_Chunks_ToolResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesExample_Messages_Chunks_ToolResponseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesExample_Messages_Chunks_ToolResponseRef {
        CesExample_Messages_Chunks_ToolResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesExample_Messages_Chunks_ToolResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the tool."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `id` after provisioning.\nThe matching ID of the tool call the response is for."]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `response` after provisioning.\nThe tool execution result in JSON object format.\nUse \"output\" key to specify tool response and \"error\" key to specify\nerror details (if any). If \"output\" and \"error\" keys are not specified,\nthen whole \"response\" is treated as tool execution result."]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tool` after provisioning.\nThe name of the tool to execute.\nFormat: 'projects/{project}/locations/{location}/apps/{app}/tools/{tool}'"]
    pub fn tool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool", self.base))
    }

    #[doc = "Get a reference to the value of field `toolset_tool` after provisioning.\n"]
    pub fn toolset_tool(&self) -> terrars::ListRef<CesExample_Messages_Chunks_ToolCall_ToolsetToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.toolset_tool", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesExample_Messages_ChunksDynamic {
    agent_transfer: std::option::Option<terrars::DynamicBlock<CesExample_Messages_Chunks_AgentTransfer>>,
    blob: std::option::Option<terrars::DynamicBlock<CesExample_Messages_Chunks_Blob>>,
    image: std::option::Option<terrars::DynamicBlock<CesExample_Messages_Chunks_Image>>,
    tool_call: std::option::Option<terrars::DynamicBlock<CesExample_Messages_Chunks_ToolCall>>,
    tool_response: std::option::Option<terrars::DynamicBlock<CesExample_Messages_Chunks_ToolResponse>>,
}

#[derive(Serialize)]
pub struct CesExample_Messages_Chunks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_variables: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_transfer: std::option::Option<std::vec::Vec<CesExample_Messages_Chunks_AgentTransfer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blob: std::option::Option<std::vec::Vec<CesExample_Messages_Chunks_Blob>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<std::vec::Vec<CesExample_Messages_Chunks_Image>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_call: std::option::Option<std::vec::Vec<CesExample_Messages_Chunks_ToolCall>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_response: std::option::Option<std::vec::Vec<CesExample_Messages_Chunks_ToolResponse>>,
    dynamic: CesExample_Messages_ChunksDynamic,
}

impl CesExample_Messages_Chunks {
    #[doc = "Set the field `text`.\nText data."]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `updated_variables`.\nA struct represents variables that were updated in the conversation,\nkeyed by variable names."]
    pub fn set_updated_variables(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `agent_transfer`.\n"]
    pub fn set_agent_transfer(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesExample_Messages_Chunks_AgentTransfer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.agent_transfer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.agent_transfer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `blob`.\n"]
    pub fn set_blob(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesExample_Messages_Chunks_Blob>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.blob = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.blob = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesExample_Messages_Chunks_Image>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_call`.\n"]
    pub fn set_tool_call(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesExample_Messages_Chunks_ToolCall>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_call = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_call = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tool_response`.\n"]
    pub fn set_tool_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesExample_Messages_Chunks_ToolResponse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tool_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tool_response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesExample_Messages_Chunks {
    type O = terrars::BlockAssignable<CesExample_Messages_Chunks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesExample_Messages_Chunks {}

impl BuildCesExample_Messages_Chunks {
    pub fn build(self) -> CesExample_Messages_Chunks {
        CesExample_Messages_Chunks {
            text: core::default::Default::default(),
            updated_variables: core::default::Default::default(),
            agent_transfer: core::default::Default::default(),
            blob: core::default::Default::default(),
            image: core::default::Default::default(),
            tool_call: core::default::Default::default(),
            tool_response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesExample_Messages_ChunksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesExample_Messages_ChunksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesExample_Messages_ChunksRef {
        CesExample_Messages_ChunksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesExample_Messages_ChunksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\nText data."]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc =
        "Get a reference to the value of field `updated_variables` after provisioning.\nA struct represents variables that were updated in the conversation,\nkeyed by variable names."]
    pub fn updated_variables(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `agent_transfer` after provisioning.\n"]
    pub fn agent_transfer(&self) -> terrars::ListRef<CesExample_Messages_Chunks_AgentTransferRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_transfer", self.base))
    }

    #[doc = "Get a reference to the value of field `blob` after provisioning.\n"]
    pub fn blob(&self) -> terrars::ListRef<CesExample_Messages_Chunks_BlobRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blob", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<CesExample_Messages_Chunks_ImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_call` after provisioning.\n"]
    pub fn tool_call(&self) -> terrars::ListRef<CesExample_Messages_Chunks_ToolCallRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_call", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_response` after provisioning.\n"]
    pub fn tool_response(&self) -> terrars::ListRef<CesExample_Messages_Chunks_ToolResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_response", self.base))
    }
}

#[derive(Serialize, Default)]
struct CesExample_MessagesDynamic {
    chunks: std::option::Option<terrars::DynamicBlock<CesExample_Messages_Chunks>>,
}

#[derive(Serialize)]
pub struct CesExample_Messages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chunks: std::option::Option<std::vec::Vec<CesExample_Messages_Chunks>>,
    dynamic: CesExample_MessagesDynamic,
}

impl CesExample_Messages {
    #[doc = "Set the field `role`.\nThe role within the conversation, e.g., user, agent."]
    pub fn set_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `chunks`.\n"]
    pub fn set_chunks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CesExample_Messages_Chunks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.chunks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.chunks = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CesExample_Messages {
    type O = terrars::BlockAssignable<CesExample_Messages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesExample_Messages {}

impl BuildCesExample_Messages {
    pub fn build(self) -> CesExample_Messages {
        CesExample_Messages {
            role: core::default::Default::default(),
            chunks: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesExample_MessagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesExample_MessagesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesExample_MessagesRef {
        CesExample_MessagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesExample_MessagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `role` after provisioning.\nThe role within the conversation, e.g., user, agent."]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }

    #[doc = "Get a reference to the value of field `chunks` after provisioning.\n"]
    pub fn chunks(&self) -> terrars::ListRef<CesExample_Messages_ChunksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chunks", self.base))
    }
}

#[derive(Serialize)]
pub struct CesExample_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesExample_Timeouts {
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

impl terrars::ToListMappable for CesExample_Timeouts {
    type O = terrars::BlockAssignable<CesExample_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesExample_Timeouts {}

impl BuildCesExample_Timeouts {
    pub fn build(self) -> CesExample_Timeouts {
        CesExample_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesExample_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesExample_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesExample_TimeoutsRef {
        CesExample_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesExample_TimeoutsRef {
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
