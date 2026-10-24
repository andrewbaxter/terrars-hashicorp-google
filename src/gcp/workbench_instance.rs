use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct WorkbenchInstanceData {
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
    disable_proxy_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_owners: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gce_setup: std::option::Option<std::vec::Vec<WorkbenchInstanceGceSetupEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<WorkbenchInstanceTimeoutsEl>,
    dynamic: WorkbenchInstanceDynamic,
}

struct WorkbenchInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkbenchInstanceData>,
}

#[derive(Clone)]
pub struct WorkbenchInstance(std::rc::Rc<WorkbenchInstance_>);

impl WorkbenchInstance {
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
        "Set the field `disable_proxy_access`.\nOptional. If true, the workbench instance will not register with the proxy."]
    pub fn set_disable_proxy_access(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_proxy_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\nRequired. User-defined unique ID of this instance."]
    pub fn set_instance_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instance_owners`.\n'Optional. Input only. The owner of this instance after creation. Format:\n'alias@example.com' Currently supports one owner only. If not specified, all of\nthe service account users of your VM instance''s service account can use the instance.'"]
    pub fn set_instance_owners(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().instance_owners = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nOptional. Labels to apply to this instance. These can be later modified\nby the UpdateInstance method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gce_setup`.\n"]
    pub fn set_gce_setup(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstanceGceSetupEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().gce_setup = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.gce_setup = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<WorkbenchInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nAn RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.\nThe milliseconds portion (\".SSS\") is optional."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nOutput only. Email address of entity that sent original CreateInstance request."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_proxy_access` after provisioning.\nOptional. If true, the workbench instance will not register with the proxy."]
    pub fn disable_proxy_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_proxy_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_info` after provisioning.\n'Output only. Additional information about instance health. Example:\nhealthInfo\": { \"docker_proxy_agent_status\": \"1\", \"docker_status\": \"1\", \"jupyterlab_api_status\":\n\"-1\", \"jupyterlab_status\": \"-1\", \"updated\": \"2020-10-18 09:40:03.573409\" }'"]
    pub fn health_info(&self) -> terrars::ListRef<WorkbenchInstanceHealthInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_state` after provisioning.\nOutput only. Instance health_state."]
    pub fn health_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nRequired. User-defined unique ID of this instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_owners` after provisioning.\n'Optional. Input only. The owner of this instance after creation. Format:\n'alias@example.com' Currently supports one owner only. If not specified, all of\nthe service account users of your VM instance''s service account can use the instance.'"]
    pub fn instance_owners(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_owners", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to apply to this instance. These can be later modified\nby the UpdateInstance method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this workbench instance. Format: 'projects/{project_id}/locations/{location}/instances/{instance_id}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxy_uri` after provisioning.\nOutput only. The proxy endpoint that is used to access the Jupyter notebook."]
    pub fn proxy_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of this instance."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nAn RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.\nThe milliseconds portion (\".SSS\") is optional."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upgrade_history` after provisioning.\nOutput only. The upgrade history of this instance."]
    pub fn upgrade_history(&self) -> terrars::ListRef<WorkbenchInstanceUpgradeHistoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_history", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gce_setup` after provisioning.\n"]
    pub fn gce_setup(&self) -> terrars::ListRef<WorkbenchInstanceGceSetupElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_setup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkbenchInstanceTimeoutsElRef {
        WorkbenchInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for WorkbenchInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WorkbenchInstance { }

impl terrars::ToListMappable for WorkbenchInstance {
    type O = terrars::ListRef<WorkbenchInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WorkbenchInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_workbench_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWorkbenchInstance {
    pub tf_id: std::string::String,
    #[doc = "Part of 'parent'. See documentation of 'projectsId'."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of this workbench instance. Format: 'projects/{project_id}/locations/{location}/instances/{instance_id}'"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWorkbenchInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> WorkbenchInstance {
        let out = WorkbenchInstance(std::rc::Rc::new(WorkbenchInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WorkbenchInstanceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                disable_proxy_access: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: core::default::Default::default(),
                instance_owners: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                gce_setup: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WorkbenchInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WorkbenchInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nAn RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.\nThe milliseconds portion (\".SSS\") is optional."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nOutput only. Email address of entity that sent original CreateInstance request."]
    pub fn creator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_proxy_access` after provisioning.\nOptional. If true, the workbench instance will not register with the proxy."]
    pub fn disable_proxy_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_proxy_access", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_info` after provisioning.\n'Output only. Additional information about instance health. Example:\nhealthInfo\": { \"docker_proxy_agent_status\": \"1\", \"docker_status\": \"1\", \"jupyterlab_api_status\":\n\"-1\", \"jupyterlab_status\": \"-1\", \"updated\": \"2020-10-18 09:40:03.573409\" }'"]
    pub fn health_info(&self) -> terrars::ListRef<WorkbenchInstanceHealthInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_info", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_state` after provisioning.\nOutput only. Instance health_state."]
    pub fn health_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_id` after provisioning.\nRequired. User-defined unique ID of this instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance_owners` after provisioning.\n'Optional. Input only. The owner of this instance after creation. Format:\n'alias@example.com' Currently supports one owner only. If not specified, all of\nthe service account users of your VM instance''s service account can use the instance.'"]
    pub fn instance_owners(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_owners", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nOptional. Labels to apply to this instance. These can be later modified\nby the UpdateInstance method.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this workbench instance. Format: 'projects/{project_id}/locations/{location}/instances/{instance_id}'"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `proxy_uri` after provisioning.\nOutput only. The proxy endpoint that is used to access the Jupyter notebook."]
    pub fn proxy_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.proxy_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The state of this instance."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nAn RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.\nThe milliseconds portion (\".SSS\") is optional."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upgrade_history` after provisioning.\nOutput only. The upgrade history of this instance."]
    pub fn upgrade_history(&self) -> terrars::ListRef<WorkbenchInstanceUpgradeHistoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_history", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gce_setup` after provisioning.\n"]
    pub fn gce_setup(&self) -> terrars::ListRef<WorkbenchInstanceGceSetupElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_setup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkbenchInstanceTimeoutsElRef {
        WorkbenchInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstanceHealthInfoEl {}

impl WorkbenchInstanceHealthInfoEl { }

impl terrars::ToListMappable for WorkbenchInstanceHealthInfoEl {
    type O = terrars::BlockAssignable<WorkbenchInstanceHealthInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstanceHealthInfoEl {}

impl BuildWorkbenchInstanceHealthInfoEl {
    pub fn build(self) -> WorkbenchInstanceHealthInfoEl {
        WorkbenchInstanceHealthInfoEl {}
    }
}

pub struct WorkbenchInstanceHealthInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstanceHealthInfoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstanceHealthInfoElRef {
        WorkbenchInstanceHealthInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstanceHealthInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstanceUpgradeHistoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    framework: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_image: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstanceUpgradeHistoryEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_image`.\n"]
    pub fn set_container_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `framework`.\n"]
    pub fn set_framework(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.framework = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot`.\n"]
    pub fn set_snapshot(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_version`.\n"]
    pub fn set_target_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vm_image`.\n"]
    pub fn set_vm_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vm_image = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstanceUpgradeHistoryEl {
    type O = terrars::BlockAssignable<WorkbenchInstanceUpgradeHistoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstanceUpgradeHistoryEl {}

impl BuildWorkbenchInstanceUpgradeHistoryEl {
    pub fn build(self) -> WorkbenchInstanceUpgradeHistoryEl {
        WorkbenchInstanceUpgradeHistoryEl {
            action: core::default::Default::default(),
            container_image: core::default::Default::default(),
            create_time: core::default::Default::default(),
            framework: core::default::Default::default(),
            snapshot: core::default::Default::default(),
            state: core::default::Default::default(),
            target_version: core::default::Default::default(),
            version: core::default::Default::default(),
            vm_image: core::default::Default::default(),
        }
    }
}

pub struct WorkbenchInstanceUpgradeHistoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstanceUpgradeHistoryElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstanceUpgradeHistoryElRef {
        WorkbenchInstanceUpgradeHistoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstanceUpgradeHistoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_image", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `framework` after provisioning.\n"]
    pub fn framework(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.framework", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot` after provisioning.\n"]
    pub fn snapshot(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `target_version` after provisioning.\n"]
    pub fn target_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_version", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_image` after provisioning.\n"]
    pub fn vm_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vm_image", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElAcceleratorConfigsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstanceGceSetupElAcceleratorConfigsEl {
    #[doc = "Set the field `core_count`.\nOptional. Count of cores of this accelerator."]
    pub fn set_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nOptional. Type of this accelerator. Possible values: [\"NVIDIA_TESLA_P100\", \"NVIDIA_TESLA_V100\", \"NVIDIA_TESLA_P4\", \"NVIDIA_TESLA_T4\", \"NVIDIA_TESLA_A100\", \"NVIDIA_A100_80GB\", \"NVIDIA_L4\", \"NVIDIA_TESLA_T4_VWS\", \"NVIDIA_TESLA_P100_VWS\", \"NVIDIA_TESLA_P4_VWS\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstanceGceSetupElAcceleratorConfigsEl {
    type O = terrars::BlockAssignable<WorkbenchInstanceGceSetupElAcceleratorConfigsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstanceGceSetupElAcceleratorConfigsEl {}

impl BuildWorkbenchInstanceGceSetupElAcceleratorConfigsEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElAcceleratorConfigsEl {
        WorkbenchInstanceGceSetupElAcceleratorConfigsEl {
            core_count: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct WorkbenchInstanceGceSetupElAcceleratorConfigsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstanceGceSetupElAcceleratorConfigsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkbenchInstanceGceSetupElAcceleratorConfigsElRef {
        WorkbenchInstanceGceSetupElAcceleratorConfigsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstanceGceSetupElAcceleratorConfigsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `core_count` after provisioning.\nOptional. Count of cores of this accelerator."]
    pub fn core_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOptional. Type of this accelerator. Possible values: [\"NVIDIA_TESLA_P100\", \"NVIDIA_TESLA_V100\", \"NVIDIA_TESLA_P4\", \"NVIDIA_TESLA_T4\", \"NVIDIA_TESLA_A100\", \"NVIDIA_A100_80GB\", \"NVIDIA_L4\", \"NVIDIA_TESLA_T4_VWS\", \"NVIDIA_TESLA_P100_VWS\", \"NVIDIA_TESLA_P4_VWS\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElBootDiskEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstanceGceSetupElBootDiskEl {
    #[doc =
        "Set the field `disk_encryption`.\nOptional. Input only. Disk encryption method used on the boot and\ndata disks, defaults to GMEK. Possible values: [\"GMEK\", \"CMEK\"]"]
    pub fn set_disk_encryption(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_size_gb`.\nOptional. The size of the boot disk in GB attached to this instance,\nup to a maximum of 64000 GB (64 TB). If not specified, this defaults to the\nrecommended value of 150GB."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_type`.\nOptional. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\"]"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key`.\n'Optional. Input only. The KMS key used to encrypt the disks, only\napplicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstanceGceSetupElBootDiskEl {
    type O = terrars::BlockAssignable<WorkbenchInstanceGceSetupElBootDiskEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstanceGceSetupElBootDiskEl {}

impl BuildWorkbenchInstanceGceSetupElBootDiskEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElBootDiskEl {
        WorkbenchInstanceGceSetupElBootDiskEl {
            disk_encryption: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}

pub struct WorkbenchInstanceGceSetupElBootDiskElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstanceGceSetupElBootDiskElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstanceGceSetupElBootDiskElRef {
        WorkbenchInstanceGceSetupElBootDiskElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstanceGceSetupElBootDiskElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_encryption` after provisioning.\nOptional. Input only. Disk encryption method used on the boot and\ndata disks, defaults to GMEK. Possible values: [\"GMEK\", \"CMEK\"]"]
    pub fn disk_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nOptional. The size of the boot disk in GB attached to this instance,\nup to a maximum of 64000 GB (64 TB). If not specified, this defaults to the\nrecommended value of 150GB."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nOptional. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\"]"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\n'Optional. Input only. The KMS key used to encrypt the disks, only\napplicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElDataDisksEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstanceGceSetupElDataDisksEl {
    #[doc =
        "Set the field `disk_encryption`.\nOptional. Input only. Disk encryption method used on the boot\nand data disks, defaults to GMEK. Possible values: [\"GMEK\", \"CMEK\"]"]
    pub fn set_disk_encryption(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_size_gb`.\nOptional. The size of the disk in GB attached to this VM instance,\nup to a maximum of 64000 GB (64 TB). If not specified, this defaults to\n100."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disk_type`.\nOptional. Input only. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\"]"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key`.\n'Optional. Input only. The KMS key used to encrypt the disks,\nonly applicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstanceGceSetupElDataDisksEl {
    type O = terrars::BlockAssignable<WorkbenchInstanceGceSetupElDataDisksEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstanceGceSetupElDataDisksEl {}

impl BuildWorkbenchInstanceGceSetupElDataDisksEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElDataDisksEl {
        WorkbenchInstanceGceSetupElDataDisksEl {
            disk_encryption: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}

pub struct WorkbenchInstanceGceSetupElDataDisksElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstanceGceSetupElDataDisksElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstanceGceSetupElDataDisksElRef {
        WorkbenchInstanceGceSetupElDataDisksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstanceGceSetupElDataDisksElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_encryption` after provisioning.\nOptional. Input only. Disk encryption method used on the boot\nand data disks, defaults to GMEK. Possible values: [\"GMEK\", \"CMEK\"]"]
    pub fn disk_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_encryption", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nOptional. The size of the disk in GB attached to this VM instance,\nup to a maximum of 64000 GB (64 TB). If not specified, this defaults to\n100."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disk_type` after provisioning.\nOptional. Input only. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\"]"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\n'Optional. Input only. The KMS key used to encrypt the disks,\nonly applicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElNetworkInterfacesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nic_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstanceGceSetupElNetworkInterfacesEl {
    #[doc = "Set the field `network`.\nOptional. The name of the VPC that this VM instance is in."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `nic_type`.\nOptional. The type of vNIC to be used on this interface. This\nmay be gVNIC or VirtioNet. Possible values: [\"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn set_nic_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nic_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\nOptional. The name of the subnet that this VM instance is in."]
    pub fn set_subnet(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstanceGceSetupElNetworkInterfacesEl {
    type O = terrars::BlockAssignable<WorkbenchInstanceGceSetupElNetworkInterfacesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstanceGceSetupElNetworkInterfacesEl {}

impl BuildWorkbenchInstanceGceSetupElNetworkInterfacesEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElNetworkInterfacesEl {
        WorkbenchInstanceGceSetupElNetworkInterfacesEl {
            network: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            subnet: core::default::Default::default(),
        }
    }
}

pub struct WorkbenchInstanceGceSetupElNetworkInterfacesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstanceGceSetupElNetworkInterfacesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkbenchInstanceGceSetupElNetworkInterfacesElRef {
        WorkbenchInstanceGceSetupElNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstanceGceSetupElNetworkInterfacesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nOptional. The name of the VPC that this VM instance is in."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `nic_type` after provisioning.\nOptional. The type of vNIC to be used on this interface. This\nmay be gVNIC or VirtioNet. Possible values: [\"VIRTIO_NET\", \"GVNIC\"]"]
    pub fn nic_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nic_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnet` after provisioning.\nOptional. The name of the subnet that this VM instance is in."]
    pub fn subnet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElServiceAccountsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstanceGceSetupElServiceAccountsEl {
    #[doc = "Set the field `email`.\nOptional. Email address of the service account."]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstanceGceSetupElServiceAccountsEl {
    type O = terrars::BlockAssignable<WorkbenchInstanceGceSetupElServiceAccountsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstanceGceSetupElServiceAccountsEl {}

impl BuildWorkbenchInstanceGceSetupElServiceAccountsEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElServiceAccountsEl {
        WorkbenchInstanceGceSetupElServiceAccountsEl { email: core::default::Default::default() }
    }
}

pub struct WorkbenchInstanceGceSetupElServiceAccountsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstanceGceSetupElServiceAccountsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkbenchInstanceGceSetupElServiceAccountsElRef {
        WorkbenchInstanceGceSetupElServiceAccountsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstanceGceSetupElServiceAccountsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nOptional. Email address of the service account."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `scopes` after provisioning.\nOutput only. The list of scopes to be made available for this\nservice account. Set by the CLH to https://www.googleapis.com/auth/cloud-platform"]
    pub fn scopes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scopes", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupElVmImageEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    family: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstanceGceSetupElVmImageEl {
    #[doc =
        "Set the field `family`.\nOptional. Use this VM image family to find the image; the newest\nimage in this family will be used."]
    pub fn set_family(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.family = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nOptional. Use VM image name to find the image."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe name of the Google Cloud project that this VM image belongs to.\nFormat: {project_id}"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstanceGceSetupElVmImageEl {
    type O = terrars::BlockAssignable<WorkbenchInstanceGceSetupElVmImageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstanceGceSetupElVmImageEl {}

impl BuildWorkbenchInstanceGceSetupElVmImageEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupElVmImageEl {
        WorkbenchInstanceGceSetupElVmImageEl {
            family: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

pub struct WorkbenchInstanceGceSetupElVmImageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstanceGceSetupElVmImageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstanceGceSetupElVmImageElRef {
        WorkbenchInstanceGceSetupElVmImageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstanceGceSetupElVmImageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `family` after provisioning.\nOptional. Use this VM image family to find the image; the newest\nimage in this family will be used."]
    pub fn family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOptional. Use VM image name to find the image."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe name of the Google Cloud project that this VM image belongs to.\nFormat: {project_id}"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkbenchInstanceGceSetupElDynamic {
    accelerator_configs: std::option::Option<terrars::DynamicBlock<WorkbenchInstanceGceSetupElAcceleratorConfigsEl>>,
    boot_disk: std::option::Option<terrars::DynamicBlock<WorkbenchInstanceGceSetupElBootDiskEl>>,
    data_disks: std::option::Option<terrars::DynamicBlock<WorkbenchInstanceGceSetupElDataDisksEl>>,
    network_interfaces: std::option::Option<terrars::DynamicBlock<WorkbenchInstanceGceSetupElNetworkInterfacesEl>>,
    service_accounts: std::option::Option<terrars::DynamicBlock<WorkbenchInstanceGceSetupElServiceAccountsEl>>,
    vm_image: std::option::Option<terrars::DynamicBlock<WorkbenchInstanceGceSetupElVmImageEl>>,
}

#[derive(Serialize)]
pub struct WorkbenchInstanceGceSetupEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ip_forwarding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_configs: std::option::Option<std::vec::Vec<WorkbenchInstanceGceSetupElAcceleratorConfigsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk: std::option::Option<std::vec::Vec<WorkbenchInstanceGceSetupElBootDiskEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_disks: std::option::Option<std::vec::Vec<WorkbenchInstanceGceSetupElDataDisksEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interfaces: std::option::Option<std::vec::Vec<WorkbenchInstanceGceSetupElNetworkInterfacesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_accounts: std::option::Option<std::vec::Vec<WorkbenchInstanceGceSetupElServiceAccountsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_image: std::option::Option<std::vec::Vec<WorkbenchInstanceGceSetupElVmImageEl>>,
    dynamic: WorkbenchInstanceGceSetupElDynamic,
}

impl WorkbenchInstanceGceSetupEl {
    #[doc =
        "Set the field `disable_public_ip`.\nOptional. If true, no external IP will be assigned to this VM instance."]
    pub fn set_disable_public_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_public_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_ip_forwarding`.\nOptional. Flag to enable ip forwarding or not, default false/off.\nhttps://cloud.google.com/vpc/docs/using-routes#canipforward"]
    pub fn set_enable_ip_forwarding(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_ip_forwarding = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `machine_type`.\nOptional. The machine type of the VM instance. https://cloud.google.com/compute/docs/machine-resource"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\nOptional. Custom metadata to apply to this instance."]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tags`.\nOptional. The Compute Engine tags to add to instance (see [Tagging\ninstances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags))."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_configs`.\n"]
    pub fn set_accelerator_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstanceGceSetupElAcceleratorConfigsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerator_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `boot_disk`.\n"]
    pub fn set_boot_disk(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstanceGceSetupElBootDiskEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boot_disk = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boot_disk = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_disks`.\n"]
    pub fn set_data_disks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstanceGceSetupElDataDisksEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_disks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_disks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_interfaces`.\n"]
    pub fn set_network_interfaces(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstanceGceSetupElNetworkInterfacesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_interfaces = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interfaces = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_accounts`.\n"]
    pub fn set_service_accounts(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstanceGceSetupElServiceAccountsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_accounts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_accounts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vm_image`.\n"]
    pub fn set_vm_image(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstanceGceSetupElVmImageEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vm_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vm_image = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstanceGceSetupEl {
    type O = terrars::BlockAssignable<WorkbenchInstanceGceSetupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstanceGceSetupEl {}

impl BuildWorkbenchInstanceGceSetupEl {
    pub fn build(self) -> WorkbenchInstanceGceSetupEl {
        WorkbenchInstanceGceSetupEl {
            disable_public_ip: core::default::Default::default(),
            enable_ip_forwarding: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            metadata: core::default::Default::default(),
            tags: core::default::Default::default(),
            accelerator_configs: core::default::Default::default(),
            boot_disk: core::default::Default::default(),
            data_disks: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
            service_accounts: core::default::Default::default(),
            vm_image: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct WorkbenchInstanceGceSetupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstanceGceSetupElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstanceGceSetupElRef {
        WorkbenchInstanceGceSetupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstanceGceSetupElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_public_ip` after provisioning.\nOptional. If true, no external IP will be assigned to this VM instance."]
    pub fn disable_public_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_public_ip", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_ip_forwarding` after provisioning.\nOptional. Flag to enable ip forwarding or not, default false/off.\nhttps://cloud.google.com/vpc/docs/using-routes#canipforward"]
    pub fn enable_ip_forwarding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ip_forwarding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nOptional. The machine type of the VM instance. https://cloud.google.com/compute/docs/machine-resource"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nOptional. Custom metadata to apply to this instance."]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nOptional. The Compute Engine tags to add to instance (see [Tagging\ninstances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags))."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_configs` after provisioning.\n"]
    pub fn accelerator_configs(&self) -> terrars::ListRef<WorkbenchInstanceGceSetupElAcceleratorConfigsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> terrars::ListRef<WorkbenchInstanceGceSetupElBootDiskElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }

    #[doc = "Get a reference to the value of field `data_disks` after provisioning.\n"]
    pub fn data_disks(&self) -> terrars::ListRef<WorkbenchInstanceGceSetupElDataDisksElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_disks", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(&self) -> terrars::ListRef<WorkbenchInstanceGceSetupElNetworkInterfacesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.base))
    }

    #[doc = "Get a reference to the value of field `service_accounts` after provisioning.\n"]
    pub fn service_accounts(&self) -> terrars::ListRef<WorkbenchInstanceGceSetupElServiceAccountsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_image` after provisioning.\n"]
    pub fn vm_image(&self) -> terrars::ListRef<WorkbenchInstanceGceSetupElVmImageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_image", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstanceTimeoutsEl {
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

impl terrars::ToListMappable for WorkbenchInstanceTimeoutsEl {
    type O = terrars::BlockAssignable<WorkbenchInstanceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstanceTimeoutsEl {}

impl BuildWorkbenchInstanceTimeoutsEl {
    pub fn build(self) -> WorkbenchInstanceTimeoutsEl {
        WorkbenchInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct WorkbenchInstanceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstanceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstanceTimeoutsElRef {
        WorkbenchInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstanceTimeoutsElRef {
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
struct WorkbenchInstanceDynamic {
    gce_setup: std::option::Option<terrars::DynamicBlock<WorkbenchInstanceGceSetupEl>>,
}
