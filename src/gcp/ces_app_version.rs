#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CesAppVersionData_ {
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
    app_version_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CesAppVersion_Timeouts>,
}

struct CesAppVersion_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CesAppVersionData_>,
}

#[derive(Clone)]
pub struct CesAppVersion(std::rc::Rc<CesAppVersion_>);

impl CesAppVersion {
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

    #[doc = "Set the field `description`.\nThe description of the app version."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe display name of the app version."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CesAppVersion_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_version_id` after provisioning.\nThe ID to use for the app version, which will become the final component\nof the app version's resource name. If not provided, a unique ID will be\nautomatically assigned for the app version."]
    pub fn app_version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_version_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the app version was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nEmail of the user who created the app version."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the app version."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the app version."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the app version.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/versions/{version}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\nA snapshot of the app."]
    pub fn snapshot(&self) -> terrars::ListRef<CesAppVersion_SnapshotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesAppVersion_TimeoutsRef {
        CesAppVersion_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CesAppVersion {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CesAppVersion { }

impl terrars::ToListMappable for CesAppVersion {
    type O = terrars::ListRef<CesAppVersionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CesAppVersion_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_ces_app_version".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCesAppVersion {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub app: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the app version, which will become the final component\nof the app version's resource name. If not provided, a unique ID will be\nautomatically assigned for the app version."]
    pub app_version_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildCesAppVersion {
    pub fn build(self, stack: &mut terrars::Stack) -> CesAppVersion {
        let out = CesAppVersion(std::rc::Rc::new(CesAppVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CesAppVersionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app: self.app,
                app_version_id: self.app_version_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CesAppVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CesAppVersionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `app` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn app(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `app_version_id` after provisioning.\nThe ID to use for the app version, which will become the final component\nof the app version's resource name. If not provided, a unique ID will be\nautomatically assigned for the app version."]
    pub fn app_version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_version_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the app version was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nEmail of the user who created the app version."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the app version."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the app version."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nEtag used to ensure the object hasn't changed during a read-modify-write\noperation. If the etag is empty, the update will overwrite any concurrent\nchanges."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique identifier of the app version.\nFormat:\n'projects/{project}/locations/{location}/apps/{app}/versions/{version}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\nA snapshot of the app."]
    pub fn snapshot(&self) -> terrars::ListRef<CesAppVersion_SnapshotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snapshot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CesAppVersion_TimeoutsRef {
        CesAppVersion_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_AfterAgentCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Agents_AfterAgentCallbacks {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proactive_execution_enabled`.\n"]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_AfterAgentCallbacks {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_AfterAgentCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_AfterAgentCallbacks {}

impl BuildCesAppVersion_Snapshot_Agents_AfterAgentCallbacks {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_AfterAgentCallbacks {
        CesAppVersion_Snapshot_Agents_AfterAgentCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_AfterAgentCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_AfterAgentCallbacksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_AfterAgentCallbacksRef {
        CesAppVersion_Snapshot_Agents_AfterAgentCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_AfterAgentCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\n"]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_AfterModelCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Agents_AfterModelCallbacks {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proactive_execution_enabled`.\n"]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_AfterModelCallbacks {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_AfterModelCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_AfterModelCallbacks {}

impl BuildCesAppVersion_Snapshot_Agents_AfterModelCallbacks {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_AfterModelCallbacks {
        CesAppVersion_Snapshot_Agents_AfterModelCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_AfterModelCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_AfterModelCallbacksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_AfterModelCallbacksRef {
        CesAppVersion_Snapshot_Agents_AfterModelCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_AfterModelCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\n"]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_AfterToolCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Agents_AfterToolCallbacks {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proactive_execution_enabled`.\n"]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_AfterToolCallbacks {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_AfterToolCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_AfterToolCallbacks {}

impl BuildCesAppVersion_Snapshot_Agents_AfterToolCallbacks {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_AfterToolCallbacks {
        CesAppVersion_Snapshot_Agents_AfterToolCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_AfterToolCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_AfterToolCallbacksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_AfterToolCallbacksRef {
        CesAppVersion_Snapshot_Agents_AfterToolCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_AfterToolCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\n"]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_BeforeAgentCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Agents_BeforeAgentCallbacks {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proactive_execution_enabled`.\n"]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_BeforeAgentCallbacks {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_BeforeAgentCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_BeforeAgentCallbacks {}

impl BuildCesAppVersion_Snapshot_Agents_BeforeAgentCallbacks {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_BeforeAgentCallbacks {
        CesAppVersion_Snapshot_Agents_BeforeAgentCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_BeforeAgentCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_BeforeAgentCallbacksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_BeforeAgentCallbacksRef {
        CesAppVersion_Snapshot_Agents_BeforeAgentCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_BeforeAgentCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\n"]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_BeforeModelCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Agents_BeforeModelCallbacks {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proactive_execution_enabled`.\n"]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_BeforeModelCallbacks {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_BeforeModelCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_BeforeModelCallbacks {}

impl BuildCesAppVersion_Snapshot_Agents_BeforeModelCallbacks {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_BeforeModelCallbacks {
        CesAppVersion_Snapshot_Agents_BeforeModelCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_BeforeModelCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_BeforeModelCallbacksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_BeforeModelCallbacksRef {
        CesAppVersion_Snapshot_Agents_BeforeModelCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_BeforeModelCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\n"]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_BeforeToolCallbacks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Agents_BeforeToolCallbacks {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proactive_execution_enabled`.\n"]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_BeforeToolCallbacks {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_BeforeToolCallbacks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_BeforeToolCallbacks {}

impl BuildCesAppVersion_Snapshot_Agents_BeforeToolCallbacks {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_BeforeToolCallbacks {
        CesAppVersion_Snapshot_Agents_BeforeToolCallbacks {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_BeforeToolCallbacksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_BeforeToolCallbacksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_BeforeToolCallbacksRef {
        CesAppVersion_Snapshot_Agents_BeforeToolCallbacksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_BeforeToolCallbacksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\n"]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_LlmAgent {}

impl CesAppVersion_Snapshot_Agents_LlmAgent { }

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_LlmAgent {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_LlmAgent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_LlmAgent {}

impl BuildCesAppVersion_Snapshot_Agents_LlmAgent {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_LlmAgent {
        CesAppVersion_Snapshot_Agents_LlmAgent {}
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_LlmAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_LlmAgentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_Agents_LlmAgentRef {
        CesAppVersion_Snapshot_Agents_LlmAgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_LlmAgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_ModelSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
}

impl CesAppVersion_Snapshot_App_ModelSettings {
    #[doc = "Set the field `model`.\n"]
    pub fn set_model(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temperature = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_ModelSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_ModelSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_ModelSettings {}

impl BuildCesAppVersion_Snapshot_App_ModelSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_ModelSettings {
        CesAppVersion_Snapshot_App_ModelSettings {
            model: core::default::Default::default(),
            temperature: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_ModelSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_ModelSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_App_ModelSettingsRef {
        CesAppVersion_Snapshot_App_ModelSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_ModelSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.base))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_RemoteDialogflowAgent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    input_variable_mapping: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code_variable: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_variable_mapping: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl CesAppVersion_Snapshot_Agents_RemoteDialogflowAgent {
    #[doc = "Set the field `agent`.\n"]
    pub fn set_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_id`.\n"]
    pub fn set_environment_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.environment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flow_id`.\n"]
    pub fn set_flow_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flow_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `input_variable_mapping`.\n"]
    pub fn set_input_variable_mapping(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.input_variable_mapping = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `language_code_variable`.\n"]
    pub fn set_language_code_variable(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.language_code_variable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_variable_mapping`.\n"]
    pub fn set_output_variable_mapping(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.output_variable_mapping = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_RemoteDialogflowAgent {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_RemoteDialogflowAgent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_RemoteDialogflowAgent {}

impl BuildCesAppVersion_Snapshot_Agents_RemoteDialogflowAgent {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_RemoteDialogflowAgent {
        CesAppVersion_Snapshot_Agents_RemoteDialogflowAgent {
            agent: core::default::Default::default(),
            environment_id: core::default::Default::default(),
            flow_id: core::default::Default::default(),
            input_variable_mapping: core::default::Default::default(),
            language_code_variable: core::default::Default::default(),
            output_variable_mapping: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_RemoteDialogflowAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_RemoteDialogflowAgentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_RemoteDialogflowAgentRef {
        CesAppVersion_Snapshot_Agents_RemoteDialogflowAgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_RemoteDialogflowAgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent` after provisioning.\n"]
    pub fn agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\n"]
    pub fn environment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `flow_id` after provisioning.\n"]
    pub fn flow_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_id", self.base))
    }

    #[doc = "Get a reference to the value of field `input_variable_mapping` after provisioning.\n"]
    pub fn input_variable_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.input_variable_mapping", self.base))
    }

    #[doc = "Get a reference to the value of field `language_code_variable` after provisioning.\n"]
    pub fn language_code_variable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code_variable", self.base))
    }

    #[doc = "Get a reference to the value of field `output_variable_mapping` after provisioning.\n"]
    pub fn output_variable_mapping(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.output_variable_mapping", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_Toolsets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    toolset: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Agents_Toolsets {
    #[doc = "Set the field `tool_ids`.\n"]
    pub fn set_tool_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tool_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `toolset`.\n"]
    pub fn set_toolset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.toolset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_Toolsets {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_Toolsets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_Toolsets {}

impl BuildCesAppVersion_Snapshot_Agents_Toolsets {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_Toolsets {
        CesAppVersion_Snapshot_Agents_Toolsets {
            tool_ids: core::default::Default::default(),
            toolset: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_ToolsetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_ToolsetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_Agents_ToolsetsRef {
        CesAppVersion_Snapshot_Agents_ToolsetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_ToolsetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tool_ids` after provisioning.\n"]
    pub fn tool_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `toolset` after provisioning.\n"]
    pub fn toolset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.toolset", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition {
    type O =
        terrars
        ::BlockAssignable<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition {}

impl BuildCesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition {
        CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition {
            expression: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionConditionRef {
        CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeCondition {
    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeCondition {
    type O =
        terrars
        ::BlockAssignable<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeCondition {}

impl BuildCesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeCondition {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeCondition {
        CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeCondition {
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeConditionRef {
        CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression_condition: std
    ::option
    ::Option<
        terrars::ListField<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code_condition: std
    ::option
    ::Option<
        terrars::ListField<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeCondition>,
    >,
}

impl CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer {
    #[doc = "Set the field `expression_condition`.\n"]
    pub fn set_expression_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition,
                        >,
                    >,
    ) -> Self {
        self.expression_condition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code_condition`.\n"]
    pub fn set_python_code_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeCondition,
                        >,
                    >,
    ) -> Self {
        self.python_code_condition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer {}

impl BuildCesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer {
        CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer {
            expression_condition: core::default::Default::default(),
            python_code_condition: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransferRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransferRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransferRef {
        CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransferRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransferRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression_condition` after provisioning.\n"]
    pub fn expression_condition(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expression_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code_condition` after provisioning.\n"]
    pub fn python_code_condition(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_PythonCodeConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_code_condition", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransfer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression_condition: std
    ::option
    ::Option<
        terrars::ListField<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition>,
    >,
}

impl CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransfer {
    #[doc = "Set the field `expression_condition`.\n"]
    pub fn set_expression_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionCondition,
                        >,
                    >,
    ) -> Self {
        self.expression_condition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransfer {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransfer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransfer {}

impl BuildCesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransfer {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransfer {
        CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransfer {
            expression_condition: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransferRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransferRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransferRef {
        CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransferRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransferRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression_condition` after provisioning.\n"]
    pub fn expression_condition(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer_ExpressionConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expression_condition", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents_TransferRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    child_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deterministic_transfer: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_planner_transfer: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransfer>>,
}

impl CesAppVersion_Snapshot_Agents_TransferRules {
    #[doc = "Set the field `child_agent`.\n"]
    pub fn set_child_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.child_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deterministic_transfer`.\n"]
    pub fn set_deterministic_transfer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransfer>>,
    ) -> Self {
        self.deterministic_transfer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_planner_transfer`.\n"]
    pub fn set_disable_planner_transfer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransfer>>,
    ) -> Self {
        self.disable_planner_transfer = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents_TransferRules {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents_TransferRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents_TransferRules {}

impl BuildCesAppVersion_Snapshot_Agents_TransferRules {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents_TransferRules {
        CesAppVersion_Snapshot_Agents_TransferRules {
            child_agent: core::default::Default::default(),
            deterministic_transfer: core::default::Default::default(),
            direction: core::default::Default::default(),
            disable_planner_transfer: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Agents_TransferRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Agents_TransferRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Agents_TransferRulesRef {
        CesAppVersion_Snapshot_Agents_TransferRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Agents_TransferRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `child_agent` after provisioning.\n"]
    pub fn child_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.child_agent", self.base))
    }

    #[doc = "Get a reference to the value of field `deterministic_transfer` after provisioning.\n"]
    pub fn deterministic_transfer(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_TransferRules_DeterministicTransferRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deterministic_transfer", self.base))
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_planner_transfer` after provisioning.\n"]
    pub fn disable_planner_transfer(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_TransferRules_DisablePlannerTransferRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disable_planner_transfer", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Agents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_agent_callbacks: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_AfterAgentCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_model_callbacks: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_AfterModelCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_tool_callbacks: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_AfterToolCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before_agent_callbacks: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_BeforeAgentCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before_model_callbacks: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_BeforeModelCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before_tool_callbacks: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_BeforeToolCallbacks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    child_agents: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generated_summary: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_agent: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_LlmAgent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_dialogflow_agent: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_RemoteDialogflowAgent>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tools: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    toolsets: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_Toolsets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_rules: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Agents_TransferRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Agents {
    #[doc = "Set the field `after_agent_callbacks`.\n"]
    pub fn set_after_agent_callbacks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_AfterAgentCallbacks>>,
    ) -> Self {
        self.after_agent_callbacks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `after_model_callbacks`.\n"]
    pub fn set_after_model_callbacks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_AfterModelCallbacks>>,
    ) -> Self {
        self.after_model_callbacks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `after_tool_callbacks`.\n"]
    pub fn set_after_tool_callbacks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_AfterToolCallbacks>>,
    ) -> Self {
        self.after_tool_callbacks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `before_agent_callbacks`.\n"]
    pub fn set_before_agent_callbacks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_BeforeAgentCallbacks>>,
    ) -> Self {
        self.before_agent_callbacks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `before_model_callbacks`.\n"]
    pub fn set_before_model_callbacks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_BeforeModelCallbacks>>,
    ) -> Self {
        self.before_model_callbacks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `before_tool_callbacks`.\n"]
    pub fn set_before_tool_callbacks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_BeforeToolCallbacks>>,
    ) -> Self {
        self.before_tool_callbacks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `child_agents`.\n"]
    pub fn set_child_agents(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.child_agents = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generated_summary`.\n"]
    pub fn set_generated_summary(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.generated_summary = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrails`.\n"]
    pub fn set_guardrails(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.guardrails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instruction`.\n"]
    pub fn set_instruction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `llm_agent`.\n"]
    pub fn set_llm_agent(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_LlmAgent>>,
    ) -> Self {
        self.llm_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    ) -> Self {
        self.model_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `remote_dialogflow_agent`.\n"]
    pub fn set_remote_dialogflow_agent(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_RemoteDialogflowAgent>>,
    ) -> Self {
        self.remote_dialogflow_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tools`.\n"]
    pub fn set_tools(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `toolsets`.\n"]
    pub fn set_toolsets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_Toolsets>>,
    ) -> Self {
        self.toolsets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transfer_rules`.\n"]
    pub fn set_transfer_rules(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Agents_TransferRules>>,
    ) -> Self {
        self.transfer_rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Agents {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Agents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Agents {}

impl BuildCesAppVersion_Snapshot_Agents {
    pub fn build(self) -> CesAppVersion_Snapshot_Agents {
        CesAppVersion_Snapshot_Agents {
            after_agent_callbacks: core::default::Default::default(),
            after_model_callbacks: core::default::Default::default(),
            after_tool_callbacks: core::default::Default::default(),
            before_agent_callbacks: core::default::Default::default(),
            before_model_callbacks: core::default::Default::default(),
            before_tool_callbacks: core::default::Default::default(),
            child_agents: core::default::Default::default(),
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            etag: core::default::Default::default(),
            generated_summary: core::default::Default::default(),
            guardrails: core::default::Default::default(),
            instruction: core::default::Default::default(),
            llm_agent: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            name: core::default::Default::default(),
            remote_dialogflow_agent: core::default::Default::default(),
            tools: core::default::Default::default(),
            toolsets: core::default::Default::default(),
            transfer_rules: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_AgentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_AgentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_AgentsRef {
        CesAppVersion_Snapshot_AgentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_AgentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `after_agent_callbacks` after provisioning.\n"]
    pub fn after_agent_callbacks(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_AfterAgentCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_agent_callbacks", self.base))
    }

    #[doc = "Get a reference to the value of field `after_model_callbacks` after provisioning.\n"]
    pub fn after_model_callbacks(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_AfterModelCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_model_callbacks", self.base))
    }

    #[doc = "Get a reference to the value of field `after_tool_callbacks` after provisioning.\n"]
    pub fn after_tool_callbacks(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_AfterToolCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_tool_callbacks", self.base))
    }

    #[doc = "Get a reference to the value of field `before_agent_callbacks` after provisioning.\n"]
    pub fn before_agent_callbacks(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_BeforeAgentCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_agent_callbacks", self.base))
    }

    #[doc = "Get a reference to the value of field `before_model_callbacks` after provisioning.\n"]
    pub fn before_model_callbacks(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_BeforeModelCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_model_callbacks", self.base))
    }

    #[doc = "Get a reference to the value of field `before_tool_callbacks` after provisioning.\n"]
    pub fn before_tool_callbacks(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_BeforeToolCallbacksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_tool_callbacks", self.base))
    }

    #[doc = "Get a reference to the value of field `child_agents` after provisioning.\n"]
    pub fn child_agents(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.child_agents", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc = "Get a reference to the value of field `generated_summary` after provisioning.\n"]
    pub fn generated_summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_summary", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\n"]
    pub fn guardrails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrails", self.base))
    }

    #[doc = "Get a reference to the value of field `instruction` after provisioning.\n"]
    pub fn instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `llm_agent` after provisioning.\n"]
    pub fn llm_agent(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_LlmAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_agent", self.base))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `remote_dialogflow_agent` after provisioning.\n"]
    pub fn remote_dialogflow_agent(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_RemoteDialogflowAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_dialogflow_agent", self.base))
    }

    #[doc = "Get a reference to the value of field `tools` after provisioning.\n"]
    pub fn tools(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.base))
    }

    #[doc = "Get a reference to the value of field `toolsets` after provisioning.\n"]
    pub fn toolsets(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_ToolsetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.toolsets", self.base))
    }

    #[doc = "Get a reference to the value of field `transfer_rules` after provisioning.\n"]
    pub fn transfer_rules(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Agents_TransferRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_rules", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prebuilt_ambient_sound: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_gain_db: std::option::Option<terrars::PrimField<f64>>,
}

impl CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfig {
    #[doc = "Set the field `gcs_uri`.\n"]
    pub fn set_gcs_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcs_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prebuilt_ambient_sound`.\n"]
    pub fn set_prebuilt_ambient_sound(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.prebuilt_ambient_sound = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_gain_db`.\n"]
    pub fn set_volume_gain_db(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_gain_db = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfig {}

impl BuildCesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfig {
        CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfig {
            gcs_uri: core::default::Default::default(),
            prebuilt_ambient_sound: core::default::Default::default(),
            volume_gain_db: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfigRef {
        CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcs_uri` after provisioning.\n"]
    pub fn gcs_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `prebuilt_ambient_sound` after provisioning.\n"]
    pub fn prebuilt_ambient_sound(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prebuilt_ambient_sound", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_gain_db` after provisioning.\n"]
    pub fn volume_gain_db(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_gain_db", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    barge_in_awareness: std::option::Option<terrars::PrimField<bool>>,
}

impl CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfig {
    #[doc = "Set the field `barge_in_awareness`.\n"]
    pub fn set_barge_in_awareness(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.barge_in_awareness = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfig {}

impl BuildCesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfig {
        CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfig {
            barge_in_awareness: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfigRef {
        CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `barge_in_awareness` after provisioning.\n"]
    pub fn barge_in_awareness(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.barge_in_awareness", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    speaking_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    voice: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigs {
    #[doc = "Set the field `language_code`.\n"]
    pub fn set_language_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `speaking_rate`.\n"]
    pub fn set_speaking_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.speaking_rate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `voice`.\n"]
    pub fn set_voice(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.voice = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigs {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigs {}

impl BuildCesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigs {
    pub fn build(self) -> CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigs {
        CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigs {
            language_code: core::default::Default::default(),
            speaking_rate: core::default::Default::default(),
            voice: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigsRef {
        CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `language_code` after provisioning.\n"]
    pub fn language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `speaking_rate` after provisioning.\n"]
    pub fn speaking_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.speaking_rate", self.base))
    }

    #[doc = "Get a reference to the value of field `voice` after provisioning.\n"]
    pub fn voice(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.voice", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_AudioProcessingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ambient_sound_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    barge_in_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inactivity_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    synthesize_speech_configs: std
    ::option
    ::Option<terrars::SetField<CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigs>>,
}

impl CesAppVersion_Snapshot_App_AudioProcessingConfig {
    #[doc = "Set the field `ambient_sound_config`.\n"]
    pub fn set_ambient_sound_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfig>>,
    ) -> Self {
        self.ambient_sound_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `barge_in_config`.\n"]
    pub fn set_barge_in_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfig>>,
    ) -> Self {
        self.barge_in_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inactivity_timeout`.\n"]
    pub fn set_inactivity_timeout(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.inactivity_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `synthesize_speech_configs`.\n"]
    pub fn set_synthesize_speech_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::SetField<CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigs>,
                    >,
    ) -> Self {
        self.synthesize_speech_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_AudioProcessingConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_AudioProcessingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_AudioProcessingConfig {}

impl BuildCesAppVersion_Snapshot_App_AudioProcessingConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_App_AudioProcessingConfig {
        CesAppVersion_Snapshot_App_AudioProcessingConfig {
            ambient_sound_config: core::default::Default::default(),
            barge_in_config: core::default::Default::default(),
            inactivity_timeout: core::default::Default::default(),
            synthesize_speech_configs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_AudioProcessingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_AudioProcessingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_AudioProcessingConfigRef {
        CesAppVersion_Snapshot_App_AudioProcessingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_AudioProcessingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ambient_sound_config` after provisioning.\n"]
    pub fn ambient_sound_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_AudioProcessingConfig_AmbientSoundConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ambient_sound_config", self.base))
    }

    #[doc = "Get a reference to the value of field `barge_in_config` after provisioning.\n"]
    pub fn barge_in_config(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_AudioProcessingConfig_BargeInConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.barge_in_config", self.base))
    }

    #[doc = "Get a reference to the value of field `inactivity_timeout` after provisioning.\n"]
    pub fn inactivity_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inactivity_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `synthesize_speech_configs` after provisioning.\n"]
    pub fn synthesize_speech_configs(
        &self,
    ) -> terrars::SetRef<CesAppVersion_Snapshot_App_AudioProcessingConfig_SynthesizeSpeechConfigsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.synthesize_speech_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_ClientCertificateSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    passphrase: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_ClientCertificateSettings {
    #[doc = "Set the field `passphrase`.\n"]
    pub fn set_passphrase(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.passphrase = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key`.\n"]
    pub fn set_private_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.private_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_certificate`.\n"]
    pub fn set_tls_certificate(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tls_certificate = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_ClientCertificateSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_ClientCertificateSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_ClientCertificateSettings {}

impl BuildCesAppVersion_Snapshot_App_ClientCertificateSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_ClientCertificateSettings {
        CesAppVersion_Snapshot_App_ClientCertificateSettings {
            passphrase: core::default::Default::default(),
            private_key: core::default::Default::default(),
            tls_certificate: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_ClientCertificateSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_ClientCertificateSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_ClientCertificateSettingsRef {
        CesAppVersion_Snapshot_App_ClientCertificateSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_ClientCertificateSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `passphrase` after provisioning.\n"]
    pub fn passphrase(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.passphrase", self.base))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_certificate` after provisioning.\n"]
    pub fn tls_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_certificate", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_DataStoreSettings_Engines {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_DataStoreSettings_Engines {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_DataStoreSettings_Engines {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_DataStoreSettings_Engines>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_DataStoreSettings_Engines {}

impl BuildCesAppVersion_Snapshot_App_DataStoreSettings_Engines {
    pub fn build(self) -> CesAppVersion_Snapshot_App_DataStoreSettings_Engines {
        CesAppVersion_Snapshot_App_DataStoreSettings_Engines {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_DataStoreSettings_EnginesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_DataStoreSettings_EnginesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_DataStoreSettings_EnginesRef {
        CesAppVersion_Snapshot_App_DataStoreSettings_EnginesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_DataStoreSettings_EnginesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_DataStoreSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engines: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_DataStoreSettings_Engines>>,
}

impl CesAppVersion_Snapshot_App_DataStoreSettings {
    #[doc = "Set the field `engines`.\n"]
    pub fn set_engines(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_DataStoreSettings_Engines>>,
    ) -> Self {
        self.engines = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_DataStoreSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_DataStoreSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_DataStoreSettings {}

impl BuildCesAppVersion_Snapshot_App_DataStoreSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_DataStoreSettings {
        CesAppVersion_Snapshot_App_DataStoreSettings { engines: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_DataStoreSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_DataStoreSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_DataStoreSettingsRef {
        CesAppVersion_Snapshot_App_DataStoreSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_DataStoreSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `engines` after provisioning.\n"]
    pub fn engines(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_DataStoreSettings_EnginesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.engines", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaProperty {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persona: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaProperty {
    #[doc = "Set the field `persona`.\n"]
    pub fn set_persona(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.persona = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaProperty {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaProperty>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaProperty {}

impl BuildCesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaProperty {
    pub fn build(self) -> CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaProperty {
        CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaProperty {
            persona: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaPropertyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaPropertyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaPropertyRef {
        CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaPropertyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaPropertyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `persona` after provisioning.\n"]
    pub fn persona(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persona", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modality: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    theme: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_widget_title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfig {
    #[doc = "Set the field `modality`.\n"]
    pub fn set_modality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.modality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `theme`.\n"]
    pub fn set_theme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.theme = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_widget_title`.\n"]
    pub fn set_web_widget_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.web_widget_title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfig {}

impl BuildCesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfig {
        CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfig {
            modality: core::default::Default::default(),
            theme: core::default::Default::default(),
            web_widget_title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfigRef {
        CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `modality` after provisioning.\n"]
    pub fn modality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modality", self.base))
    }

    #[doc = "Get a reference to the value of field `theme` after provisioning.\n"]
    pub fn theme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.theme", self.base))
    }

    #[doc = "Get a reference to the value of field `web_widget_title` after provisioning.\n"]
    pub fn web_widget_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_widget_title", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thumbnail_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    waba_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfig {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phone_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number_id`.\n"]
    pub fn set_phone_number_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.phone_number_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thumbnail_url`.\n"]
    pub fn set_thumbnail_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.thumbnail_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `waba_id`.\n"]
    pub fn set_waba_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.waba_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfig {}

impl BuildCesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfig {
        CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfig {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            phone_number: core::default::Default::default(),
            phone_number_id: core::default::Default::default(),
            thumbnail_url: core::default::Default::default(),
            waba_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfigRef {
        CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number_id` after provisioning.\n"]
    pub fn phone_number_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number_id", self.base))
    }

    #[doc = "Get a reference to the value of field `thumbnail_url` after provisioning.\n"]
    pub fn thumbnail_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thumbnail_url", self.base))
    }

    #[doc = "Get a reference to the value of field `waba_id` after provisioning.\n"]
    pub fn waba_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.waba_id", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_DefaultChannelProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_barge_in_control: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_dtmf: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persona_property: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaProperty>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_widget_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    whatsapp_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfig>>,
}

impl CesAppVersion_Snapshot_App_DefaultChannelProfile {
    #[doc = "Set the field `channel_type`.\n"]
    pub fn set_channel_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_barge_in_control`.\n"]
    pub fn set_disable_barge_in_control(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_barge_in_control = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_dtmf`.\n"]
    pub fn set_disable_dtmf(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_dtmf = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `persona_property`.\n"]
    pub fn set_persona_property(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaProperty>>,
    ) -> Self {
        self.persona_property = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `profile_id`.\n"]
    pub fn set_profile_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.profile_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_widget_config`.\n"]
    pub fn set_web_widget_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfig>>,
    ) -> Self {
        self.web_widget_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `whatsapp_config`.\n"]
    pub fn set_whatsapp_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfig>>,
    ) -> Self {
        self.whatsapp_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_DefaultChannelProfile {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_DefaultChannelProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_DefaultChannelProfile {}

impl BuildCesAppVersion_Snapshot_App_DefaultChannelProfile {
    pub fn build(self) -> CesAppVersion_Snapshot_App_DefaultChannelProfile {
        CesAppVersion_Snapshot_App_DefaultChannelProfile {
            channel_type: core::default::Default::default(),
            disable_barge_in_control: core::default::Default::default(),
            disable_dtmf: core::default::Default::default(),
            persona_property: core::default::Default::default(),
            profile_id: core::default::Default::default(),
            web_widget_config: core::default::Default::default(),
            whatsapp_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_DefaultChannelProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_DefaultChannelProfileRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_DefaultChannelProfileRef {
        CesAppVersion_Snapshot_App_DefaultChannelProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_DefaultChannelProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel_type` after provisioning.\n"]
    pub fn channel_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel_type", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_barge_in_control` after provisioning.\n"]
    pub fn disable_barge_in_control(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_barge_in_control", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_dtmf` after provisioning.\n"]
    pub fn disable_dtmf(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_dtmf", self.base))
    }

    #[doc = "Get a reference to the value of field `persona_property` after provisioning.\n"]
    pub fn persona_property(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_DefaultChannelProfile_PersonaPropertyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.persona_property", self.base))
    }

    #[doc = "Get a reference to the value of field `profile_id` after provisioning.\n"]
    pub fn profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.profile_id", self.base))
    }

    #[doc = "Get a reference to the value of field `web_widget_config` after provisioning.\n"]
    pub fn web_widget_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_DefaultChannelProfile_WebWidgetConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.web_widget_config", self.base))
    }

    #[doc = "Get a reference to the value of field `whatsapp_config` after provisioning.\n"]
    pub fn whatsapp_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_DefaultChannelProfile_WhatsappConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.whatsapp_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    escalate_session: std::option::Option<terrars::PrimField<bool>>,
}

impl CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfig {
    #[doc = "Set the field `escalate_session`.\n"]
    pub fn set_escalate_session(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.escalate_session = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfig {}

impl BuildCesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfig {
        CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfig {
            escalate_session: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfigRef {
        CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `escalate_session` after provisioning.\n"]
    pub fn escalate_session(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.escalate_session", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_fallback_messages: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_fallback_attempts: std::option::Option<terrars::PrimField<f64>>,
}

impl CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfig {
    #[doc = "Set the field `custom_fallback_messages`.\n"]
    pub fn set_custom_fallback_messages(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_fallback_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_fallback_attempts`.\n"]
    pub fn set_max_fallback_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_fallback_attempts = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfig {}

impl BuildCesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfig {
        CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfig {
            custom_fallback_messages: core::default::Default::default(),
            max_fallback_attempts: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfigRef {
        CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_fallback_messages` after provisioning.\n"]
    pub fn custom_fallback_messages(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_fallback_messages", self.base))
    }

    #[doc = "Get a reference to the value of field `max_fallback_attempts` after provisioning.\n"]
    pub fn max_fallback_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_fallback_attempts", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_ErrorHandlingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_session_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fallback_response_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfig>>,
}

impl CesAppVersion_Snapshot_App_ErrorHandlingSettings {
    #[doc = "Set the field `end_session_config`.\n"]
    pub fn set_end_session_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfig>>,
    ) -> Self {
        self.end_session_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_strategy`.\n"]
    pub fn set_error_handling_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.error_handling_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fallback_response_config`.\n"]
    pub fn set_fallback_response_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfig>,
                    >,
    ) -> Self {
        self.fallback_response_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_ErrorHandlingSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_ErrorHandlingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_ErrorHandlingSettings {}

impl BuildCesAppVersion_Snapshot_App_ErrorHandlingSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_ErrorHandlingSettings {
        CesAppVersion_Snapshot_App_ErrorHandlingSettings {
            end_session_config: core::default::Default::default(),
            error_handling_strategy: core::default::Default::default(),
            fallback_response_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_ErrorHandlingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_ErrorHandlingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_ErrorHandlingSettingsRef {
        CesAppVersion_Snapshot_App_ErrorHandlingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_ErrorHandlingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_session_config` after provisioning.\n"]
    pub fn end_session_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_ErrorHandlingSettings_EndSessionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.end_session_config", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_strategy` after provisioning.\n"]
    pub fn error_handling_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_handling_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `fallback_response_config` after provisioning.\n"]
    pub fn fallback_response_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_ErrorHandlingSettings_FallbackResponseConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fallback_response_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_invocation_parameter_correctness_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
    #[doc = "Set the field `tool_invocation_parameter_correctness_threshold`.\n"]
    pub fn set_tool_invocation_parameter_correctness_threshold(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.tool_invocation_parameter_correctness_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
    type O =
        terrars
        ::BlockAssignable<
            CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {}

impl BuildCesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
    pub fn build(
        self,
    ) -> CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds {
            tool_invocation_parameter_correctness_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef {
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `tool_invocation_parameter_correctness_threshold` after provisioning.\n"]
    pub fn tool_invocation_parameter_correctness_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tool_invocation_parameter_correctness_threshold", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extra_tool_call_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
    #[doc = "Set the field `extra_tool_call_behavior`.\n"]
    pub fn set_extra_tool_call_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.extra_tool_call_behavior = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
    type O =
        terrars
        ::BlockAssignable<
            CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {}

impl BuildCesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
    pub fn build(
        self,
    ) -> CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings {
            extra_tool_call_behavior: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef {
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `extra_tool_call_behavior` after provisioning.\n"]
    pub fn extra_tool_call_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extra_tool_call_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overall_tool_invocation_correctness_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    semantic_similarity_channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    semantic_similarity_success_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
    #[doc = "Set the field `overall_tool_invocation_correctness_threshold`.\n"]
    pub fn set_overall_tool_invocation_correctness_threshold(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.overall_tool_invocation_correctness_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `semantic_similarity_channel`.\n"]
    pub fn set_semantic_similarity_channel(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.semantic_similarity_channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `semantic_similarity_success_threshold`.\n"]
    pub fn set_semantic_similarity_success_threshold(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.semantic_similarity_success_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
    type O =
        terrars
        ::BlockAssignable<
            CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {}

impl BuildCesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
    pub fn build(
        self,
    ) -> CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds {
            overall_tool_invocation_correctness_threshold: core::default::Default::default(),
            semantic_similarity_channel: core::default::Default::default(),
            semantic_similarity_success_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef {
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `overall_tool_invocation_correctness_threshold` after provisioning.\n"]
    pub fn overall_tool_invocation_correctness_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.overall_tool_invocation_correctness_threshold", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `semantic_similarity_channel` after provisioning.\n"]
    pub fn semantic_similarity_channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.semantic_similarity_channel", self.base))
    }

    #[doc = "Get a reference to the value of field `semantic_similarity_success_threshold` after provisioning.\n"]
    pub fn semantic_similarity_success_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.semantic_similarity_success_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expectation_level_metrics_thresholds: std
    ::option
    ::Option<
        terrars
        ::ListField<
            CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_matching_settings: std
    ::option
    ::Option<
        terrars
        ::ListField<
            CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    turn_level_metrics_thresholds: std
    ::option
    ::Option<
        terrars
        ::ListField<
            CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds,
        >,
    >,
}

impl CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
    #[doc = "Set the field `expectation_level_metrics_thresholds`.\n"]
    pub fn set_expectation_level_metrics_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholds,
                        >,
                    >,
    ) -> Self {
        self.expectation_level_metrics_thresholds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tool_matching_settings`.\n"]
    pub fn set_tool_matching_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettings,
                        >,
                    >,
    ) -> Self {
        self.tool_matching_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `turn_level_metrics_thresholds`.\n"]
    pub fn set_turn_level_metrics_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholds,
                        >,
                    >,
    ) -> Self {
        self.turn_level_metrics_thresholds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
    type O =
        terrars
        ::BlockAssignable<CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {}

impl BuildCesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
    pub fn build(self) -> CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds {
            expectation_level_metrics_thresholds: core::default::Default::default(),
            tool_matching_settings: core::default::Default::default(),
            turn_level_metrics_thresholds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef {
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expectation_level_metrics_thresholds` after provisioning.\n"]
    pub fn expectation_level_metrics_thresholds(
        &self,
    ) -> terrars
    ::ListRef<
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ExpectationLevelMetricsThresholdsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expectation_level_metrics_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_matching_settings` after provisioning.\n"]
    pub fn tool_matching_settings(
        &self,
    ) -> terrars
    ::ListRef<
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_ToolMatchingSettingsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_matching_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `turn_level_metrics_thresholds` after provisioning.\n"]
    pub fn turn_level_metrics_thresholds(
        &self,
    ) -> terrars
    ::ListRef<
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds_TurnLevelMetricsThresholdsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.turn_level_metrics_thresholds", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_EvaluationMetricsThresholds {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    golden_evaluation_metrics_thresholds: std
    ::option
    ::Option<
        terrars::ListField<CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    golden_hallucination_metric_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scenario_hallucination_metric_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_EvaluationMetricsThresholds {
    #[doc = "Set the field `golden_evaluation_metrics_thresholds`.\n"]
    pub fn set_golden_evaluation_metrics_thresholds(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholds,
                        >,
                    >,
    ) -> Self {
        self.golden_evaluation_metrics_thresholds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `golden_hallucination_metric_behavior`.\n"]
    pub fn set_golden_hallucination_metric_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.golden_hallucination_metric_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scenario_hallucination_metric_behavior`.\n"]
    pub fn set_scenario_hallucination_metric_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scenario_hallucination_metric_behavior = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_EvaluationMetricsThresholds {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_EvaluationMetricsThresholds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_EvaluationMetricsThresholds {}

impl BuildCesAppVersion_Snapshot_App_EvaluationMetricsThresholds {
    pub fn build(self) -> CesAppVersion_Snapshot_App_EvaluationMetricsThresholds {
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholds {
            golden_evaluation_metrics_thresholds: core::default::Default::default(),
            golden_hallucination_metric_behavior: core::default::Default::default(),
            scenario_hallucination_metric_behavior: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_EvaluationMetricsThresholdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_EvaluationMetricsThresholdsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_EvaluationMetricsThresholdsRef {
        CesAppVersion_Snapshot_App_EvaluationMetricsThresholdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_EvaluationMetricsThresholdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `golden_evaluation_metrics_thresholds` after provisioning.\n"]
    pub fn golden_evaluation_metrics_thresholds(
        &self,
    ) -> terrars
    ::ListRef<CesAppVersion_Snapshot_App_EvaluationMetricsThresholds_GoldenEvaluationMetricsThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.golden_evaluation_metrics_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `golden_hallucination_metric_behavior` after provisioning.\n"]
    pub fn golden_hallucination_metric_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.golden_hallucination_metric_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `scenario_hallucination_metric_behavior` after provisioning.\n"]
    pub fn scenario_hallucination_metric_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.scenario_hallucination_metric_behavior", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_LanguageSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_language_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_multilingual_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fallback_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_language_codes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CesAppVersion_Snapshot_App_LanguageSettings {
    #[doc = "Set the field `default_language_code`.\n"]
    pub fn set_default_language_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_language_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_multilingual_support`.\n"]
    pub fn set_enable_multilingual_support(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_multilingual_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fallback_action`.\n"]
    pub fn set_fallback_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fallback_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_language_codes`.\n"]
    pub fn set_supported_language_codes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_language_codes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_LanguageSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_LanguageSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_LanguageSettings {}

impl BuildCesAppVersion_Snapshot_App_LanguageSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_LanguageSettings {
        CesAppVersion_Snapshot_App_LanguageSettings {
            default_language_code: core::default::Default::default(),
            enable_multilingual_support: core::default::Default::default(),
            fallback_action: core::default::Default::default(),
            supported_language_codes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_LanguageSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_LanguageSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_LanguageSettingsRef {
        CesAppVersion_Snapshot_App_LanguageSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_LanguageSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_language_code` after provisioning.\n"]
    pub fn default_language_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_language_code", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_multilingual_support` after provisioning.\n"]
    pub fn enable_multilingual_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_multilingual_support", self.base))
    }

    #[doc = "Get a reference to the value of field `fallback_action` after provisioning.\n"]
    pub fn fallback_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_action", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_language_codes` after provisioning.\n"]
    pub fn supported_language_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_language_codes", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_path_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfig {
    #[doc = "Set the field `gcs_bucket`.\n"]
    pub fn set_gcs_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcs_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcs_path_prefix`.\n"]
    pub fn set_gcs_path_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcs_path_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfig {}

impl BuildCesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfig {
        CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfig {
            gcs_bucket: core::default::Default::default(),
            gcs_path_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfigRef {
        CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcs_bucket` after provisioning.\n"]
    pub fn gcs_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `gcs_path_prefix` after provisioning.\n"]
    pub fn gcs_path_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_path_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettings {
    #[doc = "Set the field `dataset`.\n"]
    pub fn set_dataset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettings {}

impl BuildCesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettings {
        CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettings {
            dataset: core::default::Default::default(),
            enabled: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettingsRef {
        CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset` after provisioning.\n"]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_cloud_logging: std::option::Option<terrars::PrimField<bool>>,
}

impl CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettings {
    #[doc = "Set the field `enable_cloud_logging`.\n"]
    pub fn set_enable_cloud_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_cloud_logging = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettings {}

impl BuildCesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettings {
        CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettings {
            enable_cloud_logging: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettingsRef {
        CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_cloud_logging` after provisioning.\n"]
    pub fn enable_cloud_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_cloud_logging", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_conversation_logging: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_window: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettings {
    #[doc = "Set the field `disable_conversation_logging`.\n"]
    pub fn set_disable_conversation_logging(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_conversation_logging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_window`.\n"]
    pub fn set_retention_window(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retention_window = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettings {}

impl BuildCesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettings {
        CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettings {
            disable_conversation_logging: core::default::Default::default(),
            retention_window: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettingsRef {
        CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_conversation_logging` after provisioning.\n"]
    pub fn disable_conversation_logging(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_conversation_logging", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_window` after provisioning.\n"]
    pub fn retention_window(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_window", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_metrics_opted_out: std::option::Option<terrars::PrimField<bool>>,
}

impl CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettings {
    #[doc = "Set the field `llm_metrics_opted_out`.\n"]
    pub fn set_llm_metrics_opted_out(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.llm_metrics_opted_out = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettings {}

impl BuildCesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettings {
        CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettings {
            llm_metrics_opted_out: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettingsRef {
        CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `llm_metrics_opted_out` after provisioning.\n"]
    pub fn llm_metrics_opted_out(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.llm_metrics_opted_out", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deidentify_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_redaction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfig {
    #[doc = "Set the field `deidentify_template`.\n"]
    pub fn set_deidentify_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.deidentify_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_redaction`.\n"]
    pub fn set_enable_redaction(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_redaction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inspect_template`.\n"]
    pub fn set_inspect_template(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.inspect_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_LoggingSettings_RedactionConfig {}

impl BuildCesAppVersion_Snapshot_App_LoggingSettings_RedactionConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfig {
        CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfig {
            deidentify_template: core::default::Default::default(),
            enable_redaction: core::default::Default::default(),
            inspect_template: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfigRef {
        CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deidentify_template` after provisioning.\n"]
    pub fn deidentify_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deidentify_template", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_redaction` after provisioning.\n"]
    pub fn enable_redaction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_redaction", self.base))
    }

    #[doc = "Get a reference to the value of field `inspect_template` after provisioning.\n"]
    pub fn inspect_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_template", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_LoggingSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_recording_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_export_settings: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_logging_settings: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conversation_logging_settings: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_analysis_settings: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redaction_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfig>>,
}

impl CesAppVersion_Snapshot_App_LoggingSettings {
    #[doc = "Set the field `audio_recording_config`.\n"]
    pub fn set_audio_recording_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfig>>,
    ) -> Self {
        self.audio_recording_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigquery_export_settings`.\n"]
    pub fn set_bigquery_export_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettings>>,
    ) -> Self {
        self.bigquery_export_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_logging_settings`.\n"]
    pub fn set_cloud_logging_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettings>>,
    ) -> Self {
        self.cloud_logging_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conversation_logging_settings`.\n"]
    pub fn set_conversation_logging_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettings>,
                    >,
    ) -> Self {
        self.conversation_logging_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_analysis_settings`.\n"]
    pub fn set_metric_analysis_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettings>>,
    ) -> Self {
        self.metric_analysis_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `redaction_config`.\n"]
    pub fn set_redaction_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfig>>,
    ) -> Self {
        self.redaction_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_LoggingSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_LoggingSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_LoggingSettings {}

impl BuildCesAppVersion_Snapshot_App_LoggingSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_LoggingSettings {
        CesAppVersion_Snapshot_App_LoggingSettings {
            audio_recording_config: core::default::Default::default(),
            bigquery_export_settings: core::default::Default::default(),
            cloud_logging_settings: core::default::Default::default(),
            conversation_logging_settings: core::default::Default::default(),
            metric_analysis_settings: core::default::Default::default(),
            redaction_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_LoggingSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_LoggingSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_App_LoggingSettingsRef {
        CesAppVersion_Snapshot_App_LoggingSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_LoggingSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_recording_config` after provisioning.\n"]
    pub fn audio_recording_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_LoggingSettings_AudioRecordingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_recording_config", self.base))
    }

    #[doc = "Get a reference to the value of field `bigquery_export_settings` after provisioning.\n"]
    pub fn bigquery_export_settings(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_LoggingSettings_BigqueryExportSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_export_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_logging_settings` after provisioning.\n"]
    pub fn cloud_logging_settings(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_LoggingSettings_CloudLoggingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_logging_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `conversation_logging_settings` after provisioning.\n"]
    pub fn conversation_logging_settings(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_LoggingSettings_ConversationLoggingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conversation_logging_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_analysis_settings` after provisioning.\n"]
    pub fn metric_analysis_settings(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_LoggingSettings_MetricAnalysisSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_analysis_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `redaction_config` after provisioning.\n"]
    pub fn redaction_config(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_LoggingSettings_RedactionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redaction_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_TimeZoneSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_App_TimeZoneSettings {
    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_TimeZoneSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_TimeZoneSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_TimeZoneSettings {}

impl BuildCesAppVersion_Snapshot_App_TimeZoneSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_TimeZoneSettings {
        CesAppVersion_Snapshot_App_TimeZoneSettings { time_zone: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_TimeZoneSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_TimeZoneSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_TimeZoneSettingsRef {
        CesAppVersion_Snapshot_App_TimeZoneSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_TimeZoneSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_VariableDeclarations_Schema {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_of: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    defs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "enum", skip_serializing_if = "std::option::Option::is_none")]
    enum_: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_items: std::option::Option<terrars::PrimField<bool>>,
}

impl CesAppVersion_Snapshot_App_VariableDeclarations_Schema {
    #[doc = "Set the field `additional_properties`.\n"]
    pub fn set_additional_properties(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `any_of`.\n"]
    pub fn set_any_of(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.any_of = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `defs`.\n"]
    pub fn set_defs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.defs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enum_`.\n"]
    pub fn set_enum(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enum_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\n"]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_items`.\n"]
    pub fn set_prefix_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ref_`.\n"]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unique_items`.\n"]
    pub fn set_unique_items(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unique_items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_VariableDeclarations_Schema {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_VariableDeclarations_Schema>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_VariableDeclarations_Schema {}

impl BuildCesAppVersion_Snapshot_App_VariableDeclarations_Schema {
    pub fn build(self) -> CesAppVersion_Snapshot_App_VariableDeclarations_Schema {
        CesAppVersion_Snapshot_App_VariableDeclarations_Schema {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            type_: core::default::Default::default(),
            unique_items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_VariableDeclarations_SchemaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_VariableDeclarations_SchemaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_VariableDeclarations_SchemaRef {
        CesAppVersion_Snapshot_App_VariableDeclarations_SchemaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_VariableDeclarations_SchemaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_properties` after provisioning.\n"]
    pub fn additional_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `any_of` after provisioning.\n"]
    pub fn any_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc = "Get a reference to the value of field `defs` after provisioning.\n"]
    pub fn defs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_` after provisioning.\n"]
    pub fn enum_(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc = "Get a reference to the value of field `nullable` after provisioning.\n"]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_items` after provisioning.\n"]
    pub fn prefix_items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `unique_items` after provisioning.\n"]
    pub fn unique_items(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_VariableDeclarations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_VariableDeclarations_Schema>>,
}

impl CesAppVersion_Snapshot_App_VariableDeclarations {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schema`.\n"]
    pub fn set_schema(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_VariableDeclarations_Schema>>,
    ) -> Self {
        self.schema = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_VariableDeclarations {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_VariableDeclarations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_VariableDeclarations {}

impl BuildCesAppVersion_Snapshot_App_VariableDeclarations {
    pub fn build(self) -> CesAppVersion_Snapshot_App_VariableDeclarations {
        CesAppVersion_Snapshot_App_VariableDeclarations {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            schema: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_VariableDeclarationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_VariableDeclarationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_App_VariableDeclarationsRef {
        CesAppVersion_Snapshot_App_VariableDeclarationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_VariableDeclarationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `schema` after provisioning.\n"]
    pub fn schema(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_VariableDeclarations_SchemaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App_VpcScSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_origins: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CesAppVersion_Snapshot_App_VpcScSettings {
    #[doc = "Set the field `allowed_origins`.\n"]
    pub fn set_allowed_origins(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_origins = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App_VpcScSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App_VpcScSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App_VpcScSettings {}

impl BuildCesAppVersion_Snapshot_App_VpcScSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_App_VpcScSettings {
        CesAppVersion_Snapshot_App_VpcScSettings { allowed_origins: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_App_VpcScSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_App_VpcScSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_App_VpcScSettingsRef {
        CesAppVersion_Snapshot_App_VpcScSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_App_VpcScSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_origins` after provisioning.\n"]
    pub fn allowed_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_App {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_processing_config: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_AudioProcessingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_settings: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_ClientCertificateSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_DataStoreSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_channel_profile: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_DefaultChannelProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_handling_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_ErrorHandlingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_metrics_thresholds: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_App_EvaluationMetricsThresholds>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_instruction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrails: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_LanguageSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_TimeZoneSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variable_declarations: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_VariableDeclarations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_sc_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_VpcScSettings>>,
}

impl CesAppVersion_Snapshot_App {
    #[doc = "Set the field `audio_processing_config`.\n"]
    pub fn set_audio_processing_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_AudioProcessingConfig>>,
    ) -> Self {
        self.audio_processing_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_certificate_settings`.\n"]
    pub fn set_client_certificate_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_ClientCertificateSettings>>,
    ) -> Self {
        self.client_certificate_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_store_settings`.\n"]
    pub fn set_data_store_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_DataStoreSettings>>,
    ) -> Self {
        self.data_store_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_channel_profile`.\n"]
    pub fn set_default_channel_profile(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_DefaultChannelProfile>>,
    ) -> Self {
        self.default_channel_profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_count`.\n"]
    pub fn set_deployment_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.deployment_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_handling_settings`.\n"]
    pub fn set_error_handling_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_ErrorHandlingSettings>>,
    ) -> Self {
        self.error_handling_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluation_metrics_thresholds`.\n"]
    pub fn set_evaluation_metrics_thresholds(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_EvaluationMetricsThresholds>>,
    ) -> Self {
        self.evaluation_metrics_thresholds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `global_instruction`.\n"]
    pub fn set_global_instruction(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.global_instruction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrails`.\n"]
    pub fn set_guardrails(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.guardrails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `language_settings`.\n"]
    pub fn set_language_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_LanguageSettings>>,
    ) -> Self {
        self.language_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_settings`.\n"]
    pub fn set_logging_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_LoggingSettings>>,
    ) -> Self {
        self.logging_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    ) -> Self {
        self.model_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_agent`.\n"]
    pub fn set_root_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone_settings`.\n"]
    pub fn set_time_zone_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_TimeZoneSettings>>,
    ) -> Self {
        self.time_zone_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `variable_declarations`.\n"]
    pub fn set_variable_declarations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_VariableDeclarations>>,
    ) -> Self {
        self.variable_declarations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_sc_settings`.\n"]
    pub fn set_vpc_sc_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_VpcScSettings>>,
    ) -> Self {
        self.vpc_sc_settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_App {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_App>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_App {}

impl BuildCesAppVersion_Snapshot_App {
    pub fn build(self) -> CesAppVersion_Snapshot_App {
        CesAppVersion_Snapshot_App {
            audio_processing_config: core::default::Default::default(),
            client_certificate_settings: core::default::Default::default(),
            create_time: core::default::Default::default(),
            data_store_settings: core::default::Default::default(),
            default_channel_profile: core::default::Default::default(),
            deployment_count: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            error_handling_settings: core::default::Default::default(),
            etag: core::default::Default::default(),
            evaluation_metrics_thresholds: core::default::Default::default(),
            global_instruction: core::default::Default::default(),
            guardrails: core::default::Default::default(),
            language_settings: core::default::Default::default(),
            logging_settings: core::default::Default::default(),
            metadata: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            name: core::default::Default::default(),
            root_agent: core::default::Default::default(),
            time_zone_settings: core::default::Default::default(),
            update_time: core::default::Default::default(),
            variable_declarations: core::default::Default::default(),
            vpc_sc_settings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_AppRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_AppRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_AppRef {
        CesAppVersion_Snapshot_AppRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_AppRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audio_processing_config` after provisioning.\n"]
    pub fn audio_processing_config(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_AudioProcessingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_processing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `client_certificate_settings` after provisioning.\n"]
    pub fn client_certificate_settings(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_ClientCertificateSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_certificate_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `data_store_settings` after provisioning.\n"]
    pub fn data_store_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_DataStoreSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `default_channel_profile` after provisioning.\n"]
    pub fn default_channel_profile(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_DefaultChannelProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_channel_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment_count` after provisioning.\n"]
    pub fn deployment_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_count", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `error_handling_settings` after provisioning.\n"]
    pub fn error_handling_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_ErrorHandlingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_handling_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluation_metrics_thresholds` after provisioning.\n"]
    pub fn evaluation_metrics_thresholds(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_App_EvaluationMetricsThresholdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluation_metrics_thresholds", self.base))
    }

    #[doc = "Get a reference to the value of field `global_instruction` after provisioning.\n"]
    pub fn global_instruction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_instruction", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\n"]
    pub fn guardrails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrails", self.base))
    }

    #[doc = "Get a reference to the value of field `language_settings` after provisioning.\n"]
    pub fn language_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_LanguageSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.language_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `logging_settings` after provisioning.\n"]
    pub fn logging_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_LoggingSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `root_agent` after provisioning.\n"]
    pub fn root_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_agent", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone_settings` after provisioning.\n"]
    pub fn time_zone_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_TimeZoneSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_zone_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }

    #[doc = "Get a reference to the value of field `variable_declarations` after provisioning.\n"]
    pub fn variable_declarations(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_VariableDeclarationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.variable_declarations", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_sc_settings` after provisioning.\n"]
    pub fn vpc_sc_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_VpcScSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_sc_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransfer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_agent: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransfer {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_agent`.\n"]
    pub fn set_target_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_agent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransfer {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransfer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransfer {}

impl BuildCesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransfer {
    pub fn build(self) -> CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransfer {
        CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransfer {
            display_name: core::default::Default::default(),
            target_agent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransferRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransferRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransferRef {
        CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransferRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransferRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_agent` after provisioning.\n"]
    pub fn target_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_agent", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Examples_Messages_Chunks_Image {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mime_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Examples_Messages_Chunks_Image {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mime_type`.\n"]
    pub fn set_mime_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mime_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Examples_Messages_Chunks_Image {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Examples_Messages_Chunks_Image>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Examples_Messages_Chunks_Image {}

impl BuildCesAppVersion_Snapshot_Examples_Messages_Chunks_Image {
    pub fn build(self) -> CesAppVersion_Snapshot_Examples_Messages_Chunks_Image {
        CesAppVersion_Snapshot_Examples_Messages_Chunks_Image {
            data: core::default::Default::default(),
            mime_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Examples_Messages_Chunks_ImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Examples_Messages_Chunks_ImageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Examples_Messages_Chunks_ImageRef {
        CesAppVersion_Snapshot_Examples_Messages_Chunks_ImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Examples_Messages_Chunks_ImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `mime_type` after provisioning.\n"]
    pub fn mime_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mime_type", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    toolset: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool {
    #[doc = "Set the field `tool_id`.\n"]
    pub fn set_tool_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tool_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `toolset`.\n"]
    pub fn set_toolset(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.toolset = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool {}

impl BuildCesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool {
    pub fn build(self) -> CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool {
        CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool {
            tool_id: core::default::Default::default(),
            toolset: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetToolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetToolRef {
        CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tool_id` after provisioning.\n"]
    pub fn tool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool_id", self.base))
    }

    #[doc = "Get a reference to the value of field `toolset` after provisioning.\n"]
    pub fn toolset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.toolset", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    toolset_tool: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool>>,
}

impl CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `toolset_tool`.\n"]
    pub fn set_toolset_tool(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool>>,
    ) -> Self {
        self.toolset_tool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall {}

impl BuildCesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall {
    pub fn build(self) -> CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall {
        CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall {
            args: core::default::Default::default(),
            display_name: core::default::Default::default(),
            id: core::default::Default::default(),
            tool: core::default::Default::default(),
            toolset_tool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCallRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCallRef {
        CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCallRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCallRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool", self.base))
    }

    #[doc = "Get a reference to the value of field `toolset_tool` after provisioning.\n"]
    pub fn toolset_tool(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.toolset_tool", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    toolset_tool: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool>>,
}

impl CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponse {
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tool`.\n"]
    pub fn set_tool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `toolset_tool`.\n"]
    pub fn set_toolset_tool(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetTool>>,
    ) -> Self {
        self.toolset_tool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponse {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponse {}

impl BuildCesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponse {
    pub fn build(self) -> CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponse {
        CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponse {
            display_name: core::default::Default::default(),
            id: core::default::Default::default(),
            response: core::default::Default::default(),
            tool: core::default::Default::default(),
            toolset_tool: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponseRef {
        CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response", self.base))
    }

    #[doc = "Get a reference to the value of field `tool` after provisioning.\n"]
    pub fn tool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tool", self.base))
    }

    #[doc = "Get a reference to the value of field `toolset_tool` after provisioning.\n"]
    pub fn toolset_tool(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall_ToolsetToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.toolset_tool", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Examples_Messages_Chunks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent_transfer: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransfer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_Image>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_call: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tool_response: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_variables: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Examples_Messages_Chunks {
    #[doc = "Set the field `agent_transfer`.\n"]
    pub fn set_agent_transfer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransfer>>,
    ) -> Self {
        self.agent_transfer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_Image>>,
    ) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tool_call`.\n"]
    pub fn set_tool_call(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCall>>,
    ) -> Self {
        self.tool_call = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tool_response`.\n"]
    pub fn set_tool_response(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponse>>,
    ) -> Self {
        self.tool_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_variables`.\n"]
    pub fn set_updated_variables(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_variables = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Examples_Messages_Chunks {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Examples_Messages_Chunks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Examples_Messages_Chunks {}

impl BuildCesAppVersion_Snapshot_Examples_Messages_Chunks {
    pub fn build(self) -> CesAppVersion_Snapshot_Examples_Messages_Chunks {
        CesAppVersion_Snapshot_Examples_Messages_Chunks {
            agent_transfer: core::default::Default::default(),
            image: core::default::Default::default(),
            text: core::default::Default::default(),
            tool_call: core::default::Default::default(),
            tool_response: core::default::Default::default(),
            updated_variables: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Examples_Messages_ChunksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Examples_Messages_ChunksRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Examples_Messages_ChunksRef {
        CesAppVersion_Snapshot_Examples_Messages_ChunksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Examples_Messages_ChunksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_transfer` after provisioning.\n"]
    pub fn agent_transfer(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Examples_Messages_Chunks_AgentTransferRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agent_transfer", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Examples_Messages_Chunks_ImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_call` after provisioning.\n"]
    pub fn tool_call(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolCallRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_call", self.base))
    }

    #[doc = "Get a reference to the value of field `tool_response` after provisioning.\n"]
    pub fn tool_response(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Examples_Messages_Chunks_ToolResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tool_response", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_variables` after provisioning.\n"]
    pub fn updated_variables(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_variables", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Examples_Messages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chunks: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Examples_Messages {
    #[doc = "Set the field `chunks`.\n"]
    pub fn set_chunks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages_Chunks>>,
    ) -> Self {
        self.chunks = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role`.\n"]
    pub fn set_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Examples_Messages {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Examples_Messages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Examples_Messages {}

impl BuildCesAppVersion_Snapshot_Examples_Messages {
    pub fn build(self) -> CesAppVersion_Snapshot_Examples_Messages {
        CesAppVersion_Snapshot_Examples_Messages {
            chunks: core::default::Default::default(),
            role: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Examples_MessagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Examples_MessagesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_Examples_MessagesRef {
        CesAppVersion_Snapshot_Examples_MessagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Examples_MessagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chunks` after provisioning.\n"]
    pub fn chunks(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Examples_Messages_ChunksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chunks", self.base))
    }

    #[doc = "Get a reference to the value of field `role` after provisioning.\n"]
    pub fn role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Examples {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_agent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invalid: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    messages: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Examples {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entry_agent`.\n"]
    pub fn set_entry_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entry_agent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `invalid`.\n"]
    pub fn set_invalid(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invalid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `messages`.\n"]
    pub fn set_messages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Examples_Messages>>,
    ) -> Self {
        self.messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Examples {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Examples>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Examples {}

impl BuildCesAppVersion_Snapshot_Examples {
    pub fn build(self) -> CesAppVersion_Snapshot_Examples {
        CesAppVersion_Snapshot_Examples {
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            entry_agent: core::default::Default::default(),
            etag: core::default::Default::default(),
            invalid: core::default::Default::default(),
            messages: core::default::Default::default(),
            name: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_ExamplesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_ExamplesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_ExamplesRef {
        CesAppVersion_Snapshot_ExamplesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_ExamplesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `entry_agent` after provisioning.\n"]
    pub fn entry_agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_agent", self.base))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc = "Get a reference to the value of field `invalid` after provisioning.\n"]
    pub fn invalid(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invalid", self.base))
    }

    #[doc = "Get a reference to the value of field `messages` after provisioning.\n"]
    pub fn messages(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Examples_MessagesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.messages", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswer {
    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswer {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswer {}

impl BuildCesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswer {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswer {
        CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswer { prompt: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswerRef {
        CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_Responses {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_Responses {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `text`.\n"]
    pub fn set_text(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_Responses {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_Responses>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_Responses {}

impl BuildCesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_Responses {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_Responses {
        CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_Responses {
            disabled: core::default::Default::default(),
            text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_ResponsesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_ResponsesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_ResponsesRef {
        CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_ResponsesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_ResponsesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    responses: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_Responses>>,
}

impl CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately {
    #[doc = "Set the field `responses`.\n"]
    pub fn set_responses(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_Responses>>,
    ) -> Self {
        self.responses = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_Action_RespondImmediately {}

impl BuildCesAppVersion_Snapshot_Guardrails_Action_RespondImmediately {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately {
        CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately { responses: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_Action_RespondImmediatelyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_Action_RespondImmediatelyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_Action_RespondImmediatelyRef {
        CesAppVersion_Snapshot_Guardrails_Action_RespondImmediatelyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_Action_RespondImmediatelyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `responses` after provisioning.\n"]
    pub fn responses(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately_ResponsesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.responses", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_Action_TransferAgent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agent: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_Action_TransferAgent {
    #[doc = "Set the field `agent`.\n"]
    pub fn set_agent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.agent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_Action_TransferAgent {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_Action_TransferAgent>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_Action_TransferAgent {}

impl BuildCesAppVersion_Snapshot_Guardrails_Action_TransferAgent {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_Action_TransferAgent {
        CesAppVersion_Snapshot_Guardrails_Action_TransferAgent { agent: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_Action_TransferAgentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_Action_TransferAgentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_Action_TransferAgentRef {
        CesAppVersion_Snapshot_Guardrails_Action_TransferAgentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_Action_TransferAgentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent` after provisioning.\n"]
    pub fn agent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generative_answer: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    respond_immediately: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_agent: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_Action_TransferAgent>>,
}

impl CesAppVersion_Snapshot_Guardrails_Action {
    #[doc = "Set the field `generative_answer`.\n"]
    pub fn set_generative_answer(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswer>>,
    ) -> Self {
        self.generative_answer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `respond_immediately`.\n"]
    pub fn set_respond_immediately(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_Action_RespondImmediately>>,
    ) -> Self {
        self.respond_immediately = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transfer_agent`.\n"]
    pub fn set_transfer_agent(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_Action_TransferAgent>>,
    ) -> Self {
        self.transfer_agent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_Action {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_Action {}

impl BuildCesAppVersion_Snapshot_Guardrails_Action {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_Action {
        CesAppVersion_Snapshot_Guardrails_Action {
            generative_answer: core::default::Default::default(),
            respond_immediately: core::default::Default::default(),
            transfer_agent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_Guardrails_ActionRef {
        CesAppVersion_Snapshot_Guardrails_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `generative_answer` after provisioning.\n"]
    pub fn generative_answer(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_Action_GenerativeAnswerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generative_answer", self.base))
    }

    #[doc = "Get a reference to the value of field `respond_immediately` after provisioning.\n"]
    pub fn respond_immediately(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_Action_RespondImmediatelyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.respond_immediately", self.base))
    }

    #[doc = "Get a reference to the value of field `transfer_agent` after provisioning.\n"]
    pub fn transfer_agent(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_Action_TransferAgentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transfer_agent", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallback {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallback {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proactive_execution_enabled`.\n"]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallback {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallback {}

impl BuildCesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallback {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallback {
        CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallback {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallbackRef {
        CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\n"]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallback {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallback {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proactive_execution_enabled`.\n"]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallback {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallback {}

impl BuildCesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallback {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallback {
        CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallback {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallbackRef {
        CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\n"]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallback {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallback {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proactive_execution_enabled`.\n"]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallback {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallback {}

impl BuildCesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallback {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallback {
        CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallback {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallbackRef {
        CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\n"]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallback {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proactive_execution_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallback {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `proactive_execution_enabled`.\n"]
    pub fn set_proactive_execution_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.proactive_execution_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallback {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallback {}

impl BuildCesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallback {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallback {
        CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallback {
            description: core::default::Default::default(),
            disabled: core::default::Default::default(),
            proactive_execution_enabled: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallbackRef {
        CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `proactive_execution_enabled` after provisioning.\n"]
    pub fn proactive_execution_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proactive_execution_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_CodeCallback {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_agent_callback: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallback>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    after_model_callback: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallback>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before_agent_callback: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallback>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    before_model_callback: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallback>>,
}

impl CesAppVersion_Snapshot_Guardrails_CodeCallback {
    #[doc = "Set the field `after_agent_callback`.\n"]
    pub fn set_after_agent_callback(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallback>>,
    ) -> Self {
        self.after_agent_callback = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `after_model_callback`.\n"]
    pub fn set_after_model_callback(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallback>>,
    ) -> Self {
        self.after_model_callback = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `before_agent_callback`.\n"]
    pub fn set_before_agent_callback(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallback>>,
    ) -> Self {
        self.before_agent_callback = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `before_model_callback`.\n"]
    pub fn set_before_model_callback(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallback>>,
    ) -> Self {
        self.before_model_callback = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_CodeCallback {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_CodeCallback>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_CodeCallback {}

impl BuildCesAppVersion_Snapshot_Guardrails_CodeCallback {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_CodeCallback {
        CesAppVersion_Snapshot_Guardrails_CodeCallback {
            after_agent_callback: core::default::Default::default(),
            after_model_callback: core::default::Default::default(),
            before_agent_callback: core::default::Default::default(),
            before_model_callback: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_CodeCallbackRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_CodeCallbackRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_CodeCallbackRef {
        CesAppVersion_Snapshot_Guardrails_CodeCallbackRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_CodeCallbackRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `after_agent_callback` after provisioning.\n"]
    pub fn after_agent_callback(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterAgentCallbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_agent_callback", self.base))
    }

    #[doc = "Get a reference to the value of field `after_model_callback` after provisioning.\n"]
    pub fn after_model_callback(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_CodeCallback_AfterModelCallbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.after_model_callback", self.base))
    }

    #[doc = "Get a reference to the value of field `before_agent_callback` after provisioning.\n"]
    pub fn before_agent_callback(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeAgentCallbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_agent_callback", self.base))
    }

    #[doc = "Get a reference to the value of field `before_model_callback` after provisioning.\n"]
    pub fn before_model_callback(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_CodeCallback_BeforeModelCallbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.before_model_callback", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_ContentFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banned_contents: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banned_contents_in_agent_response: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    banned_contents_in_user_input: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disregard_diacritics: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_ContentFilter {
    #[doc = "Set the field `banned_contents`.\n"]
    pub fn set_banned_contents(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.banned_contents = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `banned_contents_in_agent_response`.\n"]
    pub fn set_banned_contents_in_agent_response(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.banned_contents_in_agent_response = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `banned_contents_in_user_input`.\n"]
    pub fn set_banned_contents_in_user_input(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.banned_contents_in_user_input = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disregard_diacritics`.\n"]
    pub fn set_disregard_diacritics(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disregard_diacritics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_type`.\n"]
    pub fn set_match_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.match_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_ContentFilter {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_ContentFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_ContentFilter {}

impl BuildCesAppVersion_Snapshot_Guardrails_ContentFilter {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_ContentFilter {
        CesAppVersion_Snapshot_Guardrails_ContentFilter {
            banned_contents: core::default::Default::default(),
            banned_contents_in_agent_response: core::default::Default::default(),
            banned_contents_in_user_input: core::default::Default::default(),
            disregard_diacritics: core::default::Default::default(),
            match_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_ContentFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_ContentFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_ContentFilterRef {
        CesAppVersion_Snapshot_Guardrails_ContentFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_ContentFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `banned_contents` after provisioning.\n"]
    pub fn banned_contents(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.banned_contents", self.base))
    }

    #[doc = "Get a reference to the value of field `banned_contents_in_agent_response` after provisioning.\n"]
    pub fn banned_contents_in_agent_response(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.banned_contents_in_agent_response", self.base))
    }

    #[doc = "Get a reference to the value of field `banned_contents_in_user_input` after provisioning.\n"]
    pub fn banned_contents_in_user_input(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.banned_contents_in_user_input", self.base))
    }

    #[doc = "Get a reference to the value of field `disregard_diacritics` after provisioning.\n"]
    pub fn disregard_diacritics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disregard_diacritics", self.base))
    }

    #[doc = "Get a reference to the value of field `match_type` after provisioning.\n"]
    pub fn match_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_type", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_LlmPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_conversation_messages: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_LlmPolicy {
    #[doc = "Set the field `fail_open`.\n"]
    pub fn set_fail_open(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_open = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_conversation_messages`.\n"]
    pub fn set_max_conversation_messages(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_conversation_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    ) -> Self {
        self.model_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_scope`.\n"]
    pub fn set_policy_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_LlmPolicy {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_LlmPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_LlmPolicy {}

impl BuildCesAppVersion_Snapshot_Guardrails_LlmPolicy {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_LlmPolicy {
        CesAppVersion_Snapshot_Guardrails_LlmPolicy {
            fail_open: core::default::Default::default(),
            max_conversation_messages: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            policy_scope: core::default::Default::default(),
            prompt: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_LlmPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_LlmPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_LlmPolicyRef {
        CesAppVersion_Snapshot_Guardrails_LlmPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_LlmPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\n"]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc = "Get a reference to the value of field `max_conversation_messages` after provisioning.\n"]
    pub fn max_conversation_messages(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_conversation_messages", self.base))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_scope` after provisioning.\n"]
    pub fn policy_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_scope", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_conversation_messages: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicy {
    #[doc = "Set the field `fail_open`.\n"]
    pub fn set_fail_open(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_open = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_conversation_messages`.\n"]
    pub fn set_max_conversation_messages(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_conversation_messages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    ) -> Self {
        self.model_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_scope`.\n"]
    pub fn set_policy_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicy {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicy {}

impl BuildCesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicy {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicy {
        CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicy {
            fail_open: core::default::Default::default(),
            max_conversation_messages: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            policy_scope: core::default::Default::default(),
            prompt: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicyRef {
        CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\n"]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }

    #[doc = "Get a reference to the value of field `max_conversation_messages` after provisioning.\n"]
    pub fn max_conversation_messages(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_conversation_messages", self.base))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_scope` after provisioning.\n"]
    pub fn policy_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_scope", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_prompt_template: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettings {
    #[doc = "Set the field `default_prompt_template`.\n"]
    pub fn set_default_prompt_template(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.default_prompt_template = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettings {}

impl BuildCesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettings {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettings {
        CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettings {
            default_prompt_template: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettingsRef {
        CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_prompt_template` after provisioning.\n"]
    pub fn default_prompt_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_prompt_template", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_policy: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_settings: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fail_open: std::option::Option<terrars::PrimField<bool>>,
}

impl CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity {
    #[doc = "Set the field `custom_policy`.\n"]
    pub fn set_custom_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicy>>,
    ) -> Self {
        self.custom_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_settings`.\n"]
    pub fn set_default_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettings>>,
    ) -> Self {
        self.default_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fail_open`.\n"]
    pub fn set_fail_open(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.fail_open = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_LlmPromptSecurity {}

impl BuildCesAppVersion_Snapshot_Guardrails_LlmPromptSecurity {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity {
        CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity {
            custom_policy: core::default::Default::default(),
            default_settings: core::default::Default::default(),
            fail_open: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_LlmPromptSecurityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_LlmPromptSecurityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_LlmPromptSecurityRef {
        CesAppVersion_Snapshot_Guardrails_LlmPromptSecurityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_LlmPromptSecurityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_policy` after provisioning.\n"]
    pub fn custom_policy(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_CustomPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `default_settings` after provisioning.\n"]
    pub fn default_settings(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity_DefaultSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `fail_open` after provisioning.\n"]
    pub fn fail_open(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fail_open", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    category: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettings {
    #[doc = "Set the field `category`.\n"]
    pub fn set_category(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.category = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettings {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettings {}

impl BuildCesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettings {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettings {
        CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettings {
            category: core::default::Default::default(),
            threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettingsRef {
        CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `category` after provisioning.\n"]
    pub fn category(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.category", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails_ModelSafety {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    safety_settings: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettings>>,
}

impl CesAppVersion_Snapshot_Guardrails_ModelSafety {
    #[doc = "Set the field `safety_settings`.\n"]
    pub fn set_safety_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettings>>,
    ) -> Self {
        self.safety_settings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails_ModelSafety {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails_ModelSafety>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails_ModelSafety {}

impl BuildCesAppVersion_Snapshot_Guardrails_ModelSafety {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails_ModelSafety {
        CesAppVersion_Snapshot_Guardrails_ModelSafety { safety_settings: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Guardrails_ModelSafetyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Guardrails_ModelSafetyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Guardrails_ModelSafetyRef {
        CesAppVersion_Snapshot_Guardrails_ModelSafetyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Guardrails_ModelSafetyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `safety_settings` after provisioning.\n"]
    pub fn safety_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_ModelSafety_SafetySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.safety_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Guardrails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_callback: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_CodeCallback>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_filter: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_ContentFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_policy: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_LlmPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_prompt_security: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_safety: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails_ModelSafety>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Guardrails {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_Action>>,
    ) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `code_callback`.\n"]
    pub fn set_code_callback(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_CodeCallback>>,
    ) -> Self {
        self.code_callback = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_filter`.\n"]
    pub fn set_content_filter(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_ContentFilter>>,
    ) -> Self {
        self.content_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `llm_policy`.\n"]
    pub fn set_llm_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_LlmPolicy>>,
    ) -> Self {
        self.llm_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `llm_prompt_security`.\n"]
    pub fn set_llm_prompt_security(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurity>>,
    ) -> Self {
        self.llm_prompt_security = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_safety`.\n"]
    pub fn set_model_safety(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails_ModelSafety>>,
    ) -> Self {
        self.model_safety = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Guardrails {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Guardrails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Guardrails {}

impl BuildCesAppVersion_Snapshot_Guardrails {
    pub fn build(self) -> CesAppVersion_Snapshot_Guardrails {
        CesAppVersion_Snapshot_Guardrails {
            action: core::default::Default::default(),
            code_callback: core::default::Default::default(),
            content_filter: core::default::Default::default(),
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            enabled: core::default::Default::default(),
            etag: core::default::Default::default(),
            llm_policy: core::default::Default::default(),
            llm_prompt_security: core::default::Default::default(),
            model_safety: core::default::Default::default(),
            name: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_GuardrailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_GuardrailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_GuardrailsRef {
        CesAppVersion_Snapshot_GuardrailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_GuardrailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `code_callback` after provisioning.\n"]
    pub fn code_callback(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_CodeCallbackRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_callback", self.base))
    }

    #[doc = "Get a reference to the value of field `content_filter` after provisioning.\n"]
    pub fn content_filter(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_ContentFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_filter", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc = "Get a reference to the value of field `llm_policy` after provisioning.\n"]
    pub fn llm_policy(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_LlmPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `llm_prompt_security` after provisioning.\n"]
    pub fn llm_prompt_security(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_LlmPromptSecurityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_prompt_security", self.base))
    }

    #[doc = "Get a reference to the value of field `model_safety` after provisioning.\n"]
    pub fn model_safety(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Guardrails_ModelSafetyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_safety", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_ClientFunction_Parameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_of: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    defs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "enum", skip_serializing_if = "std::option::Option::is_none")]
    enum_: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_items: std::option::Option<terrars::PrimField<bool>>,
}

impl CesAppVersion_Snapshot_Tools_ClientFunction_Parameters {
    #[doc = "Set the field `additional_properties`.\n"]
    pub fn set_additional_properties(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `any_of`.\n"]
    pub fn set_any_of(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.any_of = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `defs`.\n"]
    pub fn set_defs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.defs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enum_`.\n"]
    pub fn set_enum(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enum_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\n"]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_items`.\n"]
    pub fn set_prefix_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ref_`.\n"]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unique_items`.\n"]
    pub fn set_unique_items(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unique_items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_ClientFunction_Parameters {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_ClientFunction_Parameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_ClientFunction_Parameters {}

impl BuildCesAppVersion_Snapshot_Tools_ClientFunction_Parameters {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_ClientFunction_Parameters {
        CesAppVersion_Snapshot_Tools_ClientFunction_Parameters {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            type_: core::default::Default::default(),
            unique_items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_ClientFunction_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_ClientFunction_ParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_ClientFunction_ParametersRef {
        CesAppVersion_Snapshot_Tools_ClientFunction_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_ClientFunction_ParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_properties` after provisioning.\n"]
    pub fn additional_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `any_of` after provisioning.\n"]
    pub fn any_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc = "Get a reference to the value of field `defs` after provisioning.\n"]
    pub fn defs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_` after provisioning.\n"]
    pub fn enum_(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc = "Get a reference to the value of field `nullable` after provisioning.\n"]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_items` after provisioning.\n"]
    pub fn prefix_items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `unique_items` after provisioning.\n"]
    pub fn unique_items(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_ClientFunction_Response {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_of: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    defs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "enum", skip_serializing_if = "std::option::Option::is_none")]
    enum_: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nullable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_items: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    required: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unique_items: std::option::Option<terrars::PrimField<bool>>,
}

impl CesAppVersion_Snapshot_Tools_ClientFunction_Response {
    #[doc = "Set the field `additional_properties`.\n"]
    pub fn set_additional_properties(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `any_of`.\n"]
    pub fn set_any_of(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.any_of = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default`.\n"]
    pub fn set_default(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `defs`.\n"]
    pub fn set_defs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.defs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enum_`.\n"]
    pub fn set_enum(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enum_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nullable`.\n"]
    pub fn set_nullable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.nullable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_items`.\n"]
    pub fn set_prefix_items(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_items = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ref_`.\n"]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `required`.\n"]
    pub fn set_required(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unique_items`.\n"]
    pub fn set_unique_items(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.unique_items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_ClientFunction_Response {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_ClientFunction_Response>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_ClientFunction_Response {}

impl BuildCesAppVersion_Snapshot_Tools_ClientFunction_Response {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_ClientFunction_Response {
        CesAppVersion_Snapshot_Tools_ClientFunction_Response {
            additional_properties: core::default::Default::default(),
            any_of: core::default::Default::default(),
            default: core::default::Default::default(),
            defs: core::default::Default::default(),
            description: core::default::Default::default(),
            enum_: core::default::Default::default(),
            items: core::default::Default::default(),
            nullable: core::default::Default::default(),
            prefix_items: core::default::Default::default(),
            properties: core::default::Default::default(),
            ref_: core::default::Default::default(),
            required: core::default::Default::default(),
            type_: core::default::Default::default(),
            unique_items: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_ClientFunction_ResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_ClientFunction_ResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_ClientFunction_ResponseRef {
        CesAppVersion_Snapshot_Tools_ClientFunction_ResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_ClientFunction_ResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_properties` after provisioning.\n"]
    pub fn additional_properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `any_of` after provisioning.\n"]
    pub fn any_of(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.any_of", self.base))
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.base))
    }

    #[doc = "Get a reference to the value of field `defs` after provisioning.\n"]
    pub fn defs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.defs", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_` after provisioning.\n"]
    pub fn enum_(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc = "Get a reference to the value of field `nullable` after provisioning.\n"]
    pub fn nullable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nullable", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_items` after provisioning.\n"]
    pub fn prefix_items(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_items", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `ref_` after provisioning.\n"]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc = "Get a reference to the value of field `required` after provisioning.\n"]
    pub fn required(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.required", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `unique_items` after provisioning.\n"]
    pub fn unique_items(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unique_items", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_ClientFunction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_ClientFunction_Parameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_ClientFunction_Response>>,
}

impl CesAppVersion_Snapshot_Tools_ClientFunction {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_ClientFunction_Parameters>>,
    ) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response`.\n"]
    pub fn set_response(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_ClientFunction_Response>>,
    ) -> Self {
        self.response = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_ClientFunction {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_ClientFunction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_ClientFunction {}

impl BuildCesAppVersion_Snapshot_Tools_ClientFunction {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_ClientFunction {
        CesAppVersion_Snapshot_Tools_ClientFunction {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            parameters: core::default::Default::default(),
            response: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_ClientFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_ClientFunctionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_ClientFunctionRef {
        CesAppVersion_Snapshot_Tools_ClientFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_ClientFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_ClientFunction_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `response` after provisioning.\n"]
    pub fn response(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_ClientFunction_ResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost_amount: std::option::Option<terrars::PrimField<f64>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
    #[doc = "Set the field `attribute_value`.\n"]
    pub fn set_attribute_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boost_amount`.\n"]
    pub fn set_boost_amount(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boost_amount = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
    type O =
        terrars
        ::BlockAssignable<
            CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
    pub fn build(
        self,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
        CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints {
            attribute_value: core::default::Default::default(),
            boost_amount: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_value` after provisioning.\n"]
    pub fn attribute_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_value", self.base))
    }

    #[doc = "Get a reference to the value of field `boost_amount` after provisioning.\n"]
    pub fn boost_amount(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boost_amount", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_points: std
    ::option
    ::Option<
        terrars
        ::ListField<
            CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interpolation_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
    #[doc = "Set the field `attribute_type`.\n"]
    pub fn set_attribute_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_points`.\n"]
    pub fn set_control_points(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPoints,
                        >,
                    >,
    ) -> Self {
        self.control_points = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_name`.\n"]
    pub fn set_field_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interpolation_type`.\n"]
    pub fn set_interpolation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.interpolation_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
    type O =
        terrars
        ::BlockAssignable<
            CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
    pub fn build(
        self,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
        CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec {
            attribute_type: core::default::Default::default(),
            control_points: core::default::Default::default(),
            field_name: core::default::Default::default(),
            interpolation_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute_type` after provisioning.\n"]
    pub fn attribute_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_type", self.base))
    }

    #[doc = "Get a reference to the value of field `control_points` after provisioning.\n"]
    pub fn control_points(
        &self,
    ) -> terrars
    ::ListRef<
        CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec_ControlPointsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_points", self.base))
    }

    #[doc = "Get a reference to the value of field `field_name` after provisioning.\n"]
    pub fn field_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }

    #[doc = "Get a reference to the value of field `interpolation_type` after provisioning.\n"]
    pub fn interpolation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interpolation_type", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost_control_spec: std
    ::option
    ::Option<
        terrars
        ::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
    #[doc = "Set the field `boost`.\n"]
    pub fn set_boost(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.boost = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `boost_control_spec`.\n"]
    pub fn set_boost_control_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpec,
                        >,
                    >,
    ) -> Self {
        self.boost_control_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.condition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
    type O =
        terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
        CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs {
            boost: core::default::Default::default(),
            boost_control_spec: core::default::Default::default(),
            condition: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `boost` after provisioning.\n"]
    pub fn boost(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boost", self.base))
    }

    #[doc = "Get a reference to the value of field `boost_control_spec` after provisioning.\n"]
    pub fn boost_control_spec(
        &self,
    ) -> terrars
    ::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs_BoostControlSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boost_control_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition_boost_specs: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec {
    #[doc = "Set the field `condition_boost_specs`.\n"]
    pub fn set_condition_boost_specs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecs>,
                    >,
    ) -> Self {
        self.condition_boost_specs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec {
        CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec {
            condition_boost_specs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_SpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_SpecRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition_boost_specs` after provisioning.\n"]
    pub fn condition_boost_specs(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec_ConditionBoostSpecsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition_boost_specs", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_stores: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs {
    #[doc = "Set the field `data_stores`.\n"]
    pub fn set_data_stores(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data_stores = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_Spec>>,
    ) -> Self {
        self.spec = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs {
        CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs {
            data_stores: core::default::Default::default(),
            spec: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecsRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_stores` after provisioning.\n"]
    pub fn data_stores(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_stores", self.base))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection_display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfig {
    #[doc = "Set the field `collection`.\n"]
    pub fn set_collection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.collection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `collection_display_name`.\n"]
    pub fn set_collection_display_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.collection_display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfig {
    type O =
        terrars
        ::BlockAssignable<
            CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfig {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfig {
    pub fn build(
        self,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfig {
        CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfig {
            collection: core::default::Default::default(),
            collection_display_name: core::default::Default::default(),
            data_source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfigRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection` after provisioning.\n"]
    pub fn collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection", self.base))
    }

    #[doc = "Get a reference to the value of field `collection_display_name` after provisioning.\n"]
    pub fn collection_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_config: std
    ::option
    ::Option<
        terrars
        ::ListField<
            CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfig,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_processing_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore {
    #[doc = "Set the field `connector_config`.\n"]
    pub fn set_connector_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfig,
                        >,
                    >,
    ) -> Self {
        self.connector_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `document_processing_mode`.\n"]
    pub fn set_document_processing_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.document_processing_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore {
    type O =
        terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore {
        CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore {
            connector_config: core::default::Default::default(),
            create_time: core::default::Default::default(),
            display_name: core::default::Default::default(),
            document_processing_mode: core::default::Default::default(),
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector_config` after provisioning.\n"]
    pub fn connector_config(
        &self,
    ) -> terrars
    ::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore_ConnectorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_config", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `document_processing_mode` after provisioning.\n"]
    pub fn document_processing_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.document_processing_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources {
    #[doc = "Set the field `data_store`.\n"]
    pub fn set_data_store(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStore,
                        >,
                    >,
    ) -> Self {
        self.data_store = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources {
        CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources {
            data_store: core::default::Default::default(),
            filter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSourcesRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_store` after provisioning.\n"]
    pub fn data_store(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources_DataStoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_sources: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource {
    #[doc = "Set the field `data_store_sources`.\n"]
    pub fn set_data_store_sources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSources>,
                    >,
    ) -> Self {
        self.data_store_sources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine`.\n"]
    pub fn set_engine(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.engine = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource {
        CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource {
            data_store_sources: core::default::Default::default(),
            engine: core::default::Default::default(),
            filter: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSourceRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_store_sources` after provisioning.\n"]
    pub fn data_store_sources(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource_DataStoreSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_sources", self.base))
    }

    #[doc = "Get a reference to the value of field `engine` after provisioning.\n"]
    pub fn engine(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grounding_level: std::option::Option<terrars::PrimField<f64>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfig {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grounding_level`.\n"]
    pub fn set_grounding_level(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.grounding_level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfig {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfig {
        CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfig {
            disabled: core::default::Default::default(),
            grounding_level: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfigRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `grounding_level` after provisioning.\n"]
    pub fn grounding_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grounding_level", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfig {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    ) -> Self {
        self.model_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfig {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfig {
        CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfig {
            disabled: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            prompt: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfigRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_settings: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prompt: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfig {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_settings`.\n"]
    pub fn set_model_settings(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App_ModelSettings>>,
    ) -> Self {
        self.model_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prompt`.\n"]
    pub fn set_prompt(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prompt = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfig {
    type O =
        terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfig {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfig {
        CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfig {
            disabled: core::default::Default::default(),
            model_settings: core::default::Default::default(),
            prompt: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfigRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `model_settings` after provisioning.\n"]
    pub fn model_settings(&self) -> terrars::ListRef<CesAppVersion_Snapshot_App_ModelSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `prompt` after provisioning.\n"]
    pub fn prompt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prompt", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grounding_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modality_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rewriter_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    summarization_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfig>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs {
    #[doc = "Set the field `grounding_config`.\n"]
    pub fn set_grounding_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfig>,
                    >,
    ) -> Self {
        self.grounding_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `modality_type`.\n"]
    pub fn set_modality_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.modality_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rewriter_config`.\n"]
    pub fn set_rewriter_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfig>,
                    >,
    ) -> Self {
        self.rewriter_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `summarization_config`.\n"]
    pub fn set_summarization_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfig>,
                    >,
    ) -> Self {
        self.summarization_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs {
        CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs {
            grounding_config: core::default::Default::default(),
            modality_type: core::default::Default::default(),
            rewriter_config: core::default::Default::default(),
            summarization_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigsRef {
        CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grounding_config` after provisioning.\n"]
    pub fn grounding_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_GroundingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grounding_config", self.base))
    }

    #[doc = "Get a reference to the value of field `modality_type` after provisioning.\n"]
    pub fn modality_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.modality_type", self.base))
    }

    #[doc = "Get a reference to the value of field `rewriter_config` after provisioning.\n"]
    pub fn rewriter_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_RewriterConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rewriter_config", self.base))
    }

    #[doc = "Get a reference to the value of field `summarization_config` after provisioning.\n"]
    pub fn summarization_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs_SummarizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.summarization_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boost_specs: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_source: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_results: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    modality_configs: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_DataStoreTool {
    #[doc = "Set the field `boost_specs`.\n"]
    pub fn set_boost_specs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecs>>,
    ) -> Self {
        self.boost_specs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_source`.\n"]
    pub fn set_engine_source(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSource>>,
    ) -> Self {
        self.engine_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_results`.\n"]
    pub fn set_max_results(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_results = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `modality_configs`.\n"]
    pub fn set_modality_configs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigs>>,
    ) -> Self {
        self.modality_configs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_DataStoreTool {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_DataStoreTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_DataStoreTool {}

impl BuildCesAppVersion_Snapshot_Tools_DataStoreTool {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_DataStoreTool {
        CesAppVersion_Snapshot_Tools_DataStoreTool {
            boost_specs: core::default::Default::default(),
            description: core::default::Default::default(),
            engine_source: core::default::Default::default(),
            max_results: core::default::Default::default(),
            modality_configs: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_DataStoreToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_DataStoreToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_Tools_DataStoreToolRef {
        CesAppVersion_Snapshot_Tools_DataStoreToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_DataStoreToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `boost_specs` after provisioning.\n"]
    pub fn boost_specs(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_BoostSpecsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boost_specs", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `engine_source` after provisioning.\n"]
    pub fn engine_source(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_EngineSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.engine_source", self.base))
    }

    #[doc = "Get a reference to the value of field `max_results` after provisioning.\n"]
    pub fn max_results(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_results", self.base))
    }

    #[doc = "Get a reference to the value of field `modality_configs` after provisioning.\n"]
    pub fn modality_configs(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_DataStoreTool_ModalityConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.modality_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_GoogleSearchTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_domains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_GoogleSearchTool {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_domains`.\n"]
    pub fn set_exclude_domains(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_GoogleSearchTool {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_GoogleSearchTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_GoogleSearchTool {}

impl BuildCesAppVersion_Snapshot_Tools_GoogleSearchTool {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_GoogleSearchTool {
        CesAppVersion_Snapshot_Tools_GoogleSearchTool {
            description: core::default::Default::default(),
            exclude_domains: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_GoogleSearchToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_GoogleSearchToolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_GoogleSearchToolRef {
        CesAppVersion_Snapshot_Tools_GoogleSearchToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_GoogleSearchToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_domains` after provisioning.\n"]
    pub fn exclude_domains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_domains", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig {
    #[doc = "Set the field `api_key_secret_version`.\n"]
    pub fn set_api_key_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.api_key_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_location`.\n"]
    pub fn set_request_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.request_location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig {}

impl BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig {
            api_key_secret_version: core::default::Default::default(),
            key_name: core::default::Default::default(),
            request_location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfigRef {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_secret_version` after provisioning.\n"]
    pub fn api_key_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_key_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `request_location` after provisioning.\n"]
    pub fn request_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_location", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_grant_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scopes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig {
    #[doc = "Set the field `client_id`.\n"]
    pub fn set_client_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.client_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_secret_version`.\n"]
    pub fn set_client_secret_version(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_grant_type`.\n"]
    pub fn set_oauth_grant_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oauth_grant_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scopes`.\n"]
    pub fn set_scopes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_endpoint`.\n"]
    pub fn set_token_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig {}

impl BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig {
            client_id: core::default::Default::default(),
            client_secret_version: core::default::Default::default(),
            oauth_grant_type: core::default::Default::default(),
            scopes: core::default::Default::default(),
            token_endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfigRef {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret_version` after provisioning.\n"]
    pub fn client_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret_version", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_grant_type` after provisioning.\n"]
    pub fn oauth_grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth_grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `scopes` after provisioning.\n"]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `token_endpoint` after provisioning.\n"]
    pub fn token_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig {
    #[doc = "Set the field `service_account`.\n"]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig {
    type O =
        terrars
        ::BlockAssignable<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig {}

impl BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig {
            service_account: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfigRef {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service_account` after provisioning.\n"]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig { }

impl terrars
::ToListMappable for CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {
    type O =
        terrars
        ::BlockAssignable<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {}

impl BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig {}
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_auth_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_id_token_auth_config: std
    ::option
    ::Option<
        terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig>,
    >,
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig>,
                    >,
    ) -> Self {
        self.api_key_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig>,
                    >,
    ) -> Self {
        self.oauth_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig,
                        >,
                    >,
    ) -> Self {
        self.service_account_auth_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_agent_id_token_auth_config`.\n"]
    pub fn set_service_agent_id_token_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig,
                        >,
                    >,
    ) -> Self {
        self.service_agent_id_token_auth_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication {}

impl BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication {
            api_key_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            service_agent_id_token_auth_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthenticationRef {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_auth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_agent_id_token_auth_config` after provisioning.\n"]
    pub fn service_agent_id_token_auth_config(
        &self,
    ) -> terrars
    ::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_agent_id_token_auth_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig {
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig {}

impl BuildCesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig {
            service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfigRef {
        CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCerts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cert: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCerts {
    #[doc = "Set the field `cert`.\n"]
    pub fn set_cert(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCerts {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCerts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCerts {}

impl BuildCesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCerts {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCerts {
        CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCerts {
            cert: core::default::Default::default(),
            display_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCertsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCertsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCertsRef {
        CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCertsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCertsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cert` after provisioning.\n"]
    pub fn cert(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cert", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_certs: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCerts>>,
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig {
    #[doc = "Set the field `ca_certs`.\n"]
    pub fn set_ca_certs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCerts>>,
    ) -> Self {
        self.ca_certs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig {}

impl BuildCesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig {
        CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig { ca_certs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfigRef {
        CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_certs` after provisioning.\n"]
    pub fn ca_certs(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig_CaCertsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ca_certs", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_authentication: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_unknown_fields: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_api_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_OpenApiTool {
    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication>>,
    ) -> Self {
        self.api_authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_unknown_fields`.\n"]
    pub fn set_ignore_unknown_fields(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_unknown_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_api_schema`.\n"]
    pub fn set_open_api_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.open_api_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig>>,
    ) -> Self {
        self.service_directory_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig>>,
    ) -> Self {
        self.tls_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_OpenApiTool {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_OpenApiTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_OpenApiTool {}

impl BuildCesAppVersion_Snapshot_Tools_OpenApiTool {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_OpenApiTool {
        CesAppVersion_Snapshot_Tools_OpenApiTool {
            api_authentication: core::default::Default::default(),
            description: core::default::Default::default(),
            ignore_unknown_fields: core::default::Default::default(),
            name: core::default::Default::default(),
            open_api_schema: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_OpenApiToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_OpenApiToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_Tools_OpenApiToolRef {
        CesAppVersion_Snapshot_Tools_OpenApiToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_OpenApiToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_unknown_fields` after provisioning.\n"]
    pub fn ignore_unknown_fields(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_unknown_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `open_api_schema` after provisioning.\n"]
    pub fn open_api_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_api_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_PythonFunction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_PythonFunction {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_code`.\n"]
    pub fn set_python_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.python_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_PythonFunction {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_PythonFunction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_PythonFunction {}

impl BuildCesAppVersion_Snapshot_Tools_PythonFunction {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_PythonFunction {
        CesAppVersion_Snapshot_Tools_PythonFunction {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            python_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_PythonFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_PythonFunctionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Tools_PythonFunctionRef {
        CesAppVersion_Snapshot_Tools_PythonFunctionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_PythonFunctionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `python_code` after provisioning.\n"]
    pub fn python_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_code", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools_SystemTool {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools_SystemTool {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools_SystemTool {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools_SystemTool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools_SystemTool {}

impl BuildCesAppVersion_Snapshot_Tools_SystemTool {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools_SystemTool {
        CesAppVersion_Snapshot_Tools_SystemTool {
            description: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Tools_SystemToolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Tools_SystemToolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_Tools_SystemToolRef {
        CesAppVersion_Snapshot_Tools_SystemToolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Tools_SystemToolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Tools {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_function: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_ClientFunction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_tool: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generated_summary: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_search_tool: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_GoogleSearchTool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_api_tool: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    python_function: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_PythonFunction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_tool: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_SystemTool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Tools {
    #[doc = "Set the field `client_function`.\n"]
    pub fn set_client_function(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_ClientFunction>>,
    ) -> Self {
        self.client_function = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_store_tool`.\n"]
    pub fn set_data_store_tool(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_DataStoreTool>>,
    ) -> Self {
        self.data_store_tool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_type`.\n"]
    pub fn set_execution_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generated_summary`.\n"]
    pub fn set_generated_summary(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.generated_summary = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `google_search_tool`.\n"]
    pub fn set_google_search_tool(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_GoogleSearchTool>>,
    ) -> Self {
        self.google_search_tool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_api_tool`.\n"]
    pub fn set_open_api_tool(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool>>,
    ) -> Self {
        self.open_api_tool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `python_function`.\n"]
    pub fn set_python_function(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_PythonFunction>>,
    ) -> Self {
        self.python_function = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system_tool`.\n"]
    pub fn set_system_tool(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_SystemTool>>,
    ) -> Self {
        self.system_tool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Tools {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Tools>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Tools {}

impl BuildCesAppVersion_Snapshot_Tools {
    pub fn build(self) -> CesAppVersion_Snapshot_Tools {
        CesAppVersion_Snapshot_Tools {
            client_function: core::default::Default::default(),
            create_time: core::default::Default::default(),
            data_store_tool: core::default::Default::default(),
            display_name: core::default::Default::default(),
            etag: core::default::Default::default(),
            execution_type: core::default::Default::default(),
            generated_summary: core::default::Default::default(),
            google_search_tool: core::default::Default::default(),
            name: core::default::Default::default(),
            open_api_tool: core::default::Default::default(),
            python_function: core::default::Default::default(),
            system_tool: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_ToolsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_ToolsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_ToolsRef {
        CesAppVersion_Snapshot_ToolsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_ToolsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_function` after provisioning.\n"]
    pub fn client_function(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_ClientFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_function", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `data_store_tool` after provisioning.\n"]
    pub fn data_store_tool(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_DataStoreToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_store_tool", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_type` after provisioning.\n"]
    pub fn execution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_type", self.base))
    }

    #[doc = "Get a reference to the value of field `generated_summary` after provisioning.\n"]
    pub fn generated_summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generated_summary", self.base))
    }

    #[doc = "Get a reference to the value of field `google_search_tool` after provisioning.\n"]
    pub fn google_search_tool(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_GoogleSearchToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_search_tool", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `open_api_tool` after provisioning.\n"]
    pub fn open_api_tool(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_api_tool", self.base))
    }

    #[doc = "Get a reference to the value of field `python_function` after provisioning.\n"]
    pub fn python_function(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_PythonFunctionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.python_function", self.base))
    }

    #[doc = "Get a reference to the value of field `system_tool` after provisioning.\n"]
    pub fn system_tool(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_SystemToolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system_tool", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfig {
    #[doc = "Set the field `token`.\n"]
    pub fn set_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfig {
    type O =
        terrars::BlockAssignable<CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfig {}

impl BuildCesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfig {
    pub fn build(self) -> CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfig {
        CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfig {
            token: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfigRef {
        CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `token` after provisioning.\n"]
    pub fn token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bearer_token_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_auth_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_agent_id_token_auth_config: std
    ::option
    ::Option<
        terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig>,
    >,
}

impl CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication {
    #[doc = "Set the field `api_key_config`.\n"]
    pub fn set_api_key_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfig>,
                    >,
    ) -> Self {
        self.api_key_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bearer_token_config`.\n"]
    pub fn set_bearer_token_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfig,
                        >,
                    >,
    ) -> Self {
        self.bearer_token_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oauth_config`.\n"]
    pub fn set_oauth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfig>,
                    >,
    ) -> Self {
        self.oauth_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_auth_config`.\n"]
    pub fn set_service_account_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfig,
                        >,
                    >,
    ) -> Self {
        self.service_account_auth_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_agent_id_token_auth_config`.\n"]
    pub fn set_service_agent_id_token_auth_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfig,
                        >,
                    >,
    ) -> Self {
        self.service_agent_id_token_auth_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication {}

impl BuildCesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication {
    pub fn build(self) -> CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication {
        CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication {
            api_key_config: core::default::Default::default(),
            bearer_token_config: core::default::Default::default(),
            oauth_config: core::default::Default::default(),
            service_account_auth_config: core::default::Default::default(),
            service_agent_id_token_auth_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthenticationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthenticationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthenticationRef {
        CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthenticationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthenticationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key_config` after provisioning.\n"]
    pub fn api_key_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ApiKeyConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key_config", self.base))
    }

    #[doc = "Get a reference to the value of field `bearer_token_config` after provisioning.\n"]
    pub fn bearer_token_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication_BearerTokenConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bearer_token_config", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_config` after provisioning.\n"]
    pub fn oauth_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_OauthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_account_auth_config` after provisioning.\n"]
    pub fn service_account_auth_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAccountAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_auth_config", self.base))
    }

    #[doc = "Get a reference to the value of field `service_agent_id_token_auth_config` after provisioning.\n"]
    pub fn service_agent_id_token_auth_config(
        &self,
    ) -> terrars
    ::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_ApiAuthentication_ServiceAgentIdTokenAuthConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_agent_id_token_auth_config", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Toolsets_OpenApiToolset {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_authentication: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_unknown_fields: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_api_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_directory_config: std
    ::option
    ::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_config: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Toolsets_OpenApiToolset {
    #[doc = "Set the field `api_authentication`.\n"]
    pub fn set_api_authentication(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthentication>>,
    ) -> Self {
        self.api_authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_unknown_fields`.\n"]
    pub fn set_ignore_unknown_fields(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_unknown_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_api_schema`.\n"]
    pub fn set_open_api_schema(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.open_api_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_directory_config`.\n"]
    pub fn set_service_directory_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfig>>,
    ) -> Self {
        self.service_directory_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_config`.\n"]
    pub fn set_tls_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfig>>,
    ) -> Self {
        self.tls_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Toolsets_OpenApiToolset {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Toolsets_OpenApiToolset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Toolsets_OpenApiToolset {}

impl BuildCesAppVersion_Snapshot_Toolsets_OpenApiToolset {
    pub fn build(self) -> CesAppVersion_Snapshot_Toolsets_OpenApiToolset {
        CesAppVersion_Snapshot_Toolsets_OpenApiToolset {
            api_authentication: core::default::Default::default(),
            ignore_unknown_fields: core::default::Default::default(),
            open_api_schema: core::default::Default::default(),
            service_directory_config: core::default::Default::default(),
            tls_config: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_Toolsets_OpenApiToolsetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_Toolsets_OpenApiToolsetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CesAppVersion_Snapshot_Toolsets_OpenApiToolsetRef {
        CesAppVersion_Snapshot_Toolsets_OpenApiToolsetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_Toolsets_OpenApiToolsetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_authentication` after provisioning.\n"]
    pub fn api_authentication(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Toolsets_OpenApiToolset_ApiAuthenticationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_authentication", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_unknown_fields` after provisioning.\n"]
    pub fn ignore_unknown_fields(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_unknown_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `open_api_schema` after provisioning.\n"]
    pub fn open_api_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.open_api_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `service_directory_config` after provisioning.\n"]
    pub fn service_directory_config(
        &self,
    ) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_ServiceDirectoryConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_directory_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_config` after provisioning.\n"]
    pub fn tls_config(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Tools_OpenApiTool_TlsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_config", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot_Toolsets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    open_api_toolset: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Toolsets_OpenApiToolset>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Snapshot_Toolsets {
    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `etag`.\n"]
    pub fn set_etag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.etag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_type`.\n"]
    pub fn set_execution_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.execution_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `open_api_toolset`.\n"]
    pub fn set_open_api_toolset(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Toolsets_OpenApiToolset>>,
    ) -> Self {
        self.open_api_toolset = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot_Toolsets {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot_Toolsets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot_Toolsets {}

impl BuildCesAppVersion_Snapshot_Toolsets {
    pub fn build(self) -> CesAppVersion_Snapshot_Toolsets {
        CesAppVersion_Snapshot_Toolsets {
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            etag: core::default::Default::default(),
            execution_type: core::default::Default::default(),
            name: core::default::Default::default(),
            open_api_toolset: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_Snapshot_ToolsetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_Snapshot_ToolsetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_Snapshot_ToolsetsRef {
        CesAppVersion_Snapshot_ToolsetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_Snapshot_ToolsetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_type` after provisioning.\n"]
    pub fn execution_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `open_api_toolset` after provisioning.\n"]
    pub fn open_api_toolset(&self) -> terrars::ListRef<CesAppVersion_Snapshot_Toolsets_OpenApiToolsetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.open_api_toolset", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Snapshot {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    agents: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Agents>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_App>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    examples: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Examples>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guardrails: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Guardrails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tools: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Tools>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    toolsets: std::option::Option<terrars::ListField<CesAppVersion_Snapshot_Toolsets>>,
}

impl CesAppVersion_Snapshot {
    #[doc = "Set the field `agents`.\n"]
    pub fn set_agents(mut self, v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Agents>>) -> Self {
        self.agents = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `app`.\n"]
    pub fn set_app(mut self, v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_App>>) -> Self {
        self.app = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `examples`.\n"]
    pub fn set_examples(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Examples>>,
    ) -> Self {
        self.examples = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guardrails`.\n"]
    pub fn set_guardrails(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Guardrails>>,
    ) -> Self {
        self.guardrails = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tools`.\n"]
    pub fn set_tools(mut self, v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Tools>>) -> Self {
        self.tools = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `toolsets`.\n"]
    pub fn set_toolsets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CesAppVersion_Snapshot_Toolsets>>,
    ) -> Self {
        self.toolsets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CesAppVersion_Snapshot {
    type O = terrars::BlockAssignable<CesAppVersion_Snapshot>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Snapshot {}

impl BuildCesAppVersion_Snapshot {
    pub fn build(self) -> CesAppVersion_Snapshot {
        CesAppVersion_Snapshot {
            agents: core::default::Default::default(),
            app: core::default::Default::default(),
            examples: core::default::Default::default(),
            guardrails: core::default::Default::default(),
            tools: core::default::Default::default(),
            toolsets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_SnapshotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_SnapshotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_SnapshotRef {
        CesAppVersion_SnapshotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_SnapshotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agents` after provisioning.\n"]
    pub fn agents(&self) -> terrars::ListRef<CesAppVersion_Snapshot_AgentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.agents", self.base))
    }

    #[doc = "Get a reference to the value of field `app` after provisioning.\n"]
    pub fn app(&self) -> terrars::ListRef<CesAppVersion_Snapshot_AppRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app", self.base))
    }

    #[doc = "Get a reference to the value of field `examples` after provisioning.\n"]
    pub fn examples(&self) -> terrars::ListRef<CesAppVersion_Snapshot_ExamplesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.examples", self.base))
    }

    #[doc = "Get a reference to the value of field `guardrails` after provisioning.\n"]
    pub fn guardrails(&self) -> terrars::ListRef<CesAppVersion_Snapshot_GuardrailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.guardrails", self.base))
    }

    #[doc = "Get a reference to the value of field `tools` after provisioning.\n"]
    pub fn tools(&self) -> terrars::ListRef<CesAppVersion_Snapshot_ToolsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tools", self.base))
    }

    #[doc = "Get a reference to the value of field `toolsets` after provisioning.\n"]
    pub fn toolsets(&self) -> terrars::ListRef<CesAppVersion_Snapshot_ToolsetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.toolsets", self.base))
    }
}

#[derive(Serialize)]
pub struct CesAppVersion_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CesAppVersion_Timeouts {
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

impl terrars::ToListMappable for CesAppVersion_Timeouts {
    type O = terrars::BlockAssignable<CesAppVersion_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCesAppVersion_Timeouts {}

impl BuildCesAppVersion_Timeouts {
    pub fn build(self) -> CesAppVersion_Timeouts {
        CesAppVersion_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CesAppVersion_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CesAppVersion_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CesAppVersion_TimeoutsRef {
        CesAppVersion_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CesAppVersion_TimeoutsRef {
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
