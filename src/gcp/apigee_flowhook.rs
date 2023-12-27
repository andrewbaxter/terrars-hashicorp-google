use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeFlowhookData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    continue_on_error: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    environment: PrimField<String>,
    flow_hook_point: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    org_id: PrimField<String>,
    sharedflow: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeFlowhookTimeoutsEl>,
}

struct ApigeeFlowhook_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeFlowhookData>,
}

#[derive(Clone)]
pub struct ApigeeFlowhook(Rc<ApigeeFlowhook_>);

impl ApigeeFlowhook {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `continue_on_error`.\nFlag that specifies whether execution should continue if the flow hook throws an exception. Set to true to continue execution. Set to false to stop execution if the flow hook throws an exception. Defaults to true."]
    pub fn set_continue_on_error(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().continue_on_error = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nDescription of the flow hook."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeFlowhookTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `continue_on_error` after provisioning.\nFlag that specifies whether execution should continue if the flow hook throws an exception. Set to true to continue execution. Set to false to stop execution if the flow hook throws an exception. Defaults to true."]
    pub fn continue_on_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.continue_on_error", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the flow hook."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nThe resource ID of the environment."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `flow_hook_point` after provisioning.\nWhere in the API call flow the flow hook is invoked. Must be one of PreProxyFlowHook, PostProxyFlowHook, PreTargetFlowHook, or PostTargetFlowHook."]
    pub fn flow_hook_point(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_hook_point", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the environment"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sharedflow` after provisioning.\nId of the Sharedflow attaching to a flowhook point."]
    pub fn sharedflow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sharedflow", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeFlowhookTimeoutsElRef {
        ApigeeFlowhookTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ApigeeFlowhook {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApigeeFlowhook { }

impl ToListMappable for ApigeeFlowhook {
    type O = ListRef<ApigeeFlowhookRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApigeeFlowhook_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_flowhook".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeFlowhook {
    pub tf_id: String,
    #[doc= "The resource ID of the environment."]
    pub environment: PrimField<String>,
    #[doc= "Where in the API call flow the flow hook is invoked. Must be one of PreProxyFlowHook, PostProxyFlowHook, PreTargetFlowHook, or PostTargetFlowHook."]
    pub flow_hook_point: PrimField<String>,
    #[doc= "The Apigee Organization associated with the environment"]
    pub org_id: PrimField<String>,
    #[doc= "Id of the Sharedflow attaching to a flowhook point."]
    pub sharedflow: PrimField<String>,
}

impl BuildApigeeFlowhook {
    pub fn build(self, stack: &mut Stack) -> ApigeeFlowhook {
        let out = ApigeeFlowhook(Rc::new(ApigeeFlowhook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeFlowhookData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                continue_on_error: core::default::Default::default(),
                description: core::default::Default::default(),
                environment: self.environment,
                flow_hook_point: self.flow_hook_point,
                id: core::default::Default::default(),
                org_id: self.org_id,
                sharedflow: self.sharedflow,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeFlowhookRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeFlowhookRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeFlowhookRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `continue_on_error` after provisioning.\nFlag that specifies whether execution should continue if the flow hook throws an exception. Set to true to continue execution. Set to false to stop execution if the flow hook throws an exception. Defaults to true."]
    pub fn continue_on_error(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.continue_on_error", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the flow hook."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `environment` after provisioning.\nThe resource ID of the environment."]
    pub fn environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `flow_hook_point` after provisioning.\nWhere in the API call flow the flow hook is invoked. Must be one of PreProxyFlowHook, PostProxyFlowHook, PreTargetFlowHook, or PostTargetFlowHook."]
    pub fn flow_hook_point(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.flow_hook_point", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the environment"]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sharedflow` after provisioning.\nId of the Sharedflow attaching to a flowhook point."]
    pub fn sharedflow(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.sharedflow", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeFlowhookTimeoutsElRef {
        ApigeeFlowhookTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApigeeFlowhookTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
}

impl ApigeeFlowhookTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }

    #[doc= "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
}

impl ToListMappable for ApigeeFlowhookTimeoutsEl {
    type O = BlockAssignable<ApigeeFlowhookTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeFlowhookTimeoutsEl {}

impl BuildApigeeFlowhookTimeoutsEl {
    pub fn build(self) -> ApigeeFlowhookTimeoutsEl {
        ApigeeFlowhookTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct ApigeeFlowhookTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeFlowhookTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeFlowhookTimeoutsElRef {
        ApigeeFlowhookTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeFlowhookTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc= "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
