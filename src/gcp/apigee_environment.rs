use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApigeeEnvironmentData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    api_proxy_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployment_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    org_id: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_config: Option<Vec<ApigeeEnvironmentNodeConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ApigeeEnvironmentTimeoutsEl>,
    dynamic: ApigeeEnvironmentDynamic,
}

struct ApigeeEnvironment_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApigeeEnvironmentData>,
}

#[derive(Clone)]
pub struct ApigeeEnvironment(Rc<ApigeeEnvironment_>);

impl ApigeeEnvironment {
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

    #[doc= "Set the field `api_proxy_type`.\nOptional. API Proxy type supported by the environment. The type can be set when creating\nthe Environment and cannot be changed. Possible values: [\"API_PROXY_TYPE_UNSPECIFIED\", \"PROGRAMMABLE\", \"CONFIGURABLE\"]"]
    pub fn set_api_proxy_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().api_proxy_type = Some(v.into());
        self
    }

    #[doc= "Set the field `deployment_type`.\nOptional. Deployment type supported by the environment. The deployment type can be\nset when creating the environment and cannot be changed. When you enable archive\ndeployment, you will be prevented from performing a subset of actions within the\nenvironment, including:\nManaging the deployment of API proxy or shared flow revisions;\nCreating, updating, or deleting resource files;\nCreating, updating, or deleting target servers. Possible values: [\"DEPLOYMENT_TYPE_UNSPECIFIED\", \"PROXY\", \"ARCHIVE\"]"]
    pub fn set_deployment_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deployment_type = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\nDescription of the environment."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `display_name`.\nDisplay name of the environment."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\nTypes that can be selected for an Environment. Each of the types are\nlimited by capability and capacity. Refer to Apigee's public documentation\nto understand about each of these types in details.\nAn Apigee org can support heterogeneous Environments. Possible values: [\"ENVIRONMENT_TYPE_UNSPECIFIED\", \"BASE\", \"INTERMEDIATE\", \"COMPREHENSIVE\"]"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `node_config`.\n"]
    pub fn set_node_config(self, v: impl Into<BlockAssignable<ApigeeEnvironmentNodeConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().node_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.node_config = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ApigeeEnvironmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `api_proxy_type` after provisioning.\nOptional. API Proxy type supported by the environment. The type can be set when creating\nthe Environment and cannot be changed. Possible values: [\"API_PROXY_TYPE_UNSPECIFIED\", \"PROGRAMMABLE\", \"CONFIGURABLE\"]"]
    pub fn api_proxy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_proxy_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_type` after provisioning.\nOptional. Deployment type supported by the environment. The deployment type can be\nset when creating the environment and cannot be changed. When you enable archive\ndeployment, you will be prevented from performing a subset of actions within the\nenvironment, including:\nManaging the deployment of API proxy or shared flow revisions;\nCreating, updating, or deleting resource files;\nCreating, updating, or deleting target servers. Possible values: [\"DEPLOYMENT_TYPE_UNSPECIFIED\", \"PROXY\", \"ARCHIVE\"]"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the environment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the environment."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource ID of the environment."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee environment,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nTypes that can be selected for an Environment. Each of the types are\nlimited by capability and capacity. Refer to Apigee's public documentation\nto understand about each of these types in details.\nAn Apigee org can support heterogeneous Environments. Possible values: [\"ENVIRONMENT_TYPE_UNSPECIFIED\", \"BASE\", \"INTERMEDIATE\", \"COMPREHENSIVE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<ApigeeEnvironmentNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeEnvironmentTimeoutsElRef {
        ApigeeEnvironmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for ApigeeEnvironment {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for ApigeeEnvironment { }

impl ToListMappable for ApigeeEnvironment {
    type O = ListRef<ApigeeEnvironmentRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApigeeEnvironment_ {
    fn extract_resource_type(&self) -> String {
        "google_apigee_environment".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigeeEnvironment {
    pub tf_id: String,
    #[doc= "The resource ID of the environment."]
    pub name: PrimField<String>,
    #[doc= "The Apigee Organization associated with the Apigee environment,\nin the format 'organizations/{{org_name}}'."]
    pub org_id: PrimField<String>,
}

impl BuildApigeeEnvironment {
    pub fn build(self, stack: &mut Stack) -> ApigeeEnvironment {
        let out = ApigeeEnvironment(Rc::new(ApigeeEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApigeeEnvironmentData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_proxy_type: core::default::Default::default(),
                deployment_type: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                org_id: self.org_id,
                type_: core::default::Default::default(),
                node_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApigeeEnvironmentRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeEnvironmentRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApigeeEnvironmentRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_proxy_type` after provisioning.\nOptional. API Proxy type supported by the environment. The type can be set when creating\nthe Environment and cannot be changed. Possible values: [\"API_PROXY_TYPE_UNSPECIFIED\", \"PROGRAMMABLE\", \"CONFIGURABLE\"]"]
    pub fn api_proxy_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_proxy_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deployment_type` after provisioning.\nOptional. Deployment type supported by the environment. The deployment type can be\nset when creating the environment and cannot be changed. When you enable archive\ndeployment, you will be prevented from performing a subset of actions within the\nenvironment, including:\nManaging the deployment of API proxy or shared flow revisions;\nCreating, updating, or deleting resource files;\nCreating, updating, or deleting target servers. Possible values: [\"DEPLOYMENT_TYPE_UNSPECIFIED\", \"PROXY\", \"ARCHIVE\"]"]
    pub fn deployment_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\nDescription of the environment."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `display_name` after provisioning.\nDisplay name of the environment."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe resource ID of the environment."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `org_id` after provisioning.\nThe Apigee Organization associated with the Apigee environment,\nin the format 'organizations/{{org_name}}'."]
    pub fn org_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.org_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\nTypes that can be selected for an Environment. Each of the types are\nlimited by capability and capacity. Refer to Apigee's public documentation\nto understand about each of these types in details.\nAn Apigee org can support heterogeneous Environments. Possible values: [\"ENVIRONMENT_TYPE_UNSPECIFIED\", \"BASE\", \"INTERMEDIATE\", \"COMPREHENSIVE\"]"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `node_config` after provisioning.\n"]
    pub fn node_config(&self) -> ListRef<ApigeeEnvironmentNodeConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.node_config", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApigeeEnvironmentTimeoutsElRef {
        ApigeeEnvironmentTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ApigeeEnvironmentNodeConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    max_node_count: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_node_count: Option<PrimField<String>>,
}

impl ApigeeEnvironmentNodeConfigEl {
    #[doc= "Set the field `max_node_count`.\nThe maximum total number of gateway nodes that the is reserved for all instances that\nhas the specified environment. If not specified, the default is determined by the\nrecommended maximum number of nodes for that gateway."]
    pub fn set_max_node_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.max_node_count = Some(v.into());
        self
    }

    #[doc= "Set the field `min_node_count`.\nThe minimum total number of gateway nodes that the is reserved for all instances that\nhas the specified environment. If not specified, the default is determined by the\nrecommended minimum number of nodes for that gateway."]
    pub fn set_min_node_count(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.min_node_count = Some(v.into());
        self
    }
}

impl ToListMappable for ApigeeEnvironmentNodeConfigEl {
    type O = BlockAssignable<ApigeeEnvironmentNodeConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeEnvironmentNodeConfigEl {}

impl BuildApigeeEnvironmentNodeConfigEl {
    pub fn build(self) -> ApigeeEnvironmentNodeConfigEl {
        ApigeeEnvironmentNodeConfigEl {
            max_node_count: core::default::Default::default(),
            min_node_count: core::default::Default::default(),
        }
    }
}

pub struct ApigeeEnvironmentNodeConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeEnvironmentNodeConfigElRef {
    fn new(shared: StackShared, base: String) -> ApigeeEnvironmentNodeConfigElRef {
        ApigeeEnvironmentNodeConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeEnvironmentNodeConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `current_aggregate_node_count` after provisioning.\nThe current total number of gateway nodes that each environment currently has across\nall instances."]
    pub fn current_aggregate_node_count(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.current_aggregate_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `max_node_count` after provisioning.\nThe maximum total number of gateway nodes that the is reserved for all instances that\nhas the specified environment. If not specified, the default is determined by the\nrecommended maximum number of nodes for that gateway."]
    pub fn max_node_count(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_node_count", self.base))
    }

    #[doc= "Get a reference to the value of field `min_node_count` after provisioning.\nThe minimum total number of gateway nodes that the is reserved for all instances that\nhas the specified environment. If not specified, the default is determined by the\nrecommended minimum number of nodes for that gateway."]
    pub fn min_node_count(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.min_node_count", self.base))
    }
}

#[derive(Serialize)]
pub struct ApigeeEnvironmentTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}

impl ApigeeEnvironmentTimeoutsEl {
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

    #[doc= "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}

impl ToListMappable for ApigeeEnvironmentTimeoutsEl {
    type O = BlockAssignable<ApigeeEnvironmentTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigeeEnvironmentTimeoutsEl {}

impl BuildApigeeEnvironmentTimeoutsEl {
    pub fn build(self) -> ApigeeEnvironmentTimeoutsEl {
        ApigeeEnvironmentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct ApigeeEnvironmentTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApigeeEnvironmentTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ApigeeEnvironmentTimeoutsElRef {
        ApigeeEnvironmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApigeeEnvironmentTimeoutsElRef {
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

    #[doc= "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApigeeEnvironmentDynamic {
    node_config: Option<DynamicBlock<ApigeeEnvironmentNodeConfigEl>>,
}
