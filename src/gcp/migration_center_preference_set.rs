#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct MigrationCenterPreferenceSetData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    preference_set_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MigrationCenterPreferenceSet_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_machine_preferences: std
    ::option
    ::Option<std::vec::Vec<MigrationCenterPreferenceSet_VirtualMachinePreferences>>,
    dynamic: MigrationCenterPreferenceSetDynamic,
}

struct MigrationCenterPreferenceSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MigrationCenterPreferenceSetData_>,
}

#[derive(Clone)]
pub struct MigrationCenterPreferenceSet(std::rc::Rc<MigrationCenterPreferenceSet_>);

impl MigrationCenterPreferenceSet {
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

    #[doc = "Set the field `description`.\nA description of the preference set."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser-friendly display name. Maximum length is 63 characters."]
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
    pub fn set_timeouts(self, v: impl std::convert::Into<MigrationCenterPreferenceSet_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_machine_preferences`.\n"]
    pub fn set_virtual_machine_preferences(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MigrationCenterPreferenceSet_VirtualMachinePreferences>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().virtual_machine_preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.virtual_machine_preferences = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the preference set was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the preference set."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly display name. Maximum length is 63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the preference set."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preference_set_id` after provisioning.\nRequired. User specified ID for the preference set. It will become the last component of the preference set name. The ID must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. The ID must match the regular expression '[a-z]([a-z0-9-]{0,61}[a-z0-9])?'."]
    pub fn preference_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preference_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The timestamp when the preference set was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MigrationCenterPreferenceSet_TimeoutsRef {
        MigrationCenterPreferenceSet_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `virtual_machine_preferences` after provisioning.\n"]
    pub fn virtual_machine_preferences(
        &self,
    ) -> terrars::ListRef<MigrationCenterPreferenceSet_VirtualMachinePreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_machine_preferences", self.extract_ref()))
    }
}

impl terrars::Referable for MigrationCenterPreferenceSet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MigrationCenterPreferenceSet { }

impl terrars::ToListMappable for MigrationCenterPreferenceSet {
    type O = terrars::ListRef<MigrationCenterPreferenceSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MigrationCenterPreferenceSet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_migration_center_preference_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMigrationCenterPreferenceSet {
    pub tf_id: std::string::String,
    #[doc = "Part of 'parent'. See documentation of 'projectsId'."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. User specified ID for the preference set. It will become the last component of the preference set name. The ID must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. The ID must match the regular expression '[a-z]([a-z0-9-]{0,61}[a-z0-9])?'."]
    pub preference_set_id: terrars::PrimField<std::string::String>,
}

impl BuildMigrationCenterPreferenceSet {
    pub fn build(self, stack: &mut terrars::Stack) -> MigrationCenterPreferenceSet {
        let out = MigrationCenterPreferenceSet(std::rc::Rc::new(MigrationCenterPreferenceSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MigrationCenterPreferenceSetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                preference_set_id: self.preference_set_id,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                virtual_machine_preferences: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MigrationCenterPreferenceSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterPreferenceSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MigrationCenterPreferenceSetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the preference set was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the preference set."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly display name. Maximum length is 63 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the preference set."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `preference_set_id` after provisioning.\nRequired. User specified ID for the preference set. It will become the last component of the preference set name. The ID must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. The ID must match the regular expression '[a-z]([a-z0-9-]{0,61}[a-z0-9])?'."]
    pub fn preference_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preference_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The timestamp when the preference set was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MigrationCenterPreferenceSet_TimeoutsRef {
        MigrationCenterPreferenceSet_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `virtual_machine_preferences` after provisioning.\n"]
    pub fn virtual_machine_preferences(
        &self,
    ) -> terrars::ListRef<MigrationCenterPreferenceSet_VirtualMachinePreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.virtual_machine_preferences", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MigrationCenterPreferenceSetDynamic {
    virtual_machine_preferences: std
    ::option
    ::Option<terrars::DynamicBlock<MigrationCenterPreferenceSet_VirtualMachinePreferences>>,
}

#[derive(Serialize)]
pub struct MigrationCenterPreferenceSet_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterPreferenceSet_Timeouts {
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

impl terrars::ToListMappable for MigrationCenterPreferenceSet_Timeouts {
    type O = terrars::BlockAssignable<MigrationCenterPreferenceSet_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterPreferenceSet_Timeouts {}

impl BuildMigrationCenterPreferenceSet_Timeouts {
    pub fn build(self) -> MigrationCenterPreferenceSet_Timeouts {
        MigrationCenterPreferenceSet_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterPreferenceSet_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterPreferenceSet_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MigrationCenterPreferenceSet_TimeoutsRef {
        MigrationCenterPreferenceSet_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterPreferenceSet_TimeoutsRef {
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
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
    #[doc =
        "Set the field `code`.\nCode to identify a Compute Engine machine series. Consult https://cloud.google.com/compute/docs/machine-resource#machine_type_comparison for more details on the available series."]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {}

impl BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
    pub fn build(
        self,
    ) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries {
            code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `code` after provisioning.\nCode to identify a Compute Engine machine series. Consult https://cloud.google.com/compute/docs/machine-resource#machine_type_comparison for more details on the available series."]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
}

#[derive(Serialize, Default)]
struct MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferencesDynamic {
    allowed_machine_series: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries,
        >,
    >,
}

#[derive(Serialize)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_machine_series: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries,
        >,
    >,
    dynamic: MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferencesDynamic,
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences {
    #[doc = "Set the field `allowed_machine_series`.\n"]
    pub fn set_allowed_machine_series(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeries,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_machine_series = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_machine_series = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences {
    type O =
        terrars
        ::BlockAssignable<
            MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences {}

impl BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences {
    pub fn build(
        self,
    ) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences {
            allowed_machine_series: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferencesRef {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_machine_series` after provisioning.\n"]
    pub fn allowed_machine_series(
        &self,
    ) -> terrars
    ::ListRef<
        MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences_AllowedMachineSeriesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_machine_series", self.base))
    }
}

#[derive(Serialize, Default)]
struct MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferencesDynamic {
    machine_preferences: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences,
        >,
    >,
}

#[derive(Serialize)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    persistent_disk_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_preferences: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences>,
    >,
    dynamic: MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferencesDynamic,
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences {
    #[doc =
        "Set the field `license_type`.\nLicense type to consider when calculating costs for virtual machine insights and recommendations. If unspecified, costs are calculated based on the default licensing plan. Possible values: 'LICENSE_TYPE_UNSPECIFIED', 'LICENSE_TYPE_DEFAULT', 'LICENSE_TYPE_BRING_YOUR_OWN_LICENSE'"]
    pub fn set_license_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.license_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `persistent_disk_type`.\nPersistent disk type to use. If unspecified (default), all types are considered, based on available usage data. Possible values: [\"PERSISTENT_DISK_TYPE_STANDARD\", \"PERSISTENT_DISK_TYPE_BALANCED\", \"PERSISTENT_DISK_TYPE_SSD\"]"]
    pub fn set_persistent_disk_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.persistent_disk_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_preferences`.\n"]
    pub fn set_machine_preferences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferences,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.machine_preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.machine_preferences = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences {
    type O =
        terrars::BlockAssignable<MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences {}

impl BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences {
    pub fn build(self) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences {
            license_type: core::default::Default::default(),
            persistent_disk_type: core::default::Default::default(),
            machine_preferences: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferencesRef {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `license_type` after provisioning.\nLicense type to consider when calculating costs for virtual machine insights and recommendations. If unspecified, costs are calculated based on the default licensing plan. Possible values: 'LICENSE_TYPE_UNSPECIFIED', 'LICENSE_TYPE_DEFAULT', 'LICENSE_TYPE_BRING_YOUR_OWN_LICENSE'"]
    pub fn license_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `persistent_disk_type` after provisioning.\nPersistent disk type to use. If unspecified (default), all types are considered, based on available usage data. Possible values: [\"PERSISTENT_DISK_TYPE_STANDARD\", \"PERSISTENT_DISK_TYPE_BALANCED\", \"PERSISTENT_DISK_TYPE_SSD\"]"]
    pub fn persistent_disk_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.persistent_disk_type", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_preferences` after provisioning.\n"]
    pub fn machine_preferences(
        &self,
    ) -> terrars
    ::ListRef<MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences_MachinePreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.machine_preferences", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preferred_regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferences {
    #[doc =
        "Set the field `preferred_regions`.\nA list of preferred regions, ordered by the most preferred region first. Set only valid Google Cloud region names. See https://cloud.google.com/compute/docs/regions-zones for available regions."]
    pub fn set_preferred_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.preferred_regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferences {
    type O = terrars::BlockAssignable<MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferences {}

impl BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferences {
    pub fn build(self) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferences {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferences {
            preferred_regions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferencesRef {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `preferred_regions` after provisioning.\nA list of preferred regions, ordered by the most preferred region first. Set only valid Google Cloud region names. See https://cloud.google.com/compute/docs/regions-zones for available regions."]
    pub fn preferred_regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preferred_regions", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypes {
    #[doc =
        "Set the field `node_name`.\nName of the Sole Tenant node. Consult https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes"]
    pub fn set_node_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypes {
    type O =
        terrars
        ::BlockAssignable<MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypes {}

impl BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypes {
    pub fn build(self) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypes {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypes {
            node_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypesRef {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `node_name` after provisioning.\nName of the Sole Tenant node. Consult https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes"]
    pub fn node_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferencesDynamic {
    node_types: std
    ::option
    ::Option<
        terrars::DynamicBlock<MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypes>,
    >,
}

#[derive(Serialize)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commitment_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_overcommit_ratio: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_maintenance_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_types: std
    ::option
    ::Option<std::vec::Vec<MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypes>>,
    dynamic: MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferencesDynamic,
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences {
    #[doc =
        "Set the field `commitment_plan`.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'ON_DEMAND', 'COMMITMENT_1_YEAR', 'COMMITMENT_3_YEAR'"]
    pub fn set_commitment_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commitment_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cpu_overcommit_ratio`.\nCPU overcommit ratio. Acceptable values are between 1.0 and 2.0 inclusive."]
    pub fn set_cpu_overcommit_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_overcommit_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `host_maintenance_policy`.\nSole Tenancy nodes maintenance policy. Possible values: 'HOST_MAINTENANCE_POLICY_UNSPECIFIED', 'HOST_MAINTENANCE_POLICY_DEFAULT', 'HOST_MAINTENANCE_POLICY_RESTART_IN_PLACE', 'HOST_MAINTENANCE_POLICY_MIGRATE_WITHIN_NODE_GROUP'"]
    pub fn set_host_maintenance_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.host_maintenance_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_types`.\n"]
    pub fn set_node_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.node_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.node_types = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences {
    type O = terrars::BlockAssignable<MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences {}

impl BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences {
    pub fn build(self) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences {
            commitment_plan: core::default::Default::default(),
            cpu_overcommit_ratio: core::default::Default::default(),
            host_maintenance_policy: core::default::Default::default(),
            node_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferencesRef {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `commitment_plan` after provisioning.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'ON_DEMAND', 'COMMITMENT_1_YEAR', 'COMMITMENT_3_YEAR'"]
    pub fn commitment_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commitment_plan", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_overcommit_ratio` after provisioning.\nCPU overcommit ratio. Acceptable values are between 1.0 and 2.0 inclusive."]
    pub fn cpu_overcommit_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_overcommit_ratio", self.base))
    }

    #[doc =
        "Get a reference to the value of field `host_maintenance_policy` after provisioning.\nSole Tenancy nodes maintenance policy. Possible values: 'HOST_MAINTENANCE_POLICY_UNSPECIFIED', 'HOST_MAINTENANCE_POLICY_DEFAULT', 'HOST_MAINTENANCE_POLICY_RESTART_IN_PLACE', 'HOST_MAINTENANCE_POLICY_MIGRATE_WITHIN_NODE_GROUP'"]
    pub fn host_maintenance_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_maintenance_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `node_types` after provisioning.\n"]
    pub fn node_types(
        &self,
    ) -> terrars::ListRef<MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences_NodeTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.node_types", self.base))
    }
}

#[derive(Serialize)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commitment_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_overcommit_ratio: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_overcommit_ratio: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_deduplication_compression_ratio: std::option::Option<terrars::PrimField<f64>>,
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferences {
    #[doc =
        "Set the field `commitment_plan`.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'ON_DEMAND', 'COMMITMENT_1_YEAR_MONTHLY_PAYMENTS', 'COMMITMENT_3_YEAR_MONTHLY_PAYMENTS', 'COMMITMENT_1_YEAR_UPFRONT_PAYMENT', 'COMMITMENT_3_YEAR_UPFRONT_PAYMENT',"]
    pub fn set_commitment_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commitment_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `cpu_overcommit_ratio`.\nCPU overcommit ratio. Acceptable values are between 1.0 and 8.0, with 0.1 increment."]
    pub fn set_cpu_overcommit_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_overcommit_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `memory_overcommit_ratio`.\nMemory overcommit ratio. Acceptable values are 1.0, 1.25, 1.5, 1.75 and 2.0."]
    pub fn set_memory_overcommit_ratio(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_overcommit_ratio = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `storage_deduplication_compression_ratio`.\nThe Deduplication and Compression ratio is based on the logical (Used Before) space required to store data before applying deduplication and compression, in relation to the physical (Used After) space required after applying deduplication and compression. Specifically, the ratio is the Used Before space divided by the Used After space. For example, if the Used Before space is 3 GB, but the physical Used After space is 1 GB, the deduplication and compression ratio is 3x. Acceptable values are between 1.0 and 4.0."]
    pub fn set_storage_deduplication_compression_ratio(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.storage_deduplication_compression_ratio = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferences {
    type O =
        terrars::BlockAssignable<MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferences {}

impl BuildMigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferences {
    pub fn build(self) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferences {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferences {
            commitment_plan: core::default::Default::default(),
            cpu_overcommit_ratio: core::default::Default::default(),
            memory_overcommit_ratio: core::default::Default::default(),
            storage_deduplication_compression_ratio: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferencesRef {
        MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `commitment_plan` after provisioning.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'ON_DEMAND', 'COMMITMENT_1_YEAR_MONTHLY_PAYMENTS', 'COMMITMENT_3_YEAR_MONTHLY_PAYMENTS', 'COMMITMENT_1_YEAR_UPFRONT_PAYMENT', 'COMMITMENT_3_YEAR_UPFRONT_PAYMENT',"]
    pub fn commitment_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commitment_plan", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cpu_overcommit_ratio` after provisioning.\nCPU overcommit ratio. Acceptable values are between 1.0 and 8.0, with 0.1 increment."]
    pub fn cpu_overcommit_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_overcommit_ratio", self.base))
    }

    #[doc =
        "Get a reference to the value of field `memory_overcommit_ratio` after provisioning.\nMemory overcommit ratio. Acceptable values are 1.0, 1.25, 1.5, 1.75 and 2.0."]
    pub fn memory_overcommit_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_overcommit_ratio", self.base))
    }

    #[doc =
        "Get a reference to the value of field `storage_deduplication_compression_ratio` after provisioning.\nThe Deduplication and Compression ratio is based on the logical (Used Before) space required to store data before applying deduplication and compression, in relation to the physical (Used After) space required after applying deduplication and compression. Specifically, the ratio is the Used Before space divided by the Used After space. For example, if the Used Before space is 3 GB, but the physical Used After space is 1 GB, the deduplication and compression ratio is 3x. Acceptable values are between 1.0 and 4.0."]
    pub fn storage_deduplication_compression_ratio(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_deduplication_compression_ratio", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct MigrationCenterPreferenceSet_VirtualMachinePreferencesDynamic {
    compute_engine_preferences: std
    ::option
    ::Option<terrars::DynamicBlock<MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences>>,
    region_preferences: std
    ::option
    ::Option<terrars::DynamicBlock<MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferences>>,
    sole_tenancy_preferences: std
    ::option
    ::Option<terrars::DynamicBlock<MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences>>,
    vmware_engine_preferences: std
    ::option
    ::Option<terrars::DynamicBlock<MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferences>>,
}

#[derive(Serialize)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commitment_plan: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sizing_optimization_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_product: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_engine_preferences: std
    ::option
    ::Option<std::vec::Vec<MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_preferences: std
    ::option
    ::Option<std::vec::Vec<MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferences>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sole_tenancy_preferences: std
    ::option
    ::Option<std::vec::Vec<MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vmware_engine_preferences: std
    ::option
    ::Option<std::vec::Vec<MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferences>>,
    dynamic: MigrationCenterPreferenceSet_VirtualMachinePreferencesDynamic,
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferences {
    #[doc =
        "Set the field `commitment_plan`.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'COMMITMENT_PLAN_NONE', 'COMMITMENT_PLAN_ONE_YEAR', 'COMMITMENT_PLAN_THREE_YEARS'"]
    pub fn set_commitment_plan(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commitment_plan = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sizing_optimization_strategy`.\nSizing optimization strategy specifies the preferred strategy used when extrapolating usage data to calculate insights and recommendations for a virtual machine. If you are unsure which value to set, a moderate sizing optimization strategy is often a good value to start with. Possible values: 'SIZING_OPTIMIZATION_STRATEGY_UNSPECIFIED', 'SIZING_OPTIMIZATION_STRATEGY_SAME_AS_SOURCE', 'SIZING_OPTIMIZATION_STRATEGY_MODERATE', 'SIZING_OPTIMIZATION_STRATEGY_AGGRESSIVE'"]
    pub fn set_sizing_optimization_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sizing_optimization_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target_product`.\nTarget product for assets using this preference set. Specify either target product or business goal, but not both. Possible values: 'COMPUTE_MIGRATION_TARGET_PRODUCT_UNSPECIFIED', 'COMPUTE_MIGRATION_TARGET_PRODUCT_COMPUTE_ENGINE', 'COMPUTE_MIGRATION_TARGET_PRODUCT_VMWARE_ENGINE', 'COMPUTE_MIGRATION_TARGET_PRODUCT_SOLE_TENANCY'"]
    pub fn set_target_product(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_product = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_engine_preferences`.\n"]
    pub fn set_compute_engine_preferences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferences,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compute_engine_preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compute_engine_preferences = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `region_preferences`.\n"]
    pub fn set_region_preferences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferences>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.region_preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.region_preferences = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sole_tenancy_preferences`.\n"]
    pub fn set_sole_tenancy_preferences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferences,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sole_tenancy_preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sole_tenancy_preferences = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vmware_engine_preferences`.\n"]
    pub fn set_vmware_engine_preferences(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferences,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vmware_engine_preferences = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vmware_engine_preferences = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MigrationCenterPreferenceSet_VirtualMachinePreferences {
    type O = terrars::BlockAssignable<MigrationCenterPreferenceSet_VirtualMachinePreferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMigrationCenterPreferenceSet_VirtualMachinePreferences {}

impl BuildMigrationCenterPreferenceSet_VirtualMachinePreferences {
    pub fn build(self) -> MigrationCenterPreferenceSet_VirtualMachinePreferences {
        MigrationCenterPreferenceSet_VirtualMachinePreferences {
            commitment_plan: core::default::Default::default(),
            sizing_optimization_strategy: core::default::Default::default(),
            target_product: core::default::Default::default(),
            compute_engine_preferences: core::default::Default::default(),
            region_preferences: core::default::Default::default(),
            sole_tenancy_preferences: core::default::Default::default(),
            vmware_engine_preferences: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MigrationCenterPreferenceSet_VirtualMachinePreferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MigrationCenterPreferenceSet_VirtualMachinePreferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MigrationCenterPreferenceSet_VirtualMachinePreferencesRef {
        MigrationCenterPreferenceSet_VirtualMachinePreferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MigrationCenterPreferenceSet_VirtualMachinePreferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `commitment_plan` after provisioning.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'COMMITMENT_PLAN_NONE', 'COMMITMENT_PLAN_ONE_YEAR', 'COMMITMENT_PLAN_THREE_YEARS'"]
    pub fn commitment_plan(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commitment_plan", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sizing_optimization_strategy` after provisioning.\nSizing optimization strategy specifies the preferred strategy used when extrapolating usage data to calculate insights and recommendations for a virtual machine. If you are unsure which value to set, a moderate sizing optimization strategy is often a good value to start with. Possible values: 'SIZING_OPTIMIZATION_STRATEGY_UNSPECIFIED', 'SIZING_OPTIMIZATION_STRATEGY_SAME_AS_SOURCE', 'SIZING_OPTIMIZATION_STRATEGY_MODERATE', 'SIZING_OPTIMIZATION_STRATEGY_AGGRESSIVE'"]
    pub fn sizing_optimization_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sizing_optimization_strategy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_product` after provisioning.\nTarget product for assets using this preference set. Specify either target product or business goal, but not both. Possible values: 'COMPUTE_MIGRATION_TARGET_PRODUCT_UNSPECIFIED', 'COMPUTE_MIGRATION_TARGET_PRODUCT_COMPUTE_ENGINE', 'COMPUTE_MIGRATION_TARGET_PRODUCT_VMWARE_ENGINE', 'COMPUTE_MIGRATION_TARGET_PRODUCT_SOLE_TENANCY'"]
    pub fn target_product(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_product", self.base))
    }

    #[doc = "Get a reference to the value of field `compute_engine_preferences` after provisioning.\n"]
    pub fn compute_engine_preferences(
        &self,
    ) -> terrars::ListRef<MigrationCenterPreferenceSet_VirtualMachinePreferences_ComputeEnginePreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_engine_preferences", self.base))
    }

    #[doc = "Get a reference to the value of field `region_preferences` after provisioning.\n"]
    pub fn region_preferences(
        &self,
    ) -> terrars::ListRef<MigrationCenterPreferenceSet_VirtualMachinePreferences_RegionPreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.region_preferences", self.base))
    }

    #[doc = "Get a reference to the value of field `sole_tenancy_preferences` after provisioning.\n"]
    pub fn sole_tenancy_preferences(
        &self,
    ) -> terrars::ListRef<MigrationCenterPreferenceSet_VirtualMachinePreferences_SoleTenancyPreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sole_tenancy_preferences", self.base))
    }

    #[doc = "Get a reference to the value of field `vmware_engine_preferences` after provisioning.\n"]
    pub fn vmware_engine_preferences(
        &self,
    ) -> terrars::ListRef<MigrationCenterPreferenceSet_VirtualMachinePreferences_VmwareEnginePreferencesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vmware_engine_preferences", self.base))
    }
}
