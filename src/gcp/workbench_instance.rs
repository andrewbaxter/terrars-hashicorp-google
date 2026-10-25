#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct WorkbenchInstanceData_ {
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
    desired_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_proxy_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_managed_euc: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_third_party_identity: std::option::Option<terrars::PrimField<bool>>,
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
    gce_setup: std::option::Option<std::vec::Vec<WorkbenchInstance_GceSetup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<WorkbenchInstance_Timeouts>,
    dynamic: WorkbenchInstanceDynamic,
}

struct WorkbenchInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WorkbenchInstanceData_>,
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `desired_state`.\nDesired state of the Workbench Instance. Set this field to 'ACTIVE' to start the Instance, and 'STOPPED' to stop the Instance."]
    pub fn set_desired_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().desired_state = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_proxy_access`.\nOptional. If true, the workbench instance will not register with the proxy."]
    pub fn set_disable_proxy_access(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_proxy_access = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_deletion_protection`.\nOptional. If true, deletion protection will be enabled for this Workbench Instance."]
    pub fn set_enable_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_managed_euc`.\nFlag to enable managed end user credentials for the instance."]
    pub fn set_enable_managed_euc(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_managed_euc = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_third_party_identity`.\nFlag that specifies that a notebook can be accessed with third party\nidentity provider."]
    pub fn set_enable_third_party_identity(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_third_party_identity = std::option::Option::Some(v.into());
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
        "Set the field `instance_owners`.\n'Optional. Input only. The owner of this instance after creation. Format:\n'alias@example.com' Currently supports one owner only. If not specified, all of\nthe service account users of your VM instance''s service account can use the instance.\nIf specified, sets the access mode to 'Single user'. For more details, see\nhttps://cloud.google.com/vertex-ai/docs/workbench/instances/manage-access-jupyterlab'"]
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
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<WorkbenchInstance_Timeouts>) -> Self {
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Workbench Instance. Set this field to 'ACTIVE' to start the Instance, and 'STOPPED' to stop the Instance."]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
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
        "Get a reference to the value of field `enable_deletion_protection` after provisioning.\nOptional. If true, deletion protection will be enabled for this Workbench Instance."]
    pub fn enable_deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_managed_euc` after provisioning.\nFlag to enable managed end user credentials for the instance."]
    pub fn enable_managed_euc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_managed_euc", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_third_party_identity` after provisioning.\nFlag that specifies that a notebook can be accessed with third party\nidentity provider."]
    pub fn enable_third_party_identity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_third_party_identity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_info` after provisioning.\n'Output only. Additional information about instance health. Example:\nhealthInfo\": { \"docker_proxy_agent_status\": \"1\", \"docker_status\": \"1\", \"jupyterlab_api_status\":\n\"-1\", \"jupyterlab_status\": \"-1\", \"updated\": \"2020-10-18 09:40:03.573409\" }'"]
    pub fn health_info(&self) -> terrars::ListRef<WorkbenchInstance_HealthInfoRef> {
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
        "Get a reference to the value of field `instance_owners` after provisioning.\n'Optional. Input only. The owner of this instance after creation. Format:\n'alias@example.com' Currently supports one owner only. If not specified, all of\nthe service account users of your VM instance''s service account can use the instance.\nIf specified, sets the access mode to 'Single user'. For more details, see\nhttps://cloud.google.com/vertex-ai/docs/workbench/instances/manage-access-jupyterlab'"]
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
    pub fn upgrade_history(&self) -> terrars::ListRef<WorkbenchInstance_UpgradeHistoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_history", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gce_setup` after provisioning.\n"]
    pub fn gce_setup(&self) -> terrars::ListRef<WorkbenchInstance_GceSetupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_setup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkbenchInstance_TimeoutsRef {
        WorkbenchInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(WorkbenchInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                desired_state: core::default::Default::default(),
                disable_proxy_access: core::default::Default::default(),
                enable_deletion_protection: core::default::Default::default(),
                enable_managed_euc: core::default::Default::default(),
                enable_third_party_identity: core::default::Default::default(),
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
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `desired_state` after provisioning.\nDesired state of the Workbench Instance. Set this field to 'ACTIVE' to start the Instance, and 'STOPPED' to stop the Instance."]
    pub fn desired_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_state", self.extract_ref()))
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
        "Get a reference to the value of field `enable_deletion_protection` after provisioning.\nOptional. If true, deletion protection will be enabled for this Workbench Instance."]
    pub fn enable_deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_managed_euc` after provisioning.\nFlag to enable managed end user credentials for the instance."]
    pub fn enable_managed_euc(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_managed_euc", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_third_party_identity` after provisioning.\nFlag that specifies that a notebook can be accessed with third party\nidentity provider."]
    pub fn enable_third_party_identity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_third_party_identity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `health_info` after provisioning.\n'Output only. Additional information about instance health. Example:\nhealthInfo\": { \"docker_proxy_agent_status\": \"1\", \"docker_status\": \"1\", \"jupyterlab_api_status\":\n\"-1\", \"jupyterlab_status\": \"-1\", \"updated\": \"2020-10-18 09:40:03.573409\" }'"]
    pub fn health_info(&self) -> terrars::ListRef<WorkbenchInstance_HealthInfoRef> {
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
        "Get a reference to the value of field `instance_owners` after provisioning.\n'Optional. Input only. The owner of this instance after creation. Format:\n'alias@example.com' Currently supports one owner only. If not specified, all of\nthe service account users of your VM instance''s service account can use the instance.\nIf specified, sets the access mode to 'Single user'. For more details, see\nhttps://cloud.google.com/vertex-ai/docs/workbench/instances/manage-access-jupyterlab'"]
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
    pub fn upgrade_history(&self) -> terrars::ListRef<WorkbenchInstance_UpgradeHistoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade_history", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gce_setup` after provisioning.\n"]
    pub fn gce_setup(&self) -> terrars::ListRef<WorkbenchInstance_GceSetupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gce_setup", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> WorkbenchInstance_TimeoutsRef {
        WorkbenchInstance_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct WorkbenchInstanceDynamic {
    gce_setup: std::option::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup>>,
}

