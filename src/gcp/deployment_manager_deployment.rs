use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DeploymentManagerDeploymentData {
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
    create_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preview: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<std::vec::Vec<DeploymentManagerDeploymentLabelsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<DeploymentManagerDeploymentTargetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DeploymentManagerDeploymentTimeoutsEl>,
    dynamic: DeploymentManagerDeploymentDynamic,
}

struct DeploymentManagerDeployment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DeploymentManagerDeploymentData>,
}

#[derive(Clone)]
pub struct DeploymentManagerDeployment(std::rc::Rc<DeploymentManagerDeployment_>);

impl DeploymentManagerDeployment {
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
        "Set the field `create_policy`.\nSet the policy to use for creating new resources. Only used on\ncreate and update. Valid values are 'CREATE_OR_ACQUIRE' (default) or\n'ACQUIRE'. If set to 'ACQUIRE' and resources do not already exist,\nthe deployment will fail. Note that updating this field does not\nactually affect the deployment, just how it is updated. Default value: \"CREATE_OR_ACQUIRE\" Possible values: [\"ACQUIRE\", \"CREATE_OR_ACQUIRE\"]"]
    pub fn set_create_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().create_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete_policy`.\nSet the policy to use for deleting new resources on update/delete.\nValid values are 'DELETE' (default) or 'ABANDON'. If 'DELETE',\nresource is deleted after removal from Deployment Manager. If\n'ABANDON', the resource is only removed from Deployment Manager\nand is not actually deleted. Note that updating this field does not\nactually change the deployment, just how it is updated. Default value: \"DELETE\" Possible values: [\"ABANDON\", \"DELETE\"]"]
    pub fn set_delete_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().delete_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nOptional user-provided description of deployment."]
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
        "Set the field `preview`.\nIf set to true, a deployment is created with \"shell\" resources\nthat are not actually instantiated. This allows you to preview a\ndeployment. It can be updated to false to actually deploy\nwith real resources.\n ~>**NOTE:** Deployment Manager does not allow update\nof a deployment in preview (unless updating to preview=false). Thus,\nTerraform will force-recreate deployments if either preview is updated\nto true or if other fields are updated while preview is true."]
    pub fn set_preview(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().preview = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeploymentManagerDeploymentLabelsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().labels = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.labels = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeploymentManagerDeploymentTargetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DeploymentManagerDeploymentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_policy` after provisioning.\nSet the policy to use for creating new resources. Only used on\ncreate and update. Valid values are 'CREATE_OR_ACQUIRE' (default) or\n'ACQUIRE'. If set to 'ACQUIRE' and resources do not already exist,\nthe deployment will fail. Note that updating this field does not\nactually affect the deployment, just how it is updated. Default value: \"CREATE_OR_ACQUIRE\" Possible values: [\"ACQUIRE\", \"CREATE_OR_ACQUIRE\"]"]
    pub fn create_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_policy` after provisioning.\nSet the policy to use for deleting new resources on update/delete.\nValid values are 'DELETE' (default) or 'ABANDON'. If 'DELETE',\nresource is deleted after removal from Deployment Manager. If\n'ABANDON', the resource is only removed from Deployment Manager\nand is not actually deleted. Note that updating this field does not\nactually change the deployment, just how it is updated. Default value: \"DELETE\" Possible values: [\"ABANDON\", \"DELETE\"]"]
    pub fn delete_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_id` after provisioning.\nUnique identifier for deployment. Output only."]
    pub fn deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional user-provided description of deployment."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `manifest` after provisioning.\nOutput only. URL of the manifest representing the last manifest that\nwas successfully deployed."]
    pub fn manifest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manifest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the deployment"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preview` after provisioning.\nIf set to true, a deployment is created with \"shell\" resources\nthat are not actually instantiated. This allows you to preview a\ndeployment. It can be updated to false to actually deploy\nwith real resources.\n ~>**NOTE:** Deployment Manager does not allow update\nof a deployment in preview (unless updating to preview=false). Thus,\nTerraform will force-recreate deployments if either preview is updated\nto true or if other fields are updated while preview is true."]
    pub fn preview(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nOutput only. Server defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<DeploymentManagerDeploymentTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeploymentManagerDeploymentTimeoutsElRef {
        DeploymentManagerDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DeploymentManagerDeployment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DeploymentManagerDeployment { }

impl terrars::ToListMappable for DeploymentManagerDeployment {
    type O = terrars::ListRef<DeploymentManagerDeploymentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DeploymentManagerDeployment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_deployment_manager_deployment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDeploymentManagerDeployment {
    pub tf_id: std::string::String,
    #[doc = "Unique name for the deployment"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDeploymentManagerDeployment {
    pub fn build(self, stack: &mut terrars::Stack) -> DeploymentManagerDeployment {
        let out = DeploymentManagerDeployment(std::rc::Rc::new(DeploymentManagerDeployment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DeploymentManagerDeploymentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                create_policy: core::default::Default::default(),
                delete_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                preview: core::default::Default::default(),
                project: core::default::Default::default(),
                labels: core::default::Default::default(),
                target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DeploymentManagerDeploymentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeploymentManagerDeploymentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DeploymentManagerDeploymentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_policy` after provisioning.\nSet the policy to use for creating new resources. Only used on\ncreate and update. Valid values are 'CREATE_OR_ACQUIRE' (default) or\n'ACQUIRE'. If set to 'ACQUIRE' and resources do not already exist,\nthe deployment will fail. Note that updating this field does not\nactually affect the deployment, just how it is updated. Default value: \"CREATE_OR_ACQUIRE\" Possible values: [\"ACQUIRE\", \"CREATE_OR_ACQUIRE\"]"]
    pub fn create_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_policy` after provisioning.\nSet the policy to use for deleting new resources on update/delete.\nValid values are 'DELETE' (default) or 'ABANDON'. If 'DELETE',\nresource is deleted after removal from Deployment Manager. If\n'ABANDON', the resource is only removed from Deployment Manager\nand is not actually deleted. Note that updating this field does not\nactually change the deployment, just how it is updated. Default value: \"DELETE\" Possible values: [\"ABANDON\", \"DELETE\"]"]
    pub fn delete_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployment_id` after provisioning.\nUnique identifier for deployment. Output only."]
    pub fn deployment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nOptional user-provided description of deployment."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `manifest` after provisioning.\nOutput only. URL of the manifest representing the last manifest that\nwas successfully deployed."]
    pub fn manifest(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manifest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nUnique name for the deployment"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preview` after provisioning.\nIf set to true, a deployment is created with \"shell\" resources\nthat are not actually instantiated. This allows you to preview a\ndeployment. It can be updated to false to actually deploy\nwith real resources.\n ~>**NOTE:** Deployment Manager does not allow update\nof a deployment in preview (unless updating to preview=false). Thus,\nTerraform will force-recreate deployments if either preview is updated\nto true or if other fields are updated while preview is true."]
    pub fn preview(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preview", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nOutput only. Server defined URL for the resource."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<DeploymentManagerDeploymentTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DeploymentManagerDeploymentTimeoutsElRef {
        DeploymentManagerDeploymentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DeploymentManagerDeploymentLabelsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeploymentManagerDeploymentLabelsEl {
    #[doc = "Set the field `key`.\nKey for label."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nValue of label."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeploymentManagerDeploymentLabelsEl {
    type O = terrars::BlockAssignable<DeploymentManagerDeploymentLabelsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeploymentManagerDeploymentLabelsEl {}

impl BuildDeploymentManagerDeploymentLabelsEl {
    pub fn build(self) -> DeploymentManagerDeploymentLabelsEl {
        DeploymentManagerDeploymentLabelsEl {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DeploymentManagerDeploymentLabelsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeploymentManagerDeploymentLabelsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DeploymentManagerDeploymentLabelsElRef {
        DeploymentManagerDeploymentLabelsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeploymentManagerDeploymentLabelsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nKey for label."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of label."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DeploymentManagerDeploymentTargetElConfigEl {
    content: terrars::PrimField<std::string::String>,
}

impl DeploymentManagerDeploymentTargetElConfigEl { }

impl terrars::ToListMappable for DeploymentManagerDeploymentTargetElConfigEl {
    type O = terrars::BlockAssignable<DeploymentManagerDeploymentTargetElConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeploymentManagerDeploymentTargetElConfigEl {
    #[doc = "The full YAML contents of your configuration file."]
    pub content: terrars::PrimField<std::string::String>,
}

impl BuildDeploymentManagerDeploymentTargetElConfigEl {
    pub fn build(self) -> DeploymentManagerDeploymentTargetElConfigEl {
        DeploymentManagerDeploymentTargetElConfigEl { content: self.content }
    }
}

pub struct DeploymentManagerDeploymentTargetElConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeploymentManagerDeploymentTargetElConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeploymentManagerDeploymentTargetElConfigElRef {
        DeploymentManagerDeploymentTargetElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeploymentManagerDeploymentTargetElConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nThe full YAML contents of your configuration file."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }
}

#[derive(Serialize)]
pub struct DeploymentManagerDeploymentTargetElImportsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeploymentManagerDeploymentTargetElImportsEl {
    #[doc = "Set the field `content`.\nThe full contents of the template that you want to import."]
    pub fn set_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of the template to import, as declared in the YAML\nconfiguration."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DeploymentManagerDeploymentTargetElImportsEl {
    type O = terrars::BlockAssignable<DeploymentManagerDeploymentTargetElImportsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeploymentManagerDeploymentTargetElImportsEl {}

impl BuildDeploymentManagerDeploymentTargetElImportsEl {
    pub fn build(self) -> DeploymentManagerDeploymentTargetElImportsEl {
        DeploymentManagerDeploymentTargetElImportsEl {
            content: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DeploymentManagerDeploymentTargetElImportsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeploymentManagerDeploymentTargetElImportsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DeploymentManagerDeploymentTargetElImportsElRef {
        DeploymentManagerDeploymentTargetElImportsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeploymentManagerDeploymentTargetElImportsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content` after provisioning.\nThe full contents of the template that you want to import."]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the template to import, as declared in the YAML\nconfiguration."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct DeploymentManagerDeploymentTargetElDynamic {
    config: std::option::Option<terrars::DynamicBlock<DeploymentManagerDeploymentTargetElConfigEl>>,
    imports: std::option::Option<terrars::DynamicBlock<DeploymentManagerDeploymentTargetElImportsEl>>,
}

#[derive(Serialize)]
pub struct DeploymentManagerDeploymentTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<std::vec::Vec<DeploymentManagerDeploymentTargetElConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    imports: std::option::Option<std::vec::Vec<DeploymentManagerDeploymentTargetElImportsEl>>,
    dynamic: DeploymentManagerDeploymentTargetElDynamic,
}

impl DeploymentManagerDeploymentTargetEl {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeploymentManagerDeploymentTargetElConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `imports`.\n"]
    pub fn set_imports(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DeploymentManagerDeploymentTargetElImportsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.imports = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.imports = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DeploymentManagerDeploymentTargetEl {
    type O = terrars::BlockAssignable<DeploymentManagerDeploymentTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeploymentManagerDeploymentTargetEl {}

impl BuildDeploymentManagerDeploymentTargetEl {
    pub fn build(self) -> DeploymentManagerDeploymentTargetEl {
        DeploymentManagerDeploymentTargetEl {
            config: core::default::Default::default(),
            imports: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DeploymentManagerDeploymentTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeploymentManagerDeploymentTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DeploymentManagerDeploymentTargetElRef {
        DeploymentManagerDeploymentTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeploymentManagerDeploymentTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<DeploymentManagerDeploymentTargetElConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc = "Get a reference to the value of field `imports` after provisioning.\n"]
    pub fn imports(&self) -> terrars::ListRef<DeploymentManagerDeploymentTargetElImportsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.imports", self.base))
    }
}

#[derive(Serialize)]
pub struct DeploymentManagerDeploymentTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DeploymentManagerDeploymentTimeoutsEl {
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

impl terrars::ToListMappable for DeploymentManagerDeploymentTimeoutsEl {
    type O = terrars::BlockAssignable<DeploymentManagerDeploymentTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDeploymentManagerDeploymentTimeoutsEl {}

impl BuildDeploymentManagerDeploymentTimeoutsEl {
    pub fn build(self) -> DeploymentManagerDeploymentTimeoutsEl {
        DeploymentManagerDeploymentTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DeploymentManagerDeploymentTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DeploymentManagerDeploymentTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DeploymentManagerDeploymentTimeoutsElRef {
        DeploymentManagerDeploymentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DeploymentManagerDeploymentTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct DeploymentManagerDeploymentDynamic {
    labels: std::option::Option<terrars::DynamicBlock<DeploymentManagerDeploymentLabelsEl>>,
    target: std::option::Option<terrars::DynamicBlock<DeploymentManagerDeploymentTargetEl>>,
}
