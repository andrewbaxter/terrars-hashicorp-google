#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataLossPreventionDiscoveryConfigData_ {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_templates: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    org_config: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_OrgConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    other_cloud_starting_location: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    targets: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataLossPreventionDiscoveryConfig_Timeouts>,
    dynamic: DataLossPreventionDiscoveryConfigDynamic,
}

struct DataLossPreventionDiscoveryConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLossPreventionDiscoveryConfigData_>,
}

#[derive(Clone)]
pub struct DataLossPreventionDiscoveryConfig(std::rc::Rc<DataLossPreventionDiscoveryConfig_>);

impl DataLossPreventionDiscoveryConfig {
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

    #[doc = "Set the field `display_name`.\nDisplay Name (max 1000 Chars)"]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inspect_templates`.\nDetection logic for profile generation"]
    pub fn set_inspect_templates(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().inspect_templates = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `status`.\nRequired. A status for this configuration Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `org_config`.\n"]
    pub fn set_org_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_OrgConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().org_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.org_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `other_cloud_starting_location`.\n"]
    pub fn set_other_cloud_starting_location(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().other_cloud_starting_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.other_cloud_starting_location = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `targets`.\n"]
    pub fn set_targets(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().targets = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.targets = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataLossPreventionDiscoveryConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation timestamp of a DiscoveryConfig."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name (max 1000 Chars)"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `errors` after provisioning.\nOutput only. A stream of errors encountered when the config was activated. Repeated errors may result in the config automatically being paused. Output only field. Will return the last 100 errors. Whenever the config is modified this list will be cleared."]
    pub fn errors(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inspect_templates` after provisioning.\nDetection logic for profile generation"]
    pub fn inspect_templates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_templates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_run_time` after provisioning.\nOutput only. The timestamp of the last time this config was executed"]
    pub fn last_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_run_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation to create the discovery config in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUnique resource name for the DiscoveryConfig, assigned by the service when the DiscoveryConfig is created."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the discovery config in any of the following formats:\n\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nRequired. A status for this configuration Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last update timestamp of a DiscoveryConfig."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `org_config` after provisioning.\n"]
    pub fn org_config(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_OrgConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.org_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `other_cloud_starting_location` after provisioning.\n"]
    pub fn other_cloud_starting_location(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_OtherCloudStartingLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.other_cloud_starting_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_TargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionDiscoveryConfig_TimeoutsRef {
        DataLossPreventionDiscoveryConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataLossPreventionDiscoveryConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataLossPreventionDiscoveryConfig { }

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig {
    type O = terrars::ListRef<DataLossPreventionDiscoveryConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataLossPreventionDiscoveryConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_data_loss_prevention_discovery_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig {
    pub tf_id: std::string::String,
    #[doc = "Location to create the discovery config in."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The parent of the discovery config in any of the following formats:\n\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionDiscoveryConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLossPreventionDiscoveryConfig {
        let out = DataLossPreventionDiscoveryConfig(std::rc::Rc::new(DataLossPreventionDiscoveryConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLossPreventionDiscoveryConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                inspect_templates: core::default::Default::default(),
                location: self.location,
                parent: self.parent,
                status: core::default::Default::default(),
                actions: core::default::Default::default(),
                org_config: core::default::Default::default(),
                other_cloud_starting_location: core::default::Default::default(),
                targets: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataLossPreventionDiscoveryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLossPreventionDiscoveryConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The creation timestamp of a DiscoveryConfig."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nDisplay Name (max 1000 Chars)"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `errors` after provisioning.\nOutput only. A stream of errors encountered when the config was activated. Repeated errors may result in the config automatically being paused. Output only field. Will return the last 100 errors. Whenever the config is modified this list will be cleared."]
    pub fn errors(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_ErrorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.errors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inspect_templates` after provisioning.\nDetection logic for profile generation"]
    pub fn inspect_templates(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_templates", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_run_time` after provisioning.\nOutput only. The timestamp of the last time this config was executed"]
    pub fn last_run_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_run_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation to create the discovery config in."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nUnique resource name for the DiscoveryConfig, assigned by the service when the DiscoveryConfig is created."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the discovery config in any of the following formats:\n\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `status` after provisioning.\nRequired. A status for this configuration Possible values: [\"RUNNING\", \"PAUSED\"]"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The last update timestamp of a DiscoveryConfig."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_ActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `org_config` after provisioning.\n"]
    pub fn org_config(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_OrgConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.org_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `other_cloud_starting_location` after provisioning.\n"]
    pub fn other_cloud_starting_location(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_OtherCloudStartingLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.other_cloud_starting_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `targets` after provisioning.\n"]
    pub fn targets(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_TargetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.targets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionDiscoveryConfig_TimeoutsRef {
        DataLossPreventionDiscoveryConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfigDynamic {
    actions: std::option::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions>>,
    org_config: std::option::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_OrgConfig>>,
    other_cloud_starting_location: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation>>,
    targets: std::option::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Errors_Details {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Errors_Details {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::RecField<terrars::PrimField<std::string::String>>>>,
    ) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Errors_Details {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Errors_Details>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Errors_Details {}

impl BuildDataLossPreventionDiscoveryConfig_Errors_Details {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Errors_Details {
        DataLossPreventionDiscoveryConfig_Errors_Details {
            code: core::default::Default::default(),
            details: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Errors_DetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Errors_DetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Errors_DetailsRef {
        DataLossPreventionDiscoveryConfig_Errors_DetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Errors_DetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<terrars::RecRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Errors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    details: std::option::Option<terrars::ListField<DataLossPreventionDiscoveryConfig_Errors_Details>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Errors {
    #[doc = "Set the field `details`.\n"]
    pub fn set_details(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLossPreventionDiscoveryConfig_Errors_Details>>,
    ) -> Self {
        self.details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timestamp`.\n"]
    pub fn set_timestamp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.timestamp = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Errors {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Errors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Errors {}

impl BuildDataLossPreventionDiscoveryConfig_Errors {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Errors {
        DataLossPreventionDiscoveryConfig_Errors {
            details: core::default::Default::default(),
            timestamp: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_ErrorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_ErrorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionDiscoveryConfig_ErrorsRef {
        DataLossPreventionDiscoveryConfig_ErrorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_ErrorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `details` after provisioning.\n"]
    pub fn details(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Errors_DetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.details", self.base))
    }

    #[doc = "Get a reference to the value of field `timestamp` after provisioning.\n"]
    pub fn timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timestamp", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTable {
    #[doc = "Set the field `dataset_id`.\nDataset Id of the table"]
    pub fn set_dataset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_id`.\nThe Google Cloud Platform project ID of the project containing the table. If omitted, the project ID is inferred from the API call."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_id`.\nName of the table"]
    pub fn set_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTable {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTable {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTable {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTable {
        DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTable {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTableRef {
        DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nDataset Id of the table"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe Google Cloud Platform project ID of the project containing the table. If omitted, the project ID is inferred from the API call."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nName of the table"]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTable {
    #[doc = "Set the field `dataset_id`.\nDataset Id of the table"]
    pub fn set_dataset_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_id`.\nThe Google Cloud Platform project ID of the project containing the table. If omitted, the project ID is inferred from the API call."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_id`.\nName of the table"]
    pub fn set_table_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTable {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTable {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTable {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTable {
        DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTable {
            dataset_id: core::default::Default::default(),
            project_id: core::default::Default::default(),
            table_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTableRef {
        DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nDataset Id of the table"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe Google Cloud Platform project ID of the project containing the table. If omitted, the project ID is inferred from the API call."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nName of the table"]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Actions_ExportDataDynamic {
    profile_table: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTable>>,
    sample_findings_table: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTable>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_ExportData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_table: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_findings_table: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTable>>,
    dynamic: DataLossPreventionDiscoveryConfig_Actions_ExportDataDynamic,
}

impl DataLossPreventionDiscoveryConfig_Actions_ExportData {
    #[doc = "Set the field `profile_table`.\n"]
    pub fn set_profile_table(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTable>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.profile_table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.profile_table = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sample_findings_table`.\n"]
    pub fn set_sample_findings_table(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTable>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sample_findings_table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sample_findings_table = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_ExportData {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_ExportData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_ExportData {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_ExportData {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_ExportData {
        DataLossPreventionDiscoveryConfig_Actions_ExportData {
            profile_table: core::default::Default::default(),
            sample_findings_table: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_ExportDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_ExportDataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_ExportDataRef {
        DataLossPreventionDiscoveryConfig_Actions_ExportDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_ExportDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `profile_table` after provisioning.\n"]
    pub fn profile_table(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_ExportData_ProfileTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profile_table", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_findings_table` after provisioning.\n"]
    pub fn sample_findings_table(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_ExportData_SampleFindingsTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sample_findings_table", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_risk_score: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_sensitivity_score: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_Conditions {
    #[doc =
        "Set the field `minimum_risk_score`.\nThe minimum data risk score that triggers the condition. Possible values: [\"HIGH\", \"MEDIUM_OR_HIGH\"]"]
    pub fn set_minimum_risk_score(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.minimum_risk_score = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `minimum_sensitivity_score`.\nThe minimum sensitivity level that triggers the condition. Possible values: [\"HIGH\", \"MEDIUM_OR_HIGH\"]"]
    pub fn set_minimum_sensitivity_score(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.minimum_sensitivity_score = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_Conditions {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_Conditions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_Conditions {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_Conditions {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_Conditions {
        DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_Conditions {
            minimum_risk_score: core::default::Default::default(),
            minimum_sensitivity_score: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_ConditionsRef {
        DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `minimum_risk_score` after provisioning.\nThe minimum data risk score that triggers the condition. Possible values: [\"HIGH\", \"MEDIUM_OR_HIGH\"]"]
    pub fn minimum_risk_score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_risk_score", self.base))
    }

    #[doc =
        "Get a reference to the value of field `minimum_sensitivity_score` after provisioning.\nThe minimum sensitivity level that triggers the condition. Possible values: [\"HIGH\", \"MEDIUM_OR_HIGH\"]"]
    pub fn minimum_sensitivity_score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_ExpressionsDynamic {
    conditions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_Conditions,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logical_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_Conditions>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_ExpressionsDynamic,
}

impl DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions {
    #[doc =
        "Set the field `logical_operator`.\nThe operator to apply to the collection of conditions Possible values: [\"OR\", \"AND\"]"]
    pub fn set_logical_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.logical_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_Conditions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions {
        DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions {
            logical_operator: core::default::Default::default(),
            conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_ExpressionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_ExpressionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_ExpressionsRef {
        DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_ExpressionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_ExpressionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `logical_operator` after provisioning.\nThe operator to apply to the collection of conditions Possible values: [\"OR\", \"AND\"]"]
    pub fn logical_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logical_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubConditionDynamic {
    expressions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expressions: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions>>,
    dynamic: DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubConditionDynamic,
}

impl DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition {
    #[doc = "Set the field `expressions`.\n"]
    pub fn set_expressions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_Expressions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expressions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expressions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition {
        DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition {
            expressions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubConditionRef {
        DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expressions` after provisioning.\n"]
    pub fn expressions(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition_ExpressionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expressions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Actions_PubSubNotificationDynamic {
    pubsub_condition: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PubSubNotification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detail_of_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_condition: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition>>,
    dynamic: DataLossPreventionDiscoveryConfig_Actions_PubSubNotificationDynamic,
}

impl DataLossPreventionDiscoveryConfig_Actions_PubSubNotification {
    #[doc =
        "Set the field `detail_of_message`.\nHow much data to include in the pub/sub message. Possible values: [\"TABLE_PROFILE\", \"RESOURCE_NAME\"]"]
    pub fn set_detail_of_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.detail_of_message = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `event`.\nThe type of event that triggers a Pub/Sub. At most one PubSubNotification per EventType is permitted. Possible values: [\"NEW_PROFILE\", \"CHANGED_PROFILE\", \"SCORE_INCREASED\", \"ERROR_CHANGED\"]"]
    pub fn set_event(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `topic`.\nCloud Pub/Sub topic to send notifications to. Format is projects/{project}/topics/{topic}."]
    pub fn set_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pubsub_condition`.\n"]
    pub fn set_pubsub_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubCondition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pubsub_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pubsub_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_PubSubNotification {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_PubSubNotification {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_PubSubNotification {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_PubSubNotification {
        DataLossPreventionDiscoveryConfig_Actions_PubSubNotification {
            detail_of_message: core::default::Default::default(),
            event: core::default::Default::default(),
            topic: core::default::Default::default(),
            pubsub_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PubSubNotificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_PubSubNotificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_PubSubNotificationRef {
        DataLossPreventionDiscoveryConfig_Actions_PubSubNotificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_PubSubNotificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `detail_of_message` after provisioning.\nHow much data to include in the pub/sub message. Possible values: [\"TABLE_PROFILE\", \"RESOURCE_NAME\"]"]
    pub fn detail_of_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detail_of_message", self.base))
    }

    #[doc =
        "Get a reference to the value of field `event` after provisioning.\nThe type of event that triggers a Pub/Sub. At most one PubSubNotification per EventType is permitted. Possible values: [\"NEW_PROFILE\", \"CHANGED_PROFILE\", \"SCORE_INCREASED\", \"ERROR_CHANGED\"]"]
    pub fn event(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }

    #[doc =
        "Get a reference to the value of field `topic` after provisioning.\nCloud Pub/Sub topic to send notifications to. Format is projects/{project}/topics/{topic}."]
    pub fn topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic", self.base))
    }

    #[doc = "Get a reference to the value of field `pubsub_condition` after provisioning.\n"]
    pub fn pubsub_condition(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification_PubsubConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pubsub_condition", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PublishToChronicle {}

impl DataLossPreventionDiscoveryConfig_Actions_PublishToChronicle { }

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_PublishToChronicle {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_PublishToChronicle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_PublishToChronicle {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_PublishToChronicle {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_PublishToChronicle {
        DataLossPreventionDiscoveryConfig_Actions_PublishToChronicle {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PublishToChronicleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_PublishToChronicleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_PublishToChronicleRef {
        DataLossPreventionDiscoveryConfig_Actions_PublishToChronicleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_PublishToChronicleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalog {}

impl DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalog { }

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalog {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalog>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalog {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalog {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalog {
        DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalog {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalogRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalogRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalogRef {
        DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalogRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalogRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PublishToScc {}

impl DataLossPreventionDiscoveryConfig_Actions_PublishToScc { }

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_PublishToScc {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_PublishToScc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_PublishToScc {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_PublishToScc {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_PublishToScc {
        DataLossPreventionDiscoveryConfig_Actions_PublishToScc {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_PublishToSccRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_PublishToSccRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_PublishToSccRef {
        DataLossPreventionDiscoveryConfig_Actions_PublishToSccRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_PublishToSccRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScore {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScore { }

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScore {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScore {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\", \"SENSITIVITY_UNKNOWN\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScore {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScore {
        DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScore { score: self.score }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScoreRef {
        DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\", \"SENSITIVITY_UNKNOWN\"]"]
    pub fn score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_Tag {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespaced_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_Tag {
    #[doc =
        "Set the field `namespaced_value`.\nThe namespaced name for the tag value to attach to resources. Must be in the format '{parent_id}/{tag_key_short_name}/{short_name}', for example, \"123456/environment/prod\"."]
    pub fn set_namespaced_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespaced_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_Tag {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_Tag>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_Tag {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_Tag {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_Tag {
        DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_Tag {
            namespaced_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_TagRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_TagRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_TagRef {
        DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_TagRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_TagRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `namespaced_value` after provisioning.\nThe namespaced name for the tag value to attach to resources. Must be in the format '{parent_id}/{tag_key_short_name}/{short_name}', for example, \"123456/environment/prod\"."]
    pub fn namespaced_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespaced_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditionsDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScore>,
    >,
    tag: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_Tag>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_Tag>>,
    dynamic: DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditionsDynamic,
}

impl DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions {
    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScore,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_Tag>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions {
        DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions {
            sensitivity_score: core::default::Default::default(),
            tag: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditionsRef {
        DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions_TagRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Actions_TagResourcesDynamic {
    tag_conditions: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions_TagResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lower_data_risk_to_low: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    profile_generations_to_tag: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_conditions: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions>>,
    dynamic: DataLossPreventionDiscoveryConfig_Actions_TagResourcesDynamic,
}

impl DataLossPreventionDiscoveryConfig_Actions_TagResources {
    #[doc =
        "Set the field `lower_data_risk_to_low`.\nWhether applying a tag to a resource should lower the risk of the profile for that resource. For example, in conjunction with an [IAM deny policy](https://cloud.google.com/iam/docs/deny-overview), you can deny all principals a permission if a tag value is present, mitigating the risk of the resource. This also lowers the data risk of resources at the lower levels of the resource hierarchy. For example, reducing the data risk of a table data profile also reduces the data risk of the constituent column data profiles."]
    pub fn set_lower_data_risk_to_low(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.lower_data_risk_to_low = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `profile_generations_to_tag`.\nThe profile generations for which the tag should be attached to resources. If you attach a tag to only new profiles, then if the sensitivity score of a profile subsequently changes, its tag doesn't change. By default, this field includes only new profiles. To include both new and updated profiles for tagging, this field should explicitly include both 'PROFILE_GENERATION_NEW' and 'PROFILE_GENERATION_UPDATE'. Possible values: [\"PROFILE_GENERATION_NEW\", \"PROFILE_GENERATION_UPDATE\"]"]
    pub fn set_profile_generations_to_tag(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.profile_generations_to_tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_conditions`.\n"]
    pub fn set_tag_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions_TagResources {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_TagResources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions_TagResources {}

impl BuildDataLossPreventionDiscoveryConfig_Actions_TagResources {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions_TagResources {
        DataLossPreventionDiscoveryConfig_Actions_TagResources {
            lower_data_risk_to_low: core::default::Default::default(),
            profile_generations_to_tag: core::default::Default::default(),
            tag_conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Actions_TagResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Actions_TagResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Actions_TagResourcesRef {
        DataLossPreventionDiscoveryConfig_Actions_TagResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Actions_TagResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `lower_data_risk_to_low` after provisioning.\nWhether applying a tag to a resource should lower the risk of the profile for that resource. For example, in conjunction with an [IAM deny policy](https://cloud.google.com/iam/docs/deny-overview), you can deny all principals a permission if a tag value is present, mitigating the risk of the resource. This also lowers the data risk of resources at the lower levels of the resource hierarchy. For example, reducing the data risk of a table data profile also reduces the data risk of the constituent column data profiles."]
    pub fn lower_data_risk_to_low(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower_data_risk_to_low", self.base))
    }

    #[doc =
        "Get a reference to the value of field `profile_generations_to_tag` after provisioning.\nThe profile generations for which the tag should be attached to resources. If you attach a tag to only new profiles, then if the sensitivity score of a profile subsequently changes, its tag doesn't change. By default, this field includes only new profiles. To include both new and updated profiles for tagging, this field should explicitly include both 'PROFILE_GENERATION_NEW' and 'PROFILE_GENERATION_UPDATE'. Possible values: [\"PROFILE_GENERATION_NEW\", \"PROFILE_GENERATION_UPDATE\"]"]
    pub fn profile_generations_to_tag(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.profile_generations_to_tag", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_conditions` after provisioning.\n"]
    pub fn tag_conditions(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_TagResources_TagConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_conditions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_ActionsDynamic {
    export_data: std::option::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_ExportData>>,
    pub_sub_notification: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification>>,
    publish_to_chronicle: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_PublishToChronicle>>,
    publish_to_dataplex_catalog: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalog>>,
    publish_to_scc: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_PublishToScc>>,
    tag_resources: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Actions_TagResources>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Actions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export_data: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_ExportData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pub_sub_notification: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_to_chronicle: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_PublishToChronicle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_to_dataplex_catalog: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalog>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_to_scc: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_PublishToScc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_resources: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Actions_TagResources>>,
    dynamic: DataLossPreventionDiscoveryConfig_ActionsDynamic,
}

impl DataLossPreventionDiscoveryConfig_Actions {
    #[doc = "Set the field `export_data`.\n"]
    pub fn set_export_data(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_ExportData>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.export_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.export_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pub_sub_notification`.\n"]
    pub fn set_pub_sub_notification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_PubSubNotification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pub_sub_notification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pub_sub_notification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `publish_to_chronicle`.\n"]
    pub fn set_publish_to_chronicle(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_PublishToChronicle>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.publish_to_chronicle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_to_chronicle = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `publish_to_dataplex_catalog`.\n"]
    pub fn set_publish_to_dataplex_catalog(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalog>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.publish_to_dataplex_catalog = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_to_dataplex_catalog = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `publish_to_scc`.\n"]
    pub fn set_publish_to_scc(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_PublishToScc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.publish_to_scc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_to_scc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag_resources`.\n"]
    pub fn set_tag_resources(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions_TagResources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_resources = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Actions {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Actions {}

impl BuildDataLossPreventionDiscoveryConfig_Actions {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Actions {
        DataLossPreventionDiscoveryConfig_Actions {
            export_data: core::default::Default::default(),
            pub_sub_notification: core::default::Default::default(),
            publish_to_chronicle: core::default::Default::default(),
            publish_to_dataplex_catalog: core::default::Default::default(),
            publish_to_scc: core::default::Default::default(),
            tag_resources: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_ActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionDiscoveryConfig_ActionsRef {
        DataLossPreventionDiscoveryConfig_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `export_data` after provisioning.\n"]
    pub fn export_data(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_ExportDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.export_data", self.base))
    }

    #[doc = "Get a reference to the value of field `pub_sub_notification` after provisioning.\n"]
    pub fn pub_sub_notification(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_PubSubNotificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pub_sub_notification", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_to_chronicle` after provisioning.\n"]
    pub fn publish_to_chronicle(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_PublishToChronicleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_to_chronicle", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_to_dataplex_catalog` after provisioning.\n"]
    pub fn publish_to_dataplex_catalog(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_PublishToDataplexCatalogRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_to_dataplex_catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_to_scc` after provisioning.\n"]
    pub fn publish_to_scc(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_PublishToSccRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_to_scc", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_resources` after provisioning.\n"]
    pub fn tag_resources(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Actions_TagResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_resources", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_OrgConfig_Location {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    folder_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_OrgConfig_Location {
    #[doc = "Set the field `folder_id`.\nThe ID for the folder within an organization to scan"]
    pub fn set_folder_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.folder_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organization_id`.\nThe ID of an organization to scan"]
    pub fn set_organization_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.organization_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_OrgConfig_Location {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_OrgConfig_Location>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_OrgConfig_Location {}

impl BuildDataLossPreventionDiscoveryConfig_OrgConfig_Location {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_OrgConfig_Location {
        DataLossPreventionDiscoveryConfig_OrgConfig_Location {
            folder_id: core::default::Default::default(),
            organization_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_OrgConfig_LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_OrgConfig_LocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_OrgConfig_LocationRef {
        DataLossPreventionDiscoveryConfig_OrgConfig_LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_OrgConfig_LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `folder_id` after provisioning.\nThe ID for the folder within an organization to scan"]
    pub fn folder_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `organization_id` after provisioning.\nThe ID of an organization to scan"]
    pub fn organization_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_OrgConfigDynamic {
    location: std::option::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_OrgConfig_Location>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_OrgConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_OrgConfig_Location>>,
    dynamic: DataLossPreventionDiscoveryConfig_OrgConfigDynamic,
}

impl DataLossPreventionDiscoveryConfig_OrgConfig {
    #[doc =
        "Set the field `project_id`.\nThe project that will run the scan. The DLP service account that exists within this project must have access to all resources that are profiled, and the cloud DLP API must be enabled."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_OrgConfig_Location>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_OrgConfig {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_OrgConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_OrgConfig {}

impl BuildDataLossPreventionDiscoveryConfig_OrgConfig {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_OrgConfig {
        DataLossPreventionDiscoveryConfig_OrgConfig {
            project_id: core::default::Default::default(),
            location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_OrgConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_OrgConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_OrgConfigRef {
        DataLossPreventionDiscoveryConfig_OrgConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_OrgConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe project that will run the scan. The DLP service account that exists within this project must have access to all resources that are profiled, and the cloud DLP API must be enabled."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_OrgConfig_LocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_asset_inventory_assets: std::option::Option<terrars::PrimField<bool>>,
}

impl DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocation {
    #[doc =
        "Set the field `account_id`.\nThe AWS account ID that this discovery config applies to. Within an organization, you can find the AWS account ID inside an AWS account ARN. Example: arn:<partition>:organizations::<management-account-id>:account/<organization-id>/<account-id>"]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `all_asset_inventory_assets`.\nAll AWS assets stored in Asset Inventory that didn't match other AWS discovery configs."]
    pub fn set_all_asset_inventory_assets(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_asset_inventory_assets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocation {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocation {}

impl BuildDataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocation {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocation {
        DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocation {
            account_id: core::default::Default::default(),
            all_asset_inventory_assets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocationRef {
        DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nThe AWS account ID that this discovery config applies to. Within an organization, you can find the AWS account ID inside an AWS account ARN. Example: arn:<partition>:organizations::<management-account-id>:account/<organization-id>/<account-id>"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `all_asset_inventory_assets` after provisioning.\nAll AWS assets stored in Asset Inventory that didn't match other AWS discovery configs."]
    pub fn all_asset_inventory_assets(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_asset_inventory_assets", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_OtherCloudStartingLocationDynamic {
    aws_location: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocation>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_location: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocation>>,
    dynamic: DataLossPreventionDiscoveryConfig_OtherCloudStartingLocationDynamic,
}

impl DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation {
    #[doc = "Set the field `aws_location`.\n"]
    pub fn set_aws_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_OtherCloudStartingLocation {}

impl BuildDataLossPreventionDiscoveryConfig_OtherCloudStartingLocation {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation {
        DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation {
            aws_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_OtherCloudStartingLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_OtherCloudStartingLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_OtherCloudStartingLocationRef {
        DataLossPreventionDiscoveryConfig_OtherCloudStartingLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_OtherCloudStartingLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_location` after provisioning.\n"]
    pub fn aws_location(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_OtherCloudStartingLocation_AwsLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_location", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frequency: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence {
    #[doc =
        "Set the field `frequency`.\nHow frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.frequency = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence {
            frequency: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadenceRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nHow frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadence {
    #[doc =
        "Set the field `frequency`.\nHow frequently profiles may be updated when schemas are modified. Default to monthly Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `types`.\nThe type of events to consider when deciding if the table's schema has been modified and should have the profile updated. Defaults to NEW_COLUMN. Possible values: [\"SCHEMA_NEW_COLUMNS\", \"SCHEMA_REMOVED_COLUMNS\"]"]
    pub fn set_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadence {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadence {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadence {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadence {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadence {
            frequency: core::default::Default::default(),
            types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadenceRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nHow frequently profiles may be updated when schemas are modified. Default to monthly Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `types` after provisioning.\nThe type of events to consider when deciding if the table's schema has been modified and should have the profile updated. Defaults to NEW_COLUMN. Possible values: [\"SCHEMA_NEW_COLUMNS\", \"SCHEMA_REMOVED_COLUMNS\"]"]
    pub fn types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadence {
    #[doc =
        "Set the field `frequency`.\nHow frequently data profiles can be updated when tables are modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `types`.\nThe type of events to consider when deciding if the table has been modified and should have the profile updated. Defaults to MODIFIED_TIMESTAMP Possible values: [\"TABLE_MODIFIED_TIMESTAMP\"]"]
    pub fn set_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadence {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadence {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadence {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadence {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadence {
            frequency: core::default::Default::default(),
            types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadenceRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nHow frequently data profiles can be updated when tables are modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `types` after provisioning.\nThe type of events to consider when deciding if the table has been modified and should have the profile updated. Defaults to MODIFIED_TIMESTAMP Possible values: [\"TABLE_MODIFIED_TIMESTAMP\"]"]
    pub fn types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_CadenceDynamic {
    inspect_template_modified_cadence: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence,
        >,
    >,
    schema_modified_cadence: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadence>,
    >,
    table_modified_cadence: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadence>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template_modified_cadence: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_modified_cadence: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadence>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_modified_cadence: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadence>>,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_CadenceDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence {
    #[doc =
        "Set the field `refresh_frequency`.\nFrequency at which profiles should be updated, regardless of whether the underlying resource has changed. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_refresh_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inspect_template_modified_cadence`.\n"]
    pub fn set_inspect_template_modified_cadence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inspect_template_modified_cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inspect_template_modified_cadence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_modified_cadence`.\n"]
    pub fn set_schema_modified_cadence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadence,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_modified_cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_modified_cadence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table_modified_cadence`.\n"]
    pub fn set_table_modified_cadence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadence,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table_modified_cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table_modified_cadence = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence {
            refresh_frequency: core::default::Default::default(),
            inspect_template_modified_cadence: core::default::Default::default(),
            schema_modified_cadence: core::default::Default::default(),
            table_modified_cadence: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_CadenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_CadenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_CadenceRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_CadenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_CadenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `refresh_frequency` after provisioning.\nFrequency at which profiles should be updated, regardless of whether the underlying resource has changed. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn refresh_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `inspect_template_modified_cadence` after provisioning.\n"]
    pub fn inspect_template_modified_cadence(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_template_modified_cadence", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_modified_cadence` after provisioning.\n"]
    pub fn schema_modified_cadence(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_SchemaModifiedCadenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_modified_cadence", self.base))
    }

    #[doc = "Get a reference to the value of field `table_modified_cadence` after provisioning.\n"]
    pub fn table_modified_cadence(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_TableModifiedCadenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_modified_cadence", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_age: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_row_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditions {
    #[doc =
        "Set the field `min_age`.\nDuration format. The minimum age a table must have before Cloud DLP can profile it. Value greater than 1."]
    pub fn set_min_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_age = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_row_count`.\nMinimum number of rows that should be present before Cloud DLP profiles as a table."]
    pub fn set_min_row_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_row_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditions {
    type O =
        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditions {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditions {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditions {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditions {
            min_age: core::default::Default::default(),
            min_row_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditionsRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `min_age` after provisioning.\nDuration format. The minimum age a table must have before Cloud DLP can profile it. Value greater than 1."]
    pub fn min_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_age", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_row_count` after provisioning.\nMinimum number of rows that should be present before Cloud DLP profiles as a table."]
    pub fn min_row_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_row_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_Types {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_Types {
    #[doc =
        "Set the field `types`.\nA set of BiqQuery table types Possible values: [\"BIG_QUERY_TABLE_TYPE_TABLE\", \"BIG_QUERY_TABLE_TYPE_EXTERNAL_BIG_LAKE\"]"]
    pub fn set_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_Types {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_Types>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_Types {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_Types {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_Types {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_Types {
            types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_TypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_TypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_TypesRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_TypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_TypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `types` after provisioning.\nA set of BiqQuery table types Possible values: [\"BIG_QUERY_TABLE_TYPE_TABLE\", \"BIG_QUERY_TABLE_TYPE_EXTERNAL_BIG_LAKE\"]"]
    pub fn types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_ConditionsDynamic {
    or_conditions: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditions>>,
    types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_Types>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    type_collection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or_conditions: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    types: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_Types>>,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_ConditionsDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions {
    #[doc =
        "Set the field `created_after`.\nA timestamp in RFC3339 UTC \"Zulu\" format with nanosecond resolution and upto nine fractional digits."]
    pub fn set_created_after(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_after = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_collection`.\nRestrict discovery to categories of table types. Currently view, materialized view, snapshot and non-biglake external tables are supported. Possible values: [\"BIG_QUERY_COLLECTION_ALL_TYPES\", \"BIG_QUERY_COLLECTION_ONLY_SUPPORTED_TYPES\"]"]
    pub fn set_type_collection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_collection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `or_conditions`.\n"]
    pub fn set_or_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.or_conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.or_conditions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `types`.\n"]
    pub fn set_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_Types>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.types = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions {
            created_after: core::default::Default::default(),
            type_collection: core::default::Default::default(),
            or_conditions: core::default::Default::default(),
            types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_ConditionsRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_after` after provisioning.\nA timestamp in RFC3339 UTC \"Zulu\" format with nanosecond resolution and upto nine fractional digits."]
    pub fn created_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_collection` after provisioning.\nRestrict discovery to categories of table types. Currently view, materialized view, snapshot and non-biglake external tables are supported. Possible values: [\"BIG_QUERY_COLLECTION_ALL_TYPES\", \"BIG_QUERY_COLLECTION_ONLY_SUPPORTED_TYPES\"]"]
    pub fn type_collection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type_collection", self.base))
    }

    #[doc = "Get a reference to the value of field `or_conditions` after provisioning.\n"]
    pub fn or_conditions(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_OrConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.or_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions_TypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled {}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled { }

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_DisabledRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_DisabledRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_DisabledRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_DisabledRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_DisabledRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTables {}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTables { }

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTables {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTables {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTables {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTables {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTables {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTablesRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReference {
    dataset_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    table_id: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReference {
    #[doc =
        "Set the field `project_id`.\nThe Google Cloud project ID of the project containing the table.\nIf omitted, the project ID is inferred from the parent project.\nThis field is required if the parent resource is an organization."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReference {
    type O =
        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReference {
    #[doc = "Dataset ID of the table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the table."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReference {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReference {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReference {
            dataset_id: self.dataset_id,
            project_id: core::default::Default::default(),
            table_id: self.table_id,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReferenceRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nDataset ID of the table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe Google Cloud project ID of the project containing the table.\nIf omitted, the project ID is inferred from the parent project.\nThis field is required if the parent resource is an organization."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nName of the table."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_Patterns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dataset_id_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_id_regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_Patterns {
    #[doc = "Set the field `dataset_id_regex`.\nif unset, this property matches all datasets"]
    pub fn set_dataset_id_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dataset_id_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_id_regex`.\nFor organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project."]
    pub fn set_project_id_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_id_regex`.\nif unset, this property matches all tables"]
    pub fn set_table_id_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_id_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_Patterns {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_Patterns,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_Patterns {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_Patterns {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_Patterns {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_Patterns {
            dataset_id_regex: core::default::Default::default(),
            project_id_regex: core::default::Default::default(),
            table_id_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_PatternsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_PatternsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_PatternsRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_PatternsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_PatternsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_id_regex` after provisioning.\nif unset, this property matches all datasets"]
    pub fn dataset_id_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id_regex` after provisioning.\nFor organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project."]
    pub fn project_id_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `table_id_regex` after provisioning.\nif unset, this property matches all tables"]
    pub fn table_id_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id_regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexesDynamic {
    patterns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_Patterns>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patterns: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_Patterns>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexesDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes {
    #[doc = "Set the field `patterns`.\n"]
    pub fn set_patterns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_Patterns,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.patterns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.patterns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes {
            patterns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexesRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `patterns` after provisioning.\n"]
    pub fn patterns(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes_PatternsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.patterns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TablesDynamic {
    include_regexes: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_regexes: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes>>,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TablesDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables {
    #[doc = "Set the field `include_regexes`.\n"]
    pub fn set_include_regexes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_regexes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_regexes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables {
            include_regexes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TablesRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_regexes` after provisioning.\n"]
    pub fn include_regexes(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables_IncludeRegexesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_regexes", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_FilterDynamic {
    other_tables: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTables>>,
    table_reference: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReference>>,
    tables: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    other_tables: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTables>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_reference: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tables: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables>>,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_FilterDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter {
    #[doc = "Set the field `other_tables`.\n"]
    pub fn set_other_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTables>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.other_tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.other_tables = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table_reference`.\n"]
    pub fn set_table_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReference,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table_reference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tables`.\n"]
    pub fn set_tables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_Tables>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tables = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter {
            other_tables: core::default::Default::default(),
            table_reference: core::default::Default::default(),
            tables: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_FilterRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `other_tables` after provisioning.\n"]
    pub fn other_tables(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_OtherTablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.other_tables", self.base))
    }

    #[doc = "Get a reference to the value of field `table_reference` after provisioning.\n"]
    pub fn table_reference(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TableReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `tables` after provisioning.\n"]
    pub fn tables(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter_TablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tables", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTargetDynamic {
    cadence: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence>>,
    conditions: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions>>,
    disabled: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>>,
    filter: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cadence: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter>>,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_BigQueryTargetDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget {
    #[doc = "Set the field `cadence`.\n"]
    pub fn set_cadence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cadence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Conditions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disabled = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disabled = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_BigQueryTarget {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget {
            cadence: core::default::Default::default(),
            conditions: core::default::Default::default(),
            disabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_BigQueryTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_BigQueryTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_BigQueryTargetRef {
        DataLossPreventionDiscoveryConfig_Targets_BigQueryTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_BigQueryTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cadence` after provisioning.\n"]
    pub fn cadence(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_CadenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cadence", self.base))
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_DisabledRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_engines: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Conditions {
    #[doc =
        "Set the field `database_engines`.\nDatabase engines that should be profiled. Optional. Defaults to ALL_SUPPORTED_DATABASE_ENGINES if unspecified. Possible values: [\"ALL_SUPPORTED_DATABASE_ENGINES\", \"MYSQL\", \"POSTGRES\"]"]
    pub fn set_database_engines(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.database_engines = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `types`.\nData profiles will only be generated for the database resource types specified in this field. If not specified, defaults to [DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES]. Possible values: [\"DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES\", \"DATABASE_RESOURCE_TYPE_TABLE\"]"]
    pub fn set_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Conditions {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Conditions {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Conditions {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Conditions {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Conditions {
            database_engines: core::default::Default::default(),
            types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_ConditionsRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database_engines` after provisioning.\nDatabase engines that should be profiled. Optional. Defaults to ALL_SUPPORTED_DATABASE_ENGINES if unspecified. Possible values: [\"ALL_SUPPORTED_DATABASE_ENGINES\", \"MYSQL\", \"POSTGRES\"]"]
    pub fn database_engines(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_engines", self.base))
    }

    #[doc =
        "Get a reference to the value of field `types` after provisioning.\nData profiles will only be generated for the database resource types specified in this field. If not specified, defaults to [DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES]. Possible values: [\"DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES\", \"DATABASE_RESOURCE_TYPE_TABLE\"]"]
    pub fn types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_Patterns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_resource_name_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id_regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_Patterns {
    #[doc = "Set the field `database_regex`.\nRegex to test the database name against. If empty, all databases match."]
    pub fn set_database_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `database_resource_name_regex`.\nRegex to test the database resource's name against. An example of a database resource name is a table's name. Other database resource names like view names could be included in the future. If empty, all database resources match.'"]
    pub fn set_database_resource_name_regex(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.database_resource_name_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_regex`.\nRegex to test the instance name against. If empty, all instances match."]
    pub fn set_instance_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_id_regex`.\nFor organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project."]
    pub fn set_project_id_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_Patterns {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_Patterns,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_Patterns {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_Patterns {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_Patterns {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_Patterns {
            database_regex: core::default::Default::default(),
            database_resource_name_regex: core::default::Default::default(),
            instance_regex: core::default::Default::default(),
            project_id_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_PatternsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_PatternsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_PatternsRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_PatternsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_PatternsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database_regex` after provisioning.\nRegex to test the database name against. If empty, all databases match."]
    pub fn database_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_resource_name_regex` after provisioning.\nRegex to test the database resource's name against. An example of a database resource name is a table's name. Other database resource names like view names could be included in the future. If empty, all database resources match.'"]
    pub fn database_resource_name_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_resource_name_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance_regex` after provisioning.\nRegex to test the instance name against. If empty, all instances match."]
    pub fn instance_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id_regex` after provisioning.\nFor organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project."]
    pub fn project_id_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id_regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexesDynamic {
    patterns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_Patterns,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patterns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_Patterns>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexesDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes {
    #[doc = "Set the field `patterns`.\n"]
    pub fn set_patterns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_Patterns,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.patterns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.patterns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes {
            patterns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexesRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `patterns` after provisioning.\n"]
    pub fn patterns(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes_PatternsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.patterns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_CollectionDynamic {
    include_regexes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_regexes: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_CollectionDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection {
    #[doc = "Set the field `include_regexes`.\n"]
    pub fn set_include_regexes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_regexes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_regexes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection {
            include_regexes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_CollectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_CollectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_CollectionRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_CollectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_CollectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_regexes` after provisioning.\n"]
    pub fn include_regexes(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection_IncludeRegexesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_regexes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReference {
    database: terrars::PrimField<std::string::String>,
    database_resource: terrars::PrimField<std::string::String>,
    instance: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReference { }

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReference {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReference {
    #[doc = "Required. Name of a database within the instance."]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = "Required. Name of a database resource, for example, a table within the database."]
    pub database_resource: terrars::PrimField<std::string::String>,
    #[doc = "Required. The instance where this resource is located. For example: Cloud SQL instance ID."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc = "Required. If within a project-level config, then this must match the config's project ID."]
    pub project_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReference {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReference {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReference {
            database: self.database,
            database_resource: self.database_resource,
            instance: self.instance,
            project_id: self.project_id,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReferenceRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nRequired. Name of a database within the instance."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc =
        "Get a reference to the value of field `database_resource` after provisioning.\nRequired. Name of a database resource, for example, a table within the database."]
    pub fn database_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_resource", self.base))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nRequired. The instance where this resource is located. For example: Cloud SQL instance ID."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nRequired. If within a project-level config, then this must match the config's project ID."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others {}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others { }

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_OthersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_OthersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_OthersRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_OthersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_OthersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_FilterDynamic {
    collection: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection>>,
    database_resource_reference: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReference>,
    >,
    others: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_resource_reference: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReference>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    others: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others>>,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_FilterDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter {
    #[doc = "Set the field `collection`.\n"]
    pub fn set_collection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Collection>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.collection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.collection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `database_resource_reference`.\n"]
    pub fn set_database_resource_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReference,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.database_resource_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.database_resource_reference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `others`.\n"]
    pub fn set_others(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.others = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.others = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter {
            collection: core::default::Default::default(),
            database_resource_reference: core::default::Default::default(),
            others: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_FilterRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection` after provisioning.\n"]
    pub fn collection(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_CollectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collection", self.base))
    }

    #[doc = "Get a reference to the value of field `database_resource_reference` after provisioning.\n"]
    pub fn database_resource_reference(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_DatabaseResourceReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_resource_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `others` after provisioning.\n"]
    pub fn others(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_OthersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.others", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadence {
    frequency: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadence { }

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadence {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadence,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadence {
    #[doc =
        "How frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub frequency: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadence {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadence {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadence {
            frequency: self.frequency,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadenceRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nHow frequently data profiles can be updated when the template is modified. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadence {
    #[doc =
        "Set the field `frequency`.\nFrequency to regenerate data profiles when the schema is modified. Defaults to monthly. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `types`.\nThe types of schema modifications to consider. Defaults to NEW_COLUMNS. Possible values: [\"NEW_COLUMNS\", \"REMOVED_COLUMNS\"]"]
    pub fn set_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadence {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadence,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadence {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadence {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadence {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadence {
            frequency: core::default::Default::default(),
            types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadenceRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `frequency` after provisioning.\nFrequency to regenerate data profiles when the schema is modified. Defaults to monthly. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `types` after provisioning.\nThe types of schema modifications to consider. Defaults to NEW_COLUMNS. Possible values: [\"NEW_COLUMNS\", \"REMOVED_COLUMNS\"]"]
    pub fn types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadenceDynamic {
    inspect_template_modified_cadence: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadence,
        >,
    >,
    schema_modified_cadence: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadence,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template_modified_cadence: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadence,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_modified_cadence: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadence>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadenceDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence {
    #[doc =
        "Set the field `refresh_frequency`.\nData changes (non-schema changes) in Cloud SQL tables can't trigger reprofiling. If you set this field, profiles are refreshed at this frequency regardless of whether the underlying tables have changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_refresh_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inspect_template_modified_cadence`.\n"]
    pub fn set_inspect_template_modified_cadence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadence,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inspect_template_modified_cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inspect_template_modified_cadence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schema_modified_cadence`.\n"]
    pub fn set_schema_modified_cadence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadence,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_modified_cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_modified_cadence = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence {
            refresh_frequency: core::default::Default::default(),
            inspect_template_modified_cadence: core::default::Default::default(),
            schema_modified_cadence: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadenceRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `refresh_frequency` after provisioning.\nData changes (non-schema changes) in Cloud SQL tables can't trigger reprofiling. If you set this field, profiles are refreshed at this frequency regardless of whether the underlying tables have changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn refresh_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `inspect_template_modified_cadence` after provisioning.\n"]
    pub fn inspect_template_modified_cadence(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_InspectTemplateModifiedCadenceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_template_modified_cadence", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_modified_cadence` after provisioning.\n"]
    pub fn schema_modified_cadence(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence_SchemaModifiedCadenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_modified_cadence", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTargetDynamic {
    conditions: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Conditions>>,
    disabled: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>>,
    filter: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter>>,
    generation_cadence: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation_cadence: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence>>,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudSqlTargetDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Conditions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disabled = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disabled = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generation_cadence`.\n"]
    pub fn set_generation_cadence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadence>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generation_cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generation_cadence = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget {
            conditions: core::default::Default::default(),
            disabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            generation_cadence: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudSqlTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudSqlTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudSqlTargetRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudSqlTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudSqlTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_DisabledRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `generation_cadence` after provisioning.\n"]
    pub fn generation_cadence(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_GenerationCadenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generation_cadence", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_bucket_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_object_attributes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditions {
    #[doc =
        "Set the field `included_bucket_attributes`.\nOnly objects with the specified attributes will be scanned. Defaults to [ALL_SUPPORTED_BUCKETS] if unset. Possible values: [\"ALL_SUPPORTED_BUCKETS\", \"AUTOCLASS_DISABLED\", \"AUTOCLASS_ENABLED\"]"]
    pub fn set_included_bucket_attributes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_bucket_attributes = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `included_object_attributes`.\nOnly objects with the specified attributes will be scanned. If an object has one of the specified attributes but is inside an excluded bucket, it will not be scanned. Defaults to [ALL_SUPPORTED_OBJECTS]. A profile will be created even if no objects match the included_object_attributes. Possible values: [\"ALL_SUPPORTED_OBJECTS\", \"STANDARD\", \"NEARLINE\", \"COLDLINE\", \"ARCHIVE\", \"REGIONAL\", \"MULTI_REGIONAL\", \"DURABLE_REDUCED_AVAILABILITY\"]"]
    pub fn set_included_object_attributes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_object_attributes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditions {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditions {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditions {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditions {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditions {
            included_bucket_attributes: core::default::Default::default(),
            included_object_attributes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditionsRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `included_bucket_attributes` after provisioning.\nOnly objects with the specified attributes will be scanned. Defaults to [ALL_SUPPORTED_BUCKETS] if unset. Possible values: [\"ALL_SUPPORTED_BUCKETS\", \"AUTOCLASS_DISABLED\", \"AUTOCLASS_ENABLED\"]"]
    pub fn included_bucket_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_bucket_attributes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `included_object_attributes` after provisioning.\nOnly objects with the specified attributes will be scanned. If an object has one of the specified attributes but is inside an excluded bucket, it will not be scanned. Defaults to [ALL_SUPPORTED_OBJECTS]. A profile will be created even if no objects match the included_object_attributes. Possible values: [\"ALL_SUPPORTED_OBJECTS\", \"STANDARD\", \"NEARLINE\", \"COLDLINE\", \"ARCHIVE\", \"REGIONAL\", \"MULTI_REGIONAL\", \"DURABLE_REDUCED_AVAILABILITY\"]"]
    pub fn included_object_attributes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_object_attributes", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_ConditionsDynamic {
    cloud_storage_conditions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditions>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_age: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_conditions: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditions>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_ConditionsDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions {
    #[doc =
        "Set the field `created_after`.\nFile store must have been created after this date. Used to avoid backfilling. A timestamp in RFC3339 UTC \"Zulu\" format with nanosecond resolution and upto nine fractional digits."]
    pub fn set_created_after(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_after = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_age`.\nDuration format. Minimum age a file store must have. If set, the value must be 1 hour or greater."]
    pub fn set_min_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloud_storage_conditions`.\n"]
    pub fn set_cloud_storage_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions {
            created_after: core::default::Default::default(),
            min_age: core::default::Default::default(),
            cloud_storage_conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_ConditionsRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `created_after` after provisioning.\nFile store must have been created after this date. Used to avoid backfilling. A timestamp in RFC3339 UTC \"Zulu\" format with nanosecond resolution and upto nine fractional digits."]
    pub fn created_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_age` after provisioning.\nDuration format. Minimum age a file store must have. If set, the value must be 1 hour or greater."]
    pub fn min_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_age", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_storage_conditions` after provisioning.\n"]
    pub fn cloud_storage_conditions(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions_CloudStorageConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_conditions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReference {
    #[doc = "Set the field `bucket_name`.\nThe bucket to scan."]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_id`.\nIf within a project-level config, then this must match the config's project id."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReference {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReference,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReference {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReference {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReference {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReference {
            bucket_name: core::default::Default::default(),
            project_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReferenceRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe bucket to scan."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nIf within a project-level config, then this must match the config's project id."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id_regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegex {
    #[doc =
        "Set the field `bucket_name_regex`.\nRegex to test the bucket name against. If empty, all buckets match. Example: \"marketing2021\" or \"(marketing)\\d{4}\" will both match the bucket gs://marketing2021"]
    pub fn set_bucket_name_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project_id_regex`.\nFor organizations, if unset, will match all projects."]
    pub fn set_project_id_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegex {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegex,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegex {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegex {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegex {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegex {
            bucket_name_regex: core::default::Default::default(),
            project_id_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegexRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegexRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegexRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_name_regex` after provisioning.\nRegex to test the bucket name against. If empty, all buckets match. Example: \"marketing2021\" or \"(marketing)\\d{4}\" will both match the bucket gs://marketing2021"]
    pub fn bucket_name_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id_regex` after provisioning.\nFor organizations, if unset, will match all projects."]
    pub fn project_id_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id_regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_PatternsDynamic {
    cloud_storage_regex: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegex,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_regex: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegex,
        >,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_PatternsDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns {
    #[doc = "Set the field `cloud_storage_regex`.\n"]
    pub fn set_cloud_storage_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegex,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_regex = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns {
            cloud_storage_regex: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_PatternsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_PatternsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_PatternsRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_PatternsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_PatternsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_regex` after provisioning.\n"]
    pub fn cloud_storage_regex(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns_CloudStorageRegexRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexesDynamic {
    patterns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patterns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexesDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes {
    #[doc = "Set the field `patterns`.\n"]
    pub fn set_patterns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_Patterns,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.patterns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.patterns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes {
            patterns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexesRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `patterns` after provisioning.\n"]
    pub fn patterns(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes_PatternsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.patterns", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFilters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespaced_tag_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespaced_tag_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFilters {
    #[doc =
        "Set the field `namespaced_tag_key`.\nThe namespaced name for the tag key. Must be in the format\n'{parent_id}/{tag_key_short_name}', for example, \"123456/sensitive\" for\nan organization parent, or \"my-project/sensitive\" for a project parent."]
    pub fn set_namespaced_tag_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.namespaced_tag_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `namespaced_tag_value`.\nThe namespaced name for the tag value. Must be in the format\n'{parent_id}/{tag_key_short_name}/{short_name}', for example,\n\"123456/environment/prod\" for an organization parent, or\n\"my-project/environment/prod\" for a project parent."]
    pub fn set_namespaced_tag_value(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.namespaced_tag_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFilters {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFilters,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFilters {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFilters {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFilters {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFilters {
            namespaced_tag_key: core::default::Default::default(),
            namespaced_tag_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFiltersRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `namespaced_tag_key` after provisioning.\nThe namespaced name for the tag key. Must be in the format\n'{parent_id}/{tag_key_short_name}', for example, \"123456/sensitive\" for\nan organization parent, or \"my-project/sensitive\" for a project parent."]
    pub fn namespaced_tag_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespaced_tag_key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespaced_tag_value` after provisioning.\nThe namespaced name for the tag value. Must be in the format\n'{parent_id}/{tag_key_short_name}/{short_name}', for example,\n\"123456/environment/prod\" for an organization parent, or\n\"my-project/environment/prod\" for a project parent."]
    pub fn namespaced_tag_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespaced_tag_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTagsDynamic {
    tag_filters: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFilters,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_filters: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFilters>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTagsDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags {
    #[doc = "Set the field `tag_filters`.\n"]
    pub fn set_tag_filters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFilters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_filters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags {
            tag_filters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTagsRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tag_filters` after provisioning.\n"]
    pub fn tag_filters(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags_TagFiltersRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_filters", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CollectionDynamic {
    include_regexes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes>,
    >,
    include_tags: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_regexes: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_tags: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CollectionDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection {
    #[doc = "Set the field `include_regexes`.\n"]
    pub fn set_include_regexes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_regexes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_regexes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include_tags`.\n"]
    pub fn set_include_tags(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTags,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection {
    type O =
        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection {
            include_regexes: core::default::Default::default(),
            include_tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CollectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CollectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CollectionRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CollectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CollectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_regexes` after provisioning.\n"]
    pub fn include_regexes(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeRegexesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_regexes", self.base))
    }

    #[doc = "Get a reference to the value of field `include_tags` after provisioning.\n"]
    pub fn include_tags(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection_IncludeTagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_FilterDynamic {
    cloud_storage_resource_reference: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReference,
        >,
    >,
    collection: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection>>,
    others: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_resource_reference: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReference>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    others: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others>>,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_FilterDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter {
    #[doc = "Set the field `cloud_storage_resource_reference`.\n"]
    pub fn set_cloud_storage_resource_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReference,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_resource_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_resource_reference = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `collection`.\n"]
    pub fn set_collection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_Collection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.collection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.collection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `others`.\n"]
    pub fn set_others(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.others = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.others = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter {
            cloud_storage_resource_reference: core::default::Default::default(),
            collection: core::default::Default::default(),
            others: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_FilterRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_resource_reference` after provisioning.\n"]
    pub fn cloud_storage_resource_reference(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CloudStorageResourceReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_resource_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `collection` after provisioning.\n"]
    pub fn collection(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter_CollectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collection", self.base))
    }

    #[doc = "Get a reference to the value of field `others` after provisioning.\n"]
    pub fn others(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_OthersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.others", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadenceDynamic {
    inspect_template_modified_cadence: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template_modified_cadence: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadenceDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadence {
    #[doc =
        "Set the field `refresh_frequency`.\nData changes in Cloud Storage can't trigger reprofiling. If you set this field, profiles are refreshed at this frequency regardless of whether the underlying buckets have changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_refresh_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inspect_template_modified_cadence`.\n"]
    pub fn set_inspect_template_modified_cadence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inspect_template_modified_cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inspect_template_modified_cadence = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadence {
    type O =
        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadence {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadence {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadence {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadence {
            refresh_frequency: core::default::Default::default(),
            inspect_template_modified_cadence: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadenceRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `refresh_frequency` after provisioning.\nData changes in Cloud Storage can't trigger reprofiling. If you set this field, profiles are refreshed at this frequency regardless of whether the underlying buckets have changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn refresh_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `inspect_template_modified_cadence` after provisioning.\n"]
    pub fn inspect_template_modified_cadence(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_template_modified_cadence", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTargetDynamic {
    conditions: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions>>,
    disabled: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>>,
    filter: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter>>,
    generation_cadence: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadence>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation_cadence: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadence>>,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_CloudStorageTargetDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Conditions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disabled = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disabled = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_Filter>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generation_cadence`.\n"]
    pub fn set_generation_cadence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadence,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generation_cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generation_cadence = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget {
            conditions: core::default::Default::default(),
            disabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            generation_cadence: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_CloudStorageTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_CloudStorageTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_CloudStorageTargetRef {
        DataLossPreventionDiscoveryConfig_Targets_CloudStorageTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_CloudStorageTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_DisabledRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `generation_cadence` after provisioning.\n"]
    pub fn generation_cadence(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget_GenerationCadenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generation_cadence", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_storage_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditions {
    #[doc =
        "Set the field `bucket_types`.\nBucket types that should be profiled. Optional. Defaults to TYPE_ALL_SUPPORTED if unspecified. Possible values: [\"TYPE_ALL_SUPPORTED\", \"TYPE_GENERAL_PURPOSE\"]"]
    pub fn set_bucket_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.bucket_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `object_storage_classes`.\nObject classes that should be profiled. Optional. Defaults to ALL_SUPPORTED_CLASSES if unspecified. Possible values: [\"ALL_SUPPORTED_CLASSES\", \"STANDARD\", \"STANDARD_INFREQUENT_ACCESS\", \"GLACIER_INSTANT_RETRIEVAL\", \"INTELLIGENT_TIERING\"]"]
    pub fn set_object_storage_classes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.object_storage_classes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditions {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditions,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditions {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditions {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditions {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditions {
            bucket_types: core::default::Default::default(),
            object_storage_classes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditionsRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_types` after provisioning.\nBucket types that should be profiled. Optional. Defaults to TYPE_ALL_SUPPORTED if unspecified. Possible values: [\"TYPE_ALL_SUPPORTED\", \"TYPE_GENERAL_PURPOSE\"]"]
    pub fn bucket_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `object_storage_classes` after provisioning.\nObject classes that should be profiled. Optional. Defaults to ALL_SUPPORTED_CLASSES if unspecified. Possible values: [\"ALL_SUPPORTED_CLASSES\", \"STANDARD\", \"STANDARD_INFREQUENT_ACCESS\", \"GLACIER_INSTANT_RETRIEVAL\", \"INTELLIGENT_TIERING\"]"]
    pub fn object_storage_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.object_storage_classes", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_ConditionsDynamic {
    amazon_s3_bucket_conditions: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditions>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_age: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_s3_bucket_conditions: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditions>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_ConditionsDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions {
    #[doc =
        "Set the field `min_age`.\nDuration format.  Minimum age a resource must be before a profile can be generated. Value must be 1 hour or greater. Minimum age is not supported for Azure Blob Storage containers."]
    pub fn set_min_age(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_age = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `amazon_s3_bucket_conditions`.\n"]
    pub fn set_amazon_s3_bucket_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditions,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_s3_bucket_conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_s3_bucket_conditions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions {
            min_age: core::default::Default::default(),
            amazon_s3_bucket_conditions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_ConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_ConditionsRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `min_age` after provisioning.\nDuration format.  Minimum age a resource must be before a profile can be generated. Value must be 1 hour or greater. Minimum age is not supported for Azure Blob Storage containers."]
    pub fn min_age(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_age", self.base))
    }

    #[doc = "Get a reference to the value of field `amazon_s3_bucket_conditions` after provisioning.\n"]
    pub fn amazon_s3_bucket_conditions(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions_AmazonS3BucketConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_s3_bucket_conditions", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceType {
    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceType {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceType {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceType {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceType {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceType {
            data_source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceTypeRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id_regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegex {
    #[doc =
        "Set the field `account_id_regex`.\nRegex to test the AWS account ID against. If empty, all accounts match. Example: arn:aws:organizations::123:account/o-b2c3d4/345"]
    pub fn set_account_id_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id_regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegex {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegex,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegex {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegex {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegex {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegex {
            account_id_regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegexRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegexRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegexRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id_regex` after provisioning.\nRegex to test the AWS account ID against. If empty, all accounts match. Example: arn:aws:organizations::123:account/o-b2c3d4/345"]
    pub fn account_id_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id_regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegexDynamic {
    aws_account_regex: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegex,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_regex: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegex,
        >,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegexDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex {
    #[doc = "Set the field `bucket_name_regex`.\nRegex to test the bucket name against. If empty, all buckets match."]
    pub fn set_bucket_name_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_account_regex`.\n"]
    pub fn set_aws_account_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegex,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_account_regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_account_regex = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex {
            bucket_name_regex: core::default::Default::default(),
            aws_account_regex: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegexRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegexRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegexRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_name_regex` after provisioning.\nRegex to test the bucket name against. If empty, all buckets match."]
    pub fn bucket_name_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_account_regex` after provisioning.\n"]
    pub fn aws_account_regex(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex_AwsAccountRegexRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_account_regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_PatternsDynamic {
    amazon_s3_bucket_regex: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_s3_bucket_regex: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex,
        >,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_PatternsDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns {
    #[doc = "Set the field `amazon_s3_bucket_regex`.\n"]
    pub fn set_amazon_s3_bucket_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegex,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_s3_bucket_regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_s3_bucket_regex = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns {
            amazon_s3_bucket_regex: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_PatternsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_PatternsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_PatternsRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_PatternsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_PatternsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_s3_bucket_regex` after provisioning.\n"]
    pub fn amazon_s3_bucket_regex(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns_AmazonS3BucketRegexRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_s3_bucket_regex", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexesDynamic {
    patterns: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patterns: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexesDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes {
    #[doc = "Set the field `patterns`.\n"]
    pub fn set_patterns(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_Patterns,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.patterns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.patterns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes {
            patterns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexesRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `patterns` after provisioning.\n"]
    pub fn patterns(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes_PatternsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.patterns", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_CollectionDynamic {
    include_regexes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_regexes: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_CollectionDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection {
    #[doc = "Set the field `include_regexes`.\n"]
    pub fn set_include_regexes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_regexes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_regexes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection {
            include_regexes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_CollectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_CollectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_CollectionRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_CollectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_CollectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_regexes` after provisioning.\n"]
    pub fn include_regexes(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection_IncludeRegexesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_regexes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccount {
    #[doc = "Set the field `account_id`.\nAWS account ID."]
    pub fn set_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccount {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccount,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccount {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccount {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccount {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccount {
            account_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccountRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nAWS account ID."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3BucketDynamic {
    aws_account: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccount,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccount,
        >,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3BucketDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket {
    #[doc = "Set the field `bucket_name`.\nThe bucket name."]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_account`.\n"]
    pub fn set_aws_account(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccount,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_account = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_account = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket {
    pub fn build(
        self,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket {
            bucket_name: core::default::Default::default(),
            aws_account: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3BucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3BucketRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3BucketRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3BucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3BucketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe bucket name."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_account` after provisioning.\n"]
    pub fn aws_account(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket_AwsAccountRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_account", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResourceDynamic {
    amazon_s3_bucket: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_s3_bucket: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResourceDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource {
    #[doc = "Set the field `amazon_s3_bucket`.\n"]
    pub fn set_amazon_s3_bucket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3Bucket,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_s3_bucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_s3_bucket = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource {
    type O =
        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource {
            amazon_s3_bucket: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResourceRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_s3_bucket` after provisioning.\n"]
    pub fn amazon_s3_bucket(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource_AmazonS3BucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_s3_bucket", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_FilterDynamic {
    collection: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection>>,
    others: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others>>,
    single_resource: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    collection: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    others: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_resource: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource>>,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_FilterDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter {
    #[doc = "Set the field `collection`.\n"]
    pub fn set_collection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_Collection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.collection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.collection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `others`.\n"]
    pub fn set_others(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_Others>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.others = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.others = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `single_resource`.\n"]
    pub fn set_single_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.single_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.single_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter {
            collection: core::default::Default::default(),
            others: core::default::Default::default(),
            single_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_FilterRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `collection` after provisioning.\n"]
    pub fn collection(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_CollectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.collection", self.base))
    }

    #[doc = "Get a reference to the value of field `others` after provisioning.\n"]
    pub fn others(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget_Filter_OthersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.others", self.base))
    }

    #[doc = "Get a reference to the value of field `single_resource` after provisioning.\n"]
    pub fn single_resource(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter_SingleResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadenceDynamic {
    inspect_template_modified_cadence: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template_modified_cadence: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence>,
    >,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadenceDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadence {
    #[doc =
        "Set the field `refresh_frequency`.\nFrequency to update profiles regardless of whether the underlying resource has changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn set_refresh_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inspect_template_modified_cadence`.\n"]
    pub fn set_inspect_template_modified_cadence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadence,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inspect_template_modified_cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inspect_template_modified_cadence = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadence {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadence {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadence {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadence {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadence {
            refresh_frequency: core::default::Default::default(),
            inspect_template_modified_cadence: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadenceRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `refresh_frequency` after provisioning.\nFrequency to update profiles regardless of whether the underlying resource has changes. Defaults to never. Possible values: [\"UPDATE_FREQUENCY_NEVER\", \"UPDATE_FREQUENCY_DAILY\", \"UPDATE_FREQUENCY_MONTHLY\"]"]
    pub fn refresh_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `inspect_template_modified_cadence` after provisioning.\n"]
    pub fn inspect_template_modified_cadence(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Cadence_InspectTemplateModifiedCadenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_template_modified_cadence", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTargetDynamic {
    conditions: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions>>,
    data_source_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceType>>,
    disabled: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>>,
    filter: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter>>,
    generation_cadence: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadence>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation_cadence: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadence>>,
    dynamic: DataLossPreventionDiscoveryConfig_Targets_OtherCloudTargetDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Conditions>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_source_type`.\n"]
    pub fn set_data_source_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceType>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_source_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_source_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_Disabled>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disabled = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disabled = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_Filter>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generation_cadence`.\n"]
    pub fn set_generation_cadence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadence,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generation_cadence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generation_cadence = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget {
            conditions: core::default::Default::default(),
            data_source_type: core::default::Default::default(),
            disabled: core::default::Default::default(),
            filter: core::default::Default::default(),
            generation_cadence: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_OtherCloudTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_OtherCloudTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_OtherCloudTargetRef {
        DataLossPreventionDiscoveryConfig_Targets_OtherCloudTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_OtherCloudTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `data_source_type` after provisioning.\n"]
    pub fn data_source_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_DataSourceTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget_DisabledRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `generation_cadence` after provisioning.\n"]
    pub fn generation_cadence(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget_GenerationCadenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.generation_cadence", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets_SecretsTarget {}

impl DataLossPreventionDiscoveryConfig_Targets_SecretsTarget { }

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets_SecretsTarget {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_SecretsTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets_SecretsTarget {}

impl BuildDataLossPreventionDiscoveryConfig_Targets_SecretsTarget {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets_SecretsTarget {
        DataLossPreventionDiscoveryConfig_Targets_SecretsTarget {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_Targets_SecretsTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_Targets_SecretsTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionDiscoveryConfig_Targets_SecretsTargetRef {
        DataLossPreventionDiscoveryConfig_Targets_SecretsTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_Targets_SecretsTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionDiscoveryConfig_TargetsDynamic {
    big_query_target: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget>>,
    cloud_sql_target: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget>>,
    cloud_storage_target: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget>>,
    other_cloud_target: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget>>,
    secrets_target: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionDiscoveryConfig_Targets_SecretsTarget>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Targets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    big_query_target: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_sql_target: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_target: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    other_cloud_target: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_target: std::option::Option<std::vec::Vec<DataLossPreventionDiscoveryConfig_Targets_SecretsTarget>>,
    dynamic: DataLossPreventionDiscoveryConfig_TargetsDynamic,
}

impl DataLossPreventionDiscoveryConfig_Targets {
    #[doc = "Set the field `big_query_target`.\n"]
    pub fn set_big_query_target(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_BigQueryTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.big_query_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.big_query_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_sql_target`.\n"]
    pub fn set_cloud_sql_target(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_sql_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_sql_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_storage_target`.\n"]
    pub fn set_cloud_storage_target(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `other_cloud_target`.\n"]
    pub fn set_other_cloud_target(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.other_cloud_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.other_cloud_target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secrets_target`.\n"]
    pub fn set_secrets_target(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets_SecretsTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secrets_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secrets_target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Targets {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Targets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Targets {}

impl BuildDataLossPreventionDiscoveryConfig_Targets {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Targets {
        DataLossPreventionDiscoveryConfig_Targets {
            big_query_target: core::default::Default::default(),
            cloud_sql_target: core::default::Default::default(),
            cloud_storage_target: core::default::Default::default(),
            other_cloud_target: core::default::Default::default(),
            secrets_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_TargetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_TargetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionDiscoveryConfig_TargetsRef {
        DataLossPreventionDiscoveryConfig_TargetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_TargetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `big_query_target` after provisioning.\n"]
    pub fn big_query_target(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_BigQueryTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.big_query_target", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_sql_target` after provisioning.\n"]
    pub fn cloud_sql_target(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudSqlTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_sql_target", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_storage_target` after provisioning.\n"]
    pub fn cloud_storage_target(
        &self,
    ) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_CloudStorageTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_target", self.base))
    }

    #[doc = "Get a reference to the value of field `other_cloud_target` after provisioning.\n"]
    pub fn other_cloud_target(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_OtherCloudTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.other_cloud_target", self.base))
    }

    #[doc = "Get a reference to the value of field `secrets_target` after provisioning.\n"]
    pub fn secrets_target(&self) -> terrars::ListRef<DataLossPreventionDiscoveryConfig_Targets_SecretsTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets_target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionDiscoveryConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionDiscoveryConfig_Timeouts {
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

impl terrars::ToListMappable for DataLossPreventionDiscoveryConfig_Timeouts {
    type O = terrars::BlockAssignable<DataLossPreventionDiscoveryConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionDiscoveryConfig_Timeouts {}

impl BuildDataLossPreventionDiscoveryConfig_Timeouts {
    pub fn build(self) -> DataLossPreventionDiscoveryConfig_Timeouts {
        DataLossPreventionDiscoveryConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionDiscoveryConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionDiscoveryConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionDiscoveryConfig_TimeoutsRef {
        DataLossPreventionDiscoveryConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionDiscoveryConfig_TimeoutsRef {
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