#[derive(Serialize)]
pub struct WorkbenchInstance_HealthInfo {}

impl WorkbenchInstance_HealthInfo { }

impl terrars::ToListMappable for WorkbenchInstance_HealthInfo {
    type O = terrars::BlockAssignable<WorkbenchInstance_HealthInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_HealthInfo {}

impl BuildWorkbenchInstance_HealthInfo {
    pub fn build(self) -> WorkbenchInstance_HealthInfo {
        WorkbenchInstance_HealthInfo {}
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_HealthInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_HealthInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstance_HealthInfoRef {
        WorkbenchInstance_HealthInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_HealthInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstance_UpgradeHistory {
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

impl WorkbenchInstance_UpgradeHistory {
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

impl terrars::ToListMappable for WorkbenchInstance_UpgradeHistory {
    type O = terrars::BlockAssignable<WorkbenchInstance_UpgradeHistory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_UpgradeHistory {}

impl BuildWorkbenchInstance_UpgradeHistory {
    pub fn build(self) -> WorkbenchInstance_UpgradeHistory {
        WorkbenchInstance_UpgradeHistory {
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

#[allow(dead_code)]
pub struct WorkbenchInstance_UpgradeHistoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_UpgradeHistoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstance_UpgradeHistoryRef {
        WorkbenchInstance_UpgradeHistoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_UpgradeHistoryRef {
    #[allow(dead_code)]
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
pub struct WorkbenchInstance_GceSetup_AcceleratorConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_count: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstance_GceSetup_AcceleratorConfigs {
    #[doc = "Set the field `core_count`.\nOptional. Count of cores of this accelerator."]
    pub fn set_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nOptional. Type of this accelerator. Possible values: [\"NVIDIA_TESLA_P100\", \"NVIDIA_TESLA_V100\", \"NVIDIA_TESLA_P4\", \"NVIDIA_TESLA_T4\", \"NVIDIA_TESLA_A100\", \"NVIDIA_A100_80GB\", \"NVIDIA_L4\", \"NVIDIA_H100_80GB\", \"NVIDIA_H100_MEGA_80GB\", \"NVIDIA_H200_141GB\", \"NVIDIA_B200\", \"NVIDIA_RTX6000\", \"NVIDIA_TESLA_T4_VWS\", \"NVIDIA_TESLA_P100_VWS\", \"NVIDIA_TESLA_P4_VWS\"]"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstance_GceSetup_AcceleratorConfigs {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup_AcceleratorConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup_AcceleratorConfigs {}

impl BuildWorkbenchInstance_GceSetup_AcceleratorConfigs {
    pub fn build(self) -> WorkbenchInstance_GceSetup_AcceleratorConfigs {
        WorkbenchInstance_GceSetup_AcceleratorConfigs {
            core_count: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetup_AcceleratorConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetup_AcceleratorConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkbenchInstance_GceSetup_AcceleratorConfigsRef {
        WorkbenchInstance_GceSetup_AcceleratorConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetup_AcceleratorConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `core_count` after provisioning.\nOptional. Count of cores of this accelerator."]
    pub fn core_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nOptional. Type of this accelerator. Possible values: [\"NVIDIA_TESLA_P100\", \"NVIDIA_TESLA_V100\", \"NVIDIA_TESLA_P4\", \"NVIDIA_TESLA_T4\", \"NVIDIA_TESLA_A100\", \"NVIDIA_A100_80GB\", \"NVIDIA_L4\", \"NVIDIA_H100_80GB\", \"NVIDIA_H100_MEGA_80GB\", \"NVIDIA_H200_141GB\", \"NVIDIA_B200\", \"NVIDIA_RTX6000\", \"NVIDIA_TESLA_T4_VWS\", \"NVIDIA_TESLA_P100_VWS\", \"NVIDIA_TESLA_P4_VWS\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstance_GceSetup_BootDisk {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstance_GceSetup_BootDisk {
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
        "Set the field `disk_type`.\nOptional. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\", \"HYPERDISK_BALANCED\", \"HYPERDISK_BALANCED_HIGH_AVAILABILITY\", \"HYPERDISK_ML\"]"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key`.\n'Optional. The KMS key used to encrypt the disks, only\napplicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstance_GceSetup_BootDisk {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup_BootDisk>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup_BootDisk {}

impl BuildWorkbenchInstance_GceSetup_BootDisk {
    pub fn build(self) -> WorkbenchInstance_GceSetup_BootDisk {
        WorkbenchInstance_GceSetup_BootDisk {
            disk_encryption: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetup_BootDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetup_BootDiskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstance_GceSetup_BootDiskRef {
        WorkbenchInstance_GceSetup_BootDiskRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetup_BootDiskRef {
    #[allow(dead_code)]
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
        "Get a reference to the value of field `disk_type` after provisioning.\nOptional. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\", \"HYPERDISK_BALANCED\", \"HYPERDISK_BALANCED_HIGH_AVAILABILITY\", \"HYPERDISK_ML\"]"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\n'Optional. The KMS key used to encrypt the disks, only\napplicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstance_GceSetup_ConfidentialInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstance_GceSetup_ConfidentialInstanceConfig {
    #[doc =
        "Set the field `confidential_instance_type`.\nDefines the type of technology used by the confidential instance. Possible values: [\"SEV\"]"]
    pub fn set_confidential_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.confidential_instance_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstance_GceSetup_ConfidentialInstanceConfig {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup_ConfidentialInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup_ConfidentialInstanceConfig {}

impl BuildWorkbenchInstance_GceSetup_ConfidentialInstanceConfig {
    pub fn build(self) -> WorkbenchInstance_GceSetup_ConfidentialInstanceConfig {
        WorkbenchInstance_GceSetup_ConfidentialInstanceConfig {
            confidential_instance_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetup_ConfidentialInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetup_ConfidentialInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkbenchInstance_GceSetup_ConfidentialInstanceConfigRef {
        WorkbenchInstance_GceSetup_ConfidentialInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetup_ConfidentialInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `confidential_instance_type` after provisioning.\nDefines the type of technology used by the confidential instance. Possible values: [\"SEV\"]"]
    pub fn confidential_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidential_instance_type", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstance_GceSetup_ContainerImage {
    repository: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstance_GceSetup_ContainerImage {
    #[doc = "Set the field `tag`.\nThe tag of the container image. If not specified, this defaults to the latest tag."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstance_GceSetup_ContainerImage {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup_ContainerImage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup_ContainerImage {
    #[doc = "The path to the container image repository.\nFor example: gcr.io/{project_id}/{imageName}"]
    pub repository: terrars::PrimField<std::string::String>,
}

impl BuildWorkbenchInstance_GceSetup_ContainerImage {
    pub fn build(self) -> WorkbenchInstance_GceSetup_ContainerImage {
        WorkbenchInstance_GceSetup_ContainerImage {
            repository: self.repository,
            tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetup_ContainerImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetup_ContainerImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstance_GceSetup_ContainerImageRef {
        WorkbenchInstance_GceSetup_ContainerImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetup_ContainerImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `repository` after provisioning.\nThe path to the container image repository.\nFor example: gcr.io/{project_id}/{imageName}"]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nThe tag of the container image. If not specified, this defaults to the latest tag."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstance_GceSetup_DataDisks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_policies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl WorkbenchInstance_GceSetup_DataDisks {
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
        "Set the field `disk_type`.\nOptional. Input only. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\", \"HYPERDISK_BALANCED\", \"HYPERDISK_EXTREME\", \"HYPERDISK_THROUGHPUT\", \"HYPERDISK_BALANCED_HIGH_AVAILABILITY\", \"HYPERDISK_ML\"]"]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key`.\n'Optional. The KMS key used to encrypt the disks,\nonly applicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn set_kms_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_policies`.\nOptional. Resource policies applied to this disk."]
    pub fn set_resource_policies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_policies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstance_GceSetup_DataDisks {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup_DataDisks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup_DataDisks {}

impl BuildWorkbenchInstance_GceSetup_DataDisks {
    pub fn build(self) -> WorkbenchInstance_GceSetup_DataDisks {
        WorkbenchInstance_GceSetup_DataDisks {
            disk_encryption: core::default::Default::default(),
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
            kms_key: core::default::Default::default(),
            resource_policies: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetup_DataDisksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetup_DataDisksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstance_GceSetup_DataDisksRef {
        WorkbenchInstance_GceSetup_DataDisksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetup_DataDisksRef {
    #[allow(dead_code)]
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
        "Get a reference to the value of field `disk_type` after provisioning.\nOptional. Input only. Indicates the type of the disk. Possible values: [\"PD_STANDARD\", \"PD_SSD\", \"PD_BALANCED\", \"PD_EXTREME\", \"HYPERDISK_BALANCED\", \"HYPERDISK_EXTREME\", \"HYPERDISK_THROUGHPUT\", \"HYPERDISK_BALANCED_HIGH_AVAILABILITY\", \"HYPERDISK_ML\"]"]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\n'Optional. The KMS key used to encrypt the disks,\nonly applicable if disk_encryption is CMEK. Format: 'projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}'\nLearn more about using your own encryption keys.'"]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_policies` after provisioning.\nOptional. Resource policies applied to this disk."]
    pub fn resource_policies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_policies", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigs {
    external_ip: terrars::PrimField<std::string::String>,
}

impl WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigs { }

impl terrars::ToListMappable for WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigs {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigs {
    #[doc =
        "An external IP address associated with this instance. Specify an unused\nstatic external IP address available to the project or leave this field\nundefined to use an IP from a shared ephemeral IP address pool. If you\nspecify a static external IP address, it must live in the same region as\nthe zone of the instance."]
    pub external_ip: terrars::PrimField<std::string::String>,
}

impl BuildWorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigs {
    pub fn build(self) -> WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigs {
        WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigs { external_ip: self.external_ip }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigsRef {
        WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `external_ip` after provisioning.\nAn external IP address associated with this instance. Specify an unused\nstatic external IP address available to the project or leave this field\nundefined to use an IP from a shared ephemeral IP address pool. If you\nspecify a static external IP address, it must live in the same region as\nthe zone of the instance."]
    pub fn external_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_ip", self.base))
    }
}

#[derive(Serialize, Default)]
struct WorkbenchInstance_GceSetup_NetworkInterfacesDynamic {
    access_configs: std
    ::option
    ::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigs>>,
}

#[derive(Serialize)]
pub struct WorkbenchInstance_GceSetup_NetworkInterfaces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nic_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_configs: std::option::Option<std::vec::Vec<WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigs>>,
    dynamic: WorkbenchInstance_GceSetup_NetworkInterfacesDynamic,
}

impl WorkbenchInstance_GceSetup_NetworkInterfaces {
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

    #[doc = "Set the field `access_configs`.\n"]
    pub fn set_access_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.access_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.access_configs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstance_GceSetup_NetworkInterfaces {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup_NetworkInterfaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup_NetworkInterfaces {}

impl BuildWorkbenchInstance_GceSetup_NetworkInterfaces {
    pub fn build(self) -> WorkbenchInstance_GceSetup_NetworkInterfaces {
        WorkbenchInstance_GceSetup_NetworkInterfaces {
            network: core::default::Default::default(),
            nic_type: core::default::Default::default(),
            subnet: core::default::Default::default(),
            access_configs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetup_NetworkInterfacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetup_NetworkInterfacesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkbenchInstance_GceSetup_NetworkInterfacesRef {
        WorkbenchInstance_GceSetup_NetworkInterfacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetup_NetworkInterfacesRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `access_configs` after provisioning.\n"]
    pub fn access_configs(&self) -> terrars::ListRef<WorkbenchInstance_GceSetup_NetworkInterfaces_AccessConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstance_GceSetup_ReservationAffinity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consume_reservation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl WorkbenchInstance_GceSetup_ReservationAffinity {
    #[doc =
        "Set the field `consume_reservation_type`.\nSpecifies the type of reservation from which this instance can consume resources:\nRESERVATION_ANY (default), RESERVATION_SPECIFIC, or RESERVATION_NONE. Possible values: [\"RESERVATION_NONE\", \"RESERVATION_ANY\", \"RESERVATION_SPECIFIC\"]"]
    pub fn set_consume_reservation_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.consume_reservation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `key`.\nCorresponds to the label key of a reservation resource. To target a\nRESERVATION_SPECIFIC by name, use compute.googleapis.com/reservation-name\nas the key and specify the name of your reservation as its value."]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `values`.\nCorresponds to the label values of a reservation resource. This can be\neither a name to a reservation in the same project or\n\"projects/different-project/reservations/some-reservation-name\"\nto target a shared reservation in the same zone but in a different project."]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstance_GceSetup_ReservationAffinity {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup_ReservationAffinity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup_ReservationAffinity {}

impl BuildWorkbenchInstance_GceSetup_ReservationAffinity {
    pub fn build(self) -> WorkbenchInstance_GceSetup_ReservationAffinity {
        WorkbenchInstance_GceSetup_ReservationAffinity {
            consume_reservation_type: core::default::Default::default(),
            key: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetup_ReservationAffinityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetup_ReservationAffinityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkbenchInstance_GceSetup_ReservationAffinityRef {
        WorkbenchInstance_GceSetup_ReservationAffinityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetup_ReservationAffinityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `consume_reservation_type` after provisioning.\nSpecifies the type of reservation from which this instance can consume resources:\nRESERVATION_ANY (default), RESERVATION_SPECIFIC, or RESERVATION_NONE. Possible values: [\"RESERVATION_NONE\", \"RESERVATION_ANY\", \"RESERVATION_SPECIFIC\"]"]
    pub fn consume_reservation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.consume_reservation_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nCorresponds to the label key of a reservation resource. To target a\nRESERVATION_SPECIFIC by name, use compute.googleapis.com/reservation-name\nas the key and specify the name of your reservation as its value."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nCorresponds to the label values of a reservation resource. This can be\neither a name to a reservation in the same project or\n\"projects/different-project/reservations/some-reservation-name\"\nto target a shared reservation in the same zone but in a different project."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstance_GceSetup_ServiceAccounts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstance_GceSetup_ServiceAccounts {
    #[doc = "Set the field `email`.\nOptional. Email address of the service account."]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstance_GceSetup_ServiceAccounts {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup_ServiceAccounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup_ServiceAccounts {}

impl BuildWorkbenchInstance_GceSetup_ServiceAccounts {
    pub fn build(self) -> WorkbenchInstance_GceSetup_ServiceAccounts {
        WorkbenchInstance_GceSetup_ServiceAccounts { email: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetup_ServiceAccountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetup_ServiceAccountsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstance_GceSetup_ServiceAccountsRef {
        WorkbenchInstance_GceSetup_ServiceAccountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetup_ServiceAccountsRef {
    #[allow(dead_code)]
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
pub struct WorkbenchInstance_GceSetup_ShieldedInstanceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_integrity_monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_vtpm: std::option::Option<terrars::PrimField<bool>>,
}

impl WorkbenchInstance_GceSetup_ShieldedInstanceConfig {
    #[doc =
        "Set the field `enable_integrity_monitoring`.\nOptional. Defines whether the VM instance has integrity monitoring\nenabled. Enables monitoring and attestation of the boot integrity of the VM\ninstance. The attestation is performed against the integrity policy baseline.\nThis baseline is initially derived from the implicitly trusted boot image\nwhen the VM instance is created. Enabled by default."]
    pub fn set_enable_integrity_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_integrity_monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_secure_boot`.\nOptional. Defines whether the VM instance has Secure Boot enabled.\nSecure Boot helps ensure that the system only runs authentic software by verifying\nthe digital signature of all boot components, and halting the boot process\nif signature verification fails. Disabled by default."]
    pub fn set_enable_secure_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_secure_boot = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_vtpm`.\nOptional. Defines whether the VM instance has the vTPM enabled.\nEnabled by default."]
    pub fn set_enable_vtpm(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_vtpm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WorkbenchInstance_GceSetup_ShieldedInstanceConfig {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup_ShieldedInstanceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup_ShieldedInstanceConfig {}

impl BuildWorkbenchInstance_GceSetup_ShieldedInstanceConfig {
    pub fn build(self) -> WorkbenchInstance_GceSetup_ShieldedInstanceConfig {
        WorkbenchInstance_GceSetup_ShieldedInstanceConfig {
            enable_integrity_monitoring: core::default::Default::default(),
            enable_secure_boot: core::default::Default::default(),
            enable_vtpm: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetup_ShieldedInstanceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetup_ShieldedInstanceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WorkbenchInstance_GceSetup_ShieldedInstanceConfigRef {
        WorkbenchInstance_GceSetup_ShieldedInstanceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetup_ShieldedInstanceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_integrity_monitoring` after provisioning.\nOptional. Defines whether the VM instance has integrity monitoring\nenabled. Enables monitoring and attestation of the boot integrity of the VM\ninstance. The attestation is performed against the integrity policy baseline.\nThis baseline is initially derived from the implicitly trusted boot image\nwhen the VM instance is created. Enabled by default."]
    pub fn enable_integrity_monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_integrity_monitoring", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_secure_boot` after provisioning.\nOptional. Defines whether the VM instance has Secure Boot enabled.\nSecure Boot helps ensure that the system only runs authentic software by verifying\nthe digital signature of all boot components, and halting the boot process\nif signature verification fails. Disabled by default."]
    pub fn enable_secure_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_vtpm` after provisioning.\nOptional. Defines whether the VM instance has the vTPM enabled.\nEnabled by default."]
    pub fn enable_vtpm(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_vtpm", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstance_GceSetup_VmImage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    family: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstance_GceSetup_VmImage {
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

impl terrars::ToListMappable for WorkbenchInstance_GceSetup_VmImage {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup_VmImage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup_VmImage {}

impl BuildWorkbenchInstance_GceSetup_VmImage {
    pub fn build(self) -> WorkbenchInstance_GceSetup_VmImage {
        WorkbenchInstance_GceSetup_VmImage {
            family: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetup_VmImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetup_VmImageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstance_GceSetup_VmImageRef {
        WorkbenchInstance_GceSetup_VmImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetup_VmImageRef {
    #[allow(dead_code)]
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
struct WorkbenchInstance_GceSetupDynamic {
    accelerator_configs: std::option::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup_AcceleratorConfigs>>,
    boot_disk: std::option::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup_BootDisk>>,
    confidential_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup_ConfidentialInstanceConfig>>,
    container_image: std::option::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup_ContainerImage>>,
    data_disks: std::option::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup_DataDisks>>,
    network_interfaces: std::option::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup_NetworkInterfaces>>,
    reservation_affinity: std::option::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup_ReservationAffinity>>,
    service_accounts: std::option::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup_ServiceAccounts>>,
    shielded_instance_config: std
    ::option
    ::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup_ShieldedInstanceConfig>>,
    vm_image: std::option::Option<terrars::DynamicBlock<WorkbenchInstance_GceSetup_VmImage>>,
}

#[derive(Serialize)]
pub struct WorkbenchInstance_GceSetup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ip_forwarding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_cpu_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_configs: std::option::Option<std::vec::Vec<WorkbenchInstance_GceSetup_AcceleratorConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boot_disk: std::option::Option<std::vec::Vec<WorkbenchInstance_GceSetup_BootDisk>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidential_instance_config: std
    ::option
    ::Option<std::vec::Vec<WorkbenchInstance_GceSetup_ConfidentialInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_image: std::option::Option<std::vec::Vec<WorkbenchInstance_GceSetup_ContainerImage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_disks: std::option::Option<std::vec::Vec<WorkbenchInstance_GceSetup_DataDisks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interfaces: std::option::Option<std::vec::Vec<WorkbenchInstance_GceSetup_NetworkInterfaces>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_affinity: std::option::Option<std::vec::Vec<WorkbenchInstance_GceSetup_ReservationAffinity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_accounts: std::option::Option<std::vec::Vec<WorkbenchInstance_GceSetup_ServiceAccounts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_instance_config: std::option::Option<std::vec::Vec<WorkbenchInstance_GceSetup_ShieldedInstanceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vm_image: std::option::Option<std::vec::Vec<WorkbenchInstance_GceSetup_VmImage>>,
    dynamic: WorkbenchInstance_GceSetupDynamic,
}

impl WorkbenchInstance_GceSetup {
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

    #[doc = "Set the field `min_cpu_platform`.\nOptional. The minimum CPU platform to use for this instance."]
    pub fn set_min_cpu_platform(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_cpu_platform = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup_AcceleratorConfigs>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup_BootDisk>>,
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

    #[doc = "Set the field `confidential_instance_config`.\n"]
    pub fn set_confidential_instance_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup_ConfidentialInstanceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confidential_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confidential_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container_image`.\n"]
    pub fn set_container_image(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup_ContainerImage>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_disks`.\n"]
    pub fn set_data_disks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup_DataDisks>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup_NetworkInterfaces>>,
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

    #[doc = "Set the field `reservation_affinity`.\n"]
    pub fn set_reservation_affinity(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup_ReservationAffinity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.reservation_affinity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.reservation_affinity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_accounts`.\n"]
    pub fn set_service_accounts(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup_ServiceAccounts>>,
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

    #[doc = "Set the field `shielded_instance_config`.\n"]
    pub fn set_shielded_instance_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup_ShieldedInstanceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.shielded_instance_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.shielded_instance_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vm_image`.\n"]
    pub fn set_vm_image(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WorkbenchInstance_GceSetup_VmImage>>,
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

impl terrars::ToListMappable for WorkbenchInstance_GceSetup {
    type O = terrars::BlockAssignable<WorkbenchInstance_GceSetup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_GceSetup {}

impl BuildWorkbenchInstance_GceSetup {
    pub fn build(self) -> WorkbenchInstance_GceSetup {
        WorkbenchInstance_GceSetup {
            disable_public_ip: core::default::Default::default(),
            enable_ip_forwarding: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            metadata: core::default::Default::default(),
            min_cpu_platform: core::default::Default::default(),
            tags: core::default::Default::default(),
            accelerator_configs: core::default::Default::default(),
            boot_disk: core::default::Default::default(),
            confidential_instance_config: core::default::Default::default(),
            container_image: core::default::Default::default(),
            data_disks: core::default::Default::default(),
            network_interfaces: core::default::Default::default(),
            reservation_affinity: core::default::Default::default(),
            service_accounts: core::default::Default::default(),
            shielded_instance_config: core::default::Default::default(),
            vm_image: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_GceSetupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_GceSetupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstance_GceSetupRef {
        WorkbenchInstance_GceSetupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_GceSetupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `compute_instance_id` after provisioning.\nOutput only. The unique numeric identifier of the underlying Compute Engine VM instance."]
    pub fn compute_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_instance_id", self.base))
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
        "Get a reference to the value of field `min_cpu_platform` after provisioning.\nOptional. The minimum CPU platform to use for this instance."]
    pub fn min_cpu_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_cpu_platform", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nOptional. The Compute Engine tags to add to instance (see [Tagging\ninstances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags))."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_configs` after provisioning.\n"]
    pub fn accelerator_configs(&self) -> terrars::ListRef<WorkbenchInstance_GceSetup_AcceleratorConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `boot_disk` after provisioning.\n"]
    pub fn boot_disk(&self) -> terrars::ListRef<WorkbenchInstance_GceSetup_BootDiskRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boot_disk", self.base))
    }

    #[doc = "Get a reference to the value of field `confidential_instance_config` after provisioning.\n"]
    pub fn confidential_instance_config(
        &self,
    ) -> terrars::ListRef<WorkbenchInstance_GceSetup_ConfidentialInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.confidential_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `container_image` after provisioning.\n"]
    pub fn container_image(&self) -> terrars::ListRef<WorkbenchInstance_GceSetup_ContainerImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_image", self.base))
    }

    #[doc = "Get a reference to the value of field `data_disks` after provisioning.\n"]
    pub fn data_disks(&self) -> terrars::ListRef<WorkbenchInstance_GceSetup_DataDisksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_disks", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(&self) -> terrars::ListRef<WorkbenchInstance_GceSetup_NetworkInterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_affinity` after provisioning.\n"]
    pub fn reservation_affinity(&self) -> terrars::ListRef<WorkbenchInstance_GceSetup_ReservationAffinityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.reservation_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `service_accounts` after provisioning.\n"]
    pub fn service_accounts(&self) -> terrars::ListRef<WorkbenchInstance_GceSetup_ServiceAccountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `shielded_instance_config` after provisioning.\n"]
    pub fn shielded_instance_config(&self) -> terrars::ListRef<WorkbenchInstance_GceSetup_ShieldedInstanceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_instance_config", self.base))
    }

    #[doc = "Get a reference to the value of field `vm_image` after provisioning.\n"]
    pub fn vm_image(&self) -> terrars::ListRef<WorkbenchInstance_GceSetup_VmImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vm_image", self.base))
    }
}

#[derive(Serialize)]
pub struct WorkbenchInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl WorkbenchInstance_Timeouts {
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

impl terrars::ToListMappable for WorkbenchInstance_Timeouts {
    type O = terrars::BlockAssignable<WorkbenchInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWorkbenchInstance_Timeouts {}

impl BuildWorkbenchInstance_Timeouts {
    pub fn build(self) -> WorkbenchInstance_Timeouts {
        WorkbenchInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WorkbenchInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WorkbenchInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WorkbenchInstance_TimeoutsRef {
        WorkbenchInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WorkbenchInstance_TimeoutsRef {
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
