#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ColabRuntimeTemplateData_ {
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
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_persistent_disk_spec: std::option::Option<std::vec::Vec<ColabRuntimeTemplate_DataPersistentDiskSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_spec: std::option::Option<std::vec::Vec<ColabRuntimeTemplate_EncryptionSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    euc_config: std::option::Option<std::vec::Vec<ColabRuntimeTemplate_EucConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_shutdown_config: std::option::Option<std::vec::Vec<ColabRuntimeTemplate_IdleShutdownConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_spec: std::option::Option<std::vec::Vec<ColabRuntimeTemplate_MachineSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_spec: std::option::Option<std::vec::Vec<ColabRuntimeTemplate_NetworkSpec>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shielded_vm_config: std::option::Option<std::vec::Vec<ColabRuntimeTemplate_ShieldedVmConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    software_config: std::option::Option<std::vec::Vec<ColabRuntimeTemplate_SoftwareConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ColabRuntimeTemplate_Timeouts>,
    dynamic: ColabRuntimeTemplateDynamic,
}

struct ColabRuntimeTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ColabRuntimeTemplateData_>,
}

#[derive(Clone)]
pub struct ColabRuntimeTemplate(std::rc::Rc<ColabRuntimeTemplate_>);

impl ColabRuntimeTemplate {
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

    #[doc = "Set the field `description`.\nThe description of the Runtime Template."]
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
        "Set the field `labels`.\nLabels to identify and group the runtime template.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe resource name of the Runtime Template"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_tags`.\nApplies the given Compute Engine tags to the runtime."]
    pub fn set_network_tags(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().network_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_persistent_disk_spec`.\n"]
    pub fn set_data_persistent_disk_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabRuntimeTemplate_DataPersistentDiskSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_persistent_disk_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_persistent_disk_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_spec`.\n"]
    pub fn set_encryption_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabRuntimeTemplate_EncryptionSpec>>,
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

    #[doc = "Set the field `euc_config`.\n"]
    pub fn set_euc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabRuntimeTemplate_EucConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().euc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.euc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `idle_shutdown_config`.\n"]
    pub fn set_idle_shutdown_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabRuntimeTemplate_IdleShutdownConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().idle_shutdown_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.idle_shutdown_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `machine_spec`.\n"]
    pub fn set_machine_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabRuntimeTemplate_MachineSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().machine_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.machine_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_spec`.\n"]
    pub fn set_network_spec(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabRuntimeTemplate_NetworkSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `shielded_vm_config`.\n"]
    pub fn set_shielded_vm_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabRuntimeTemplate_ShieldedVmConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().shielded_vm_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.shielded_vm_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `software_config`.\n"]
    pub fn set_software_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabRuntimeTemplate_SoftwareConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().software_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.software_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ColabRuntimeTemplate_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the Runtime Template."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Runtime Template."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to identify and group the runtime template.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Runtime Template"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_tags` after provisioning.\nApplies the given Compute Engine tags to the runtime."]
    pub fn network_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_persistent_disk_spec` after provisioning.\n"]
    pub fn data_persistent_disk_spec(&self) -> terrars::ListRef<ColabRuntimeTemplate_DataPersistentDiskSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_persistent_disk_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<ColabRuntimeTemplate_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `euc_config` after provisioning.\n"]
    pub fn euc_config(&self) -> terrars::ListRef<ColabRuntimeTemplate_EucConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.euc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_shutdown_config` after provisioning.\n"]
    pub fn idle_shutdown_config(&self) -> terrars::ListRef<ColabRuntimeTemplate_IdleShutdownConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle_shutdown_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(&self) -> terrars::ListRef<ColabRuntimeTemplate_MachineSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_spec` after provisioning.\n"]
    pub fn network_spec(&self) -> terrars::ListRef<ColabRuntimeTemplate_NetworkSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shielded_vm_config` after provisioning.\n"]
    pub fn shielded_vm_config(&self) -> terrars::ListRef<ColabRuntimeTemplate_ShieldedVmConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_vm_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> terrars::ListRef<ColabRuntimeTemplate_SoftwareConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabRuntimeTemplate_TimeoutsRef {
        ColabRuntimeTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ColabRuntimeTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ColabRuntimeTemplate { }

impl terrars::ToListMappable for ColabRuntimeTemplate {
    type O = terrars::ListRef<ColabRuntimeTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ColabRuntimeTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_colab_runtime_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildColabRuntimeTemplate {
    pub tf_id: std::string::String,
    #[doc = "Required. The display name of the Runtime Template."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc = "The location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildColabRuntimeTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> ColabRuntimeTemplate {
        let out = ColabRuntimeTemplate(std::rc::Rc::new(ColabRuntimeTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ColabRuntimeTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: core::default::Default::default(),
                network_tags: core::default::Default::default(),
                project: core::default::Default::default(),
                data_persistent_disk_spec: core::default::Default::default(),
                encryption_spec: core::default::Default::default(),
                euc_config: core::default::Default::default(),
                idle_shutdown_config: core::default::Default::default(),
                machine_spec: core::default::Default::default(),
                network_spec: core::default::Default::default(),
                shielded_vm_config: core::default::Default::default(),
                software_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ColabRuntimeTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ColabRuntimeTemplateRef {
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

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the Runtime Template."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nRequired. The display name of the Runtime Template."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels to identify and group the runtime template.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location for the resource: https://cloud.google.com/colab/docs/locations"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Runtime Template"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `network_tags` after provisioning.\nApplies the given Compute Engine tags to the runtime."]
    pub fn network_tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_persistent_disk_spec` after provisioning.\n"]
    pub fn data_persistent_disk_spec(&self) -> terrars::ListRef<ColabRuntimeTemplate_DataPersistentDiskSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_persistent_disk_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_spec` after provisioning.\n"]
    pub fn encryption_spec(&self) -> terrars::ListRef<ColabRuntimeTemplate_EncryptionSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `euc_config` after provisioning.\n"]
    pub fn euc_config(&self) -> terrars::ListRef<ColabRuntimeTemplate_EucConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.euc_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_shutdown_config` after provisioning.\n"]
    pub fn idle_shutdown_config(&self) -> terrars::ListRef<ColabRuntimeTemplate_IdleShutdownConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle_shutdown_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `machine_spec` after provisioning.\n"]
    pub fn machine_spec(&self) -> terrars::ListRef<ColabRuntimeTemplate_MachineSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_spec` after provisioning.\n"]
    pub fn network_spec(&self) -> terrars::ListRef<ColabRuntimeTemplate_NetworkSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `shielded_vm_config` after provisioning.\n"]
    pub fn shielded_vm_config(&self) -> terrars::ListRef<ColabRuntimeTemplate_ShieldedVmConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shielded_vm_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `software_config` after provisioning.\n"]
    pub fn software_config(&self) -> terrars::ListRef<ColabRuntimeTemplate_SoftwareConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.software_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ColabRuntimeTemplate_TimeoutsRef {
        ColabRuntimeTemplate_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ColabRuntimeTemplateDynamic {
    data_persistent_disk_spec: std
    ::option
    ::Option<terrars::DynamicBlock<ColabRuntimeTemplate_DataPersistentDiskSpec>>,
    encryption_spec: std::option::Option<terrars::DynamicBlock<ColabRuntimeTemplate_EncryptionSpec>>,
    euc_config: std::option::Option<terrars::DynamicBlock<ColabRuntimeTemplate_EucConfig>>,
    idle_shutdown_config: std::option::Option<terrars::DynamicBlock<ColabRuntimeTemplate_IdleShutdownConfig>>,
    machine_spec: std::option::Option<terrars::DynamicBlock<ColabRuntimeTemplate_MachineSpec>>,
    network_spec: std::option::Option<terrars::DynamicBlock<ColabRuntimeTemplate_NetworkSpec>>,
    shielded_vm_config: std::option::Option<terrars::DynamicBlock<ColabRuntimeTemplate_ShieldedVmConfig>>,
    software_config: std::option::Option<terrars::DynamicBlock<ColabRuntimeTemplate_SoftwareConfig>>,
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_DataPersistentDiskSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_size_gb: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabRuntimeTemplate_DataPersistentDiskSpec {
    #[doc =
        "Set the field `disk_size_gb`.\nThe disk size of the runtime in GB. If specified, the diskType must also be specified. The minimum size is 10GB and the maximum is 65536GB."]
    pub fn set_disk_size_gb(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_size_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_type`.\nThe type of the persistent disk."]
    pub fn set_disk_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabRuntimeTemplate_DataPersistentDiskSpec {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_DataPersistentDiskSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_DataPersistentDiskSpec {}

impl BuildColabRuntimeTemplate_DataPersistentDiskSpec {
    pub fn build(self) -> ColabRuntimeTemplate_DataPersistentDiskSpec {
        ColabRuntimeTemplate_DataPersistentDiskSpec {
            disk_size_gb: core::default::Default::default(),
            disk_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_DataPersistentDiskSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_DataPersistentDiskSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabRuntimeTemplate_DataPersistentDiskSpecRef {
        ColabRuntimeTemplate_DataPersistentDiskSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_DataPersistentDiskSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disk_size_gb` after provisioning.\nThe disk size of the runtime in GB. If specified, the diskType must also be specified. The minimum size is 10GB and the maximum is 65536GB."]
    pub fn disk_size_gb(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_size_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `disk_type` after provisioning.\nThe type of the persistent disk."]
    pub fn disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_EncryptionSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabRuntimeTemplate_EncryptionSpec {
    #[doc =
        "Set the field `kms_key_name`.\nThe Cloud KMS encryption key (customer-managed encryption key) used to protect the runtime."]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabRuntimeTemplate_EncryptionSpec {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_EncryptionSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_EncryptionSpec {}

impl BuildColabRuntimeTemplate_EncryptionSpec {
    pub fn build(self) -> ColabRuntimeTemplate_EncryptionSpec {
        ColabRuntimeTemplate_EncryptionSpec { kms_key_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_EncryptionSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_EncryptionSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabRuntimeTemplate_EncryptionSpecRef {
        ColabRuntimeTemplate_EncryptionSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_EncryptionSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nThe Cloud KMS encryption key (customer-managed encryption key) used to protect the runtime."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_EucConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    euc_disabled: std::option::Option<terrars::PrimField<bool>>,
}

impl ColabRuntimeTemplate_EucConfig {
    #[doc = "Set the field `euc_disabled`.\nDisable end user credential access for the runtime."]
    pub fn set_euc_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.euc_disabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabRuntimeTemplate_EucConfig {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_EucConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_EucConfig {}

impl BuildColabRuntimeTemplate_EucConfig {
    pub fn build(self) -> ColabRuntimeTemplate_EucConfig {
        ColabRuntimeTemplate_EucConfig { euc_disabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_EucConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_EucConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabRuntimeTemplate_EucConfigRef {
        ColabRuntimeTemplate_EucConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_EucConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `euc_disabled` after provisioning.\nDisable end user credential access for the runtime."]
    pub fn euc_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.euc_disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_IdleShutdownConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabRuntimeTemplate_IdleShutdownConfig {
    #[doc =
        "Set the field `idle_timeout`.\nThe duration after which the runtime is automatically shut down. An input of 0s disables the idle shutdown feature, and a valid range is [10m, 24h]."]
    pub fn set_idle_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.idle_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabRuntimeTemplate_IdleShutdownConfig {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_IdleShutdownConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_IdleShutdownConfig {}

impl BuildColabRuntimeTemplate_IdleShutdownConfig {
    pub fn build(self) -> ColabRuntimeTemplate_IdleShutdownConfig {
        ColabRuntimeTemplate_IdleShutdownConfig { idle_timeout: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_IdleShutdownConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_IdleShutdownConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabRuntimeTemplate_IdleShutdownConfigRef {
        ColabRuntimeTemplate_IdleShutdownConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_IdleShutdownConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `idle_timeout` after provisioning.\nThe duration after which the runtime is automatically shut down. An input of 0s disables the idle shutdown feature, and a valid range is [10m, 24h]."]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_MachineSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabRuntimeTemplate_MachineSpec {
    #[doc = "Set the field `accelerator_count`.\nThe number of accelerators used by the runtime."]
    pub fn set_accelerator_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `accelerator_type`.\nThe type of hardware accelerator used by the runtime. If specified, acceleratorCount must also be specified."]
    pub fn set_accelerator_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.accelerator_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\nThe Compute Engine machine type selected for the runtime."]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabRuntimeTemplate_MachineSpec {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_MachineSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_MachineSpec {}

impl BuildColabRuntimeTemplate_MachineSpec {
    pub fn build(self) -> ColabRuntimeTemplate_MachineSpec {
        ColabRuntimeTemplate_MachineSpec {
            accelerator_count: core::default::Default::default(),
            accelerator_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_MachineSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_MachineSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabRuntimeTemplate_MachineSpecRef {
        ColabRuntimeTemplate_MachineSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_MachineSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `accelerator_count` after provisioning.\nThe number of accelerators used by the runtime."]
    pub fn accelerator_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `accelerator_type` after provisioning.\nThe type of hardware accelerator used by the runtime. If specified, acceleratorCount must also be specified."]
    pub fn accelerator_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.accelerator_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `machine_type` after provisioning.\nThe Compute Engine machine type selected for the runtime."]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_NetworkSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_internet_access: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabRuntimeTemplate_NetworkSpec {
    #[doc = "Set the field `enable_internet_access`.\nEnable public internet access for the runtime."]
    pub fn set_enable_internet_access(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_internet_access = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\nThe name of the VPC that this runtime is in."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnetwork`.\nThe name of the subnetwork that this runtime is in."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabRuntimeTemplate_NetworkSpec {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_NetworkSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_NetworkSpec {}

impl BuildColabRuntimeTemplate_NetworkSpec {
    pub fn build(self) -> ColabRuntimeTemplate_NetworkSpec {
        ColabRuntimeTemplate_NetworkSpec {
            enable_internet_access: core::default::Default::default(),
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_NetworkSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_NetworkSpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabRuntimeTemplate_NetworkSpecRef {
        ColabRuntimeTemplate_NetworkSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_NetworkSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_internet_access` after provisioning.\nEnable public internet access for the runtime."]
    pub fn enable_internet_access(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_internet_access", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC that this runtime is in."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe name of the subnetwork that this runtime is in."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_ShieldedVmConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_secure_boot: std::option::Option<terrars::PrimField<bool>>,
}

impl ColabRuntimeTemplate_ShieldedVmConfig {
    #[doc = "Set the field `enable_secure_boot`.\nEnables secure boot for the runtime."]
    pub fn set_enable_secure_boot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_secure_boot = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabRuntimeTemplate_ShieldedVmConfig {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_ShieldedVmConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_ShieldedVmConfig {}

impl BuildColabRuntimeTemplate_ShieldedVmConfig {
    pub fn build(self) -> ColabRuntimeTemplate_ShieldedVmConfig {
        ColabRuntimeTemplate_ShieldedVmConfig { enable_secure_boot: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_ShieldedVmConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_ShieldedVmConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabRuntimeTemplate_ShieldedVmConfigRef {
        ColabRuntimeTemplate_ShieldedVmConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_ShieldedVmConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_secure_boot` after provisioning.\nEnables secure boot for the runtime."]
    pub fn enable_secure_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_secure_boot", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_SoftwareConfig_ColabImage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    release_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabRuntimeTemplate_SoftwareConfig_ColabImage {
    #[doc =
        "Set the field `release_name`.\nThe release name of the NotebookRuntime Colab image, e.g. \"py310\". If not specified, detault to the latest release."]
    pub fn set_release_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.release_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabRuntimeTemplate_SoftwareConfig_ColabImage {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_SoftwareConfig_ColabImage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_SoftwareConfig_ColabImage {}

impl BuildColabRuntimeTemplate_SoftwareConfig_ColabImage {
    pub fn build(self) -> ColabRuntimeTemplate_SoftwareConfig_ColabImage {
        ColabRuntimeTemplate_SoftwareConfig_ColabImage { release_name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_SoftwareConfig_ColabImageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_SoftwareConfig_ColabImageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabRuntimeTemplate_SoftwareConfig_ColabImageRef {
        ColabRuntimeTemplate_SoftwareConfig_ColabImageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_SoftwareConfig_ColabImageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `release_name` after provisioning.\nThe release name of the NotebookRuntime Colab image, e.g. \"py310\". If not specified, detault to the latest release."]
    pub fn release_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_name", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_SoftwareConfig_Env {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabRuntimeTemplate_SoftwareConfig_Env {
    #[doc = "Set the field `name`.\nName of the environment variable. Must be a valid C identifier."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `value`.\nVariables that reference a $(VAR_NAME) are expanded using the previous defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabRuntimeTemplate_SoftwareConfig_Env {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_SoftwareConfig_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_SoftwareConfig_Env {}

impl BuildColabRuntimeTemplate_SoftwareConfig_Env {
    pub fn build(self) -> ColabRuntimeTemplate_SoftwareConfig_Env {
        ColabRuntimeTemplate_SoftwareConfig_Env {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_SoftwareConfig_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_SoftwareConfig_EnvRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabRuntimeTemplate_SoftwareConfig_EnvRef {
        ColabRuntimeTemplate_SoftwareConfig_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_SoftwareConfig_EnvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the environment variable. Must be a valid C identifier."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nVariables that reference a $(VAR_NAME) are expanded using the previous defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_startup_script: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_startup_script_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_startup_script_url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfig {
    #[doc = "Set the field `post_startup_script`.\nPost startup script to run after runtime is started."]
    pub fn set_post_startup_script(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.post_startup_script = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `post_startup_script_behavior`.\nPost startup script behavior that defines download and execution behavior. Possible values: [\"RUN_ONCE\", \"RUN_EVERY_START\", \"DOWNLOAD_AND_RUN_EVERY_START\"]"]
    pub fn set_post_startup_script_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.post_startup_script_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `post_startup_script_url`.\nPost startup script url to download. Example: https://bucket/script.sh."]
    pub fn set_post_startup_script_url(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.post_startup_script_url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfig {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfig {}

impl BuildColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfig {
    pub fn build(self) -> ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfig {
        ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfig {
            post_startup_script: core::default::Default::default(),
            post_startup_script_behavior: core::default::Default::default(),
            post_startup_script_url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfigRef {
        ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `post_startup_script` after provisioning.\nPost startup script to run after runtime is started."]
    pub fn post_startup_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.post_startup_script", self.base))
    }

    #[doc =
        "Get a reference to the value of field `post_startup_script_behavior` after provisioning.\nPost startup script behavior that defines download and execution behavior. Possible values: [\"RUN_ONCE\", \"RUN_EVERY_START\", \"DOWNLOAD_AND_RUN_EVERY_START\"]"]
    pub fn post_startup_script_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.post_startup_script_behavior", self.base))
    }

    #[doc =
        "Get a reference to the value of field `post_startup_script_url` after provisioning.\nPost startup script url to download. Example: https://bucket/script.sh."]
    pub fn post_startup_script_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.post_startup_script_url", self.base))
    }
}

#[derive(Serialize, Default)]
struct ColabRuntimeTemplate_SoftwareConfigDynamic {
    colab_image: std::option::Option<terrars::DynamicBlock<ColabRuntimeTemplate_SoftwareConfig_ColabImage>>,
    env: std::option::Option<terrars::DynamicBlock<ColabRuntimeTemplate_SoftwareConfig_Env>>,
    post_startup_script_config: std
    ::option
    ::Option<terrars::DynamicBlock<ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfig>>,
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_SoftwareConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    colab_image: std::option::Option<std::vec::Vec<ColabRuntimeTemplate_SoftwareConfig_ColabImage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<ColabRuntimeTemplate_SoftwareConfig_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_startup_script_config: std
    ::option
    ::Option<std::vec::Vec<ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfig>>,
    dynamic: ColabRuntimeTemplate_SoftwareConfigDynamic,
}

impl ColabRuntimeTemplate_SoftwareConfig {
    #[doc = "Set the field `colab_image`.\n"]
    pub fn set_colab_image(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabRuntimeTemplate_SoftwareConfig_ColabImage>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.colab_image = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.colab_image = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ColabRuntimeTemplate_SoftwareConfig_Env>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `post_startup_script_config`.\n"]
    pub fn set_post_startup_script_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.post_startup_script_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.post_startup_script_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ColabRuntimeTemplate_SoftwareConfig {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_SoftwareConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_SoftwareConfig {}

impl BuildColabRuntimeTemplate_SoftwareConfig {
    pub fn build(self) -> ColabRuntimeTemplate_SoftwareConfig {
        ColabRuntimeTemplate_SoftwareConfig {
            colab_image: core::default::Default::default(),
            env: core::default::Default::default(),
            post_startup_script_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_SoftwareConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_SoftwareConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabRuntimeTemplate_SoftwareConfigRef {
        ColabRuntimeTemplate_SoftwareConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_SoftwareConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `colab_image` after provisioning.\n"]
    pub fn colab_image(&self) -> terrars::ListRef<ColabRuntimeTemplate_SoftwareConfig_ColabImageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.colab_image", self.base))
    }

    #[doc = "Get a reference to the value of field `env` after provisioning.\n"]
    pub fn env(&self) -> terrars::ListRef<ColabRuntimeTemplate_SoftwareConfig_EnvRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `post_startup_script_config` after provisioning.\n"]
    pub fn post_startup_script_config(
        &self,
    ) -> terrars::ListRef<ColabRuntimeTemplate_SoftwareConfig_PostStartupScriptConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_startup_script_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ColabRuntimeTemplate_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ColabRuntimeTemplate_Timeouts {
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

impl terrars::ToListMappable for ColabRuntimeTemplate_Timeouts {
    type O = terrars::BlockAssignable<ColabRuntimeTemplate_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildColabRuntimeTemplate_Timeouts {}

impl BuildColabRuntimeTemplate_Timeouts {
    pub fn build(self) -> ColabRuntimeTemplate_Timeouts {
        ColabRuntimeTemplate_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ColabRuntimeTemplate_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ColabRuntimeTemplate_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ColabRuntimeTemplate_TimeoutsRef {
        ColabRuntimeTemplate_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ColabRuntimeTemplate_TimeoutsRef {
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
