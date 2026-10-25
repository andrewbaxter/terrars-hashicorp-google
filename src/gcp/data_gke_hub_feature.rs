#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataGkeHubFeatureData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataGkeHubFeature_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataGkeHubFeatureData_>,
}

#[derive(Clone)]
pub struct DataGkeHubFeature(std::rc::Rc<DataGkeHubFeature_>);

impl DataGkeHubFeature {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. When the Feature resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. When the Feature resource was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fleet_default_member_config` after provisioning.\nOptional. Fleet Default Membership Configuration."]
    pub fn fleet_default_member_config(&self) -> terrars::ListRef<DataGkeHubFeature_FleetDefaultMemberConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet_default_member_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nGCP labels for this Feature.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full, unique name of this Feature resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_state` after provisioning.\nState of the Feature resource itself."]
    pub fn resource_state(&self) -> terrars::ListRef<DataGkeHubFeature_ResourceStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `spec` after provisioning.\nOptional. Hub-wide Feature configuration. If this Feature does not support any Hub-wide configuration, this field may be unused."]
    pub fn spec(&self) -> terrars::ListRef<DataGkeHubFeature_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The Hub-wide Feature state"]
    pub fn state(&self) -> terrars::ListRef<DataGkeHubFeature_StateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. When the Feature resource was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataGkeHubFeature {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataGkeHubFeature { }

impl terrars::ToListMappable for DataGkeHubFeature {
    type O = terrars::ListRef<DataGkeHubFeatureRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataGkeHubFeature_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_gke_hub_feature".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGkeHubFeature {
    pub tf_id: std::string::String,
    #[doc = "The location for the resource"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The full, unique name of this Feature resource"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataGkeHubFeature {
    pub fn build(self, stack: &mut terrars::Stack) -> DataGkeHubFeature {
        let out = DataGkeHubFeature(std::rc::Rc::new(DataGkeHubFeature_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataGkeHubFeatureData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGkeHubFeatureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeatureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataGkeHubFeatureRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nOutput only. When the Feature resource was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `delete_time` after provisioning.\nOutput only. When the Feature resource was deleted."]
    pub fn delete_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `fleet_default_member_config` after provisioning.\nOptional. Fleet Default Membership Configuration."]
    pub fn fleet_default_member_config(&self) -> terrars::ListRef<DataGkeHubFeature_FleetDefaultMemberConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet_default_member_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nGCP labels for this Feature.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the resource"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full, unique name of this Feature resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_state` after provisioning.\nState of the Feature resource itself."]
    pub fn resource_state(&self) -> terrars::ListRef<DataGkeHubFeature_ResourceStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `spec` after provisioning.\nOptional. Hub-wide Feature configuration. If this Feature does not support any Hub-wide configuration, this field may be unused."]
    pub fn spec(&self) -> terrars::ListRef<DataGkeHubFeature_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. The Hub-wide Feature state"]
    pub fn state(&self) -> terrars::ListRef<DataGkeHubFeature_StateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nOutput only. When the Feature resource was last updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Git {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_proxy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_branch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_repo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_rev: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_wait_secs: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Git {
    #[doc = "Set the field `gcp_service_account_email`.\n"]
    pub fn set_gcp_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `https_proxy`.\n"]
    pub fn set_https_proxy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.https_proxy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_dir`.\n"]
    pub fn set_policy_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_type`.\n"]
    pub fn set_secret_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_branch`.\n"]
    pub fn set_sync_branch(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_branch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_repo`.\n"]
    pub fn set_sync_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_repo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_rev`.\n"]
    pub fn set_sync_rev(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_rev = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_wait_secs`.\n"]
    pub fn set_sync_wait_secs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_wait_secs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Git {
    type O = terrars::BlockAssignable<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Git>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Git {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Git {
    pub fn build(self) -> DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Git {
        DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Git {
            gcp_service_account_email: core::default::Default::default(),
            https_proxy: core::default::Default::default(),
            policy_dir: core::default::Default::default(),
            secret_type: core::default::Default::default(),
            sync_branch: core::default::Default::default(),
            sync_repo: core::default::Default::default(),
            sync_rev: core::default::Default::default(),
            sync_wait_secs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_GitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_GitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_GitRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_GitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_GitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcp_service_account_email` after provisioning.\n"]
    pub fn gcp_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `https_proxy` after provisioning.\n"]
    pub fn https_proxy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.https_proxy", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_dir` after provisioning.\n"]
    pub fn policy_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_type` after provisioning.\n"]
    pub fn secret_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }

    #[doc = "Get a reference to the value of field `sync_branch` after provisioning.\n"]
    pub fn sync_branch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_branch", self.base))
    }

    #[doc = "Get a reference to the value of field `sync_repo` after provisioning.\n"]
    pub fn sync_repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }

    #[doc = "Get a reference to the value of field `sync_rev` after provisioning.\n"]
    pub fn sync_rev(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_rev", self.base))
    }

    #[doc = "Get a reference to the value of field `sync_wait_secs` after provisioning.\n"]
    pub fn sync_wait_secs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_wait_secs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Oci {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_repo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_wait_secs: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Oci {
    #[doc = "Set the field `gcp_service_account_email`.\n"]
    pub fn set_gcp_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.gcp_service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_dir`.\n"]
    pub fn set_policy_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_type`.\n"]
    pub fn set_secret_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_repo`.\n"]
    pub fn set_sync_repo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_repo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_wait_secs`.\n"]
    pub fn set_sync_wait_secs(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sync_wait_secs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Oci {
    type O = terrars::BlockAssignable<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Oci>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Oci {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Oci {
    pub fn build(self) -> DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Oci {
        DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Oci {
            gcp_service_account_email: core::default::Default::default(),
            policy_dir: core::default::Default::default(),
            secret_type: core::default::Default::default(),
            sync_repo: core::default::Default::default(),
            sync_wait_secs: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_OciRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_OciRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_OciRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_OciRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_OciRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gcp_service_account_email` after provisioning.\n"]
    pub fn gcp_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_dir` after provisioning.\n"]
    pub fn policy_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_type` after provisioning.\n"]
    pub fn secret_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }

    #[doc = "Get a reference to the value of field `sync_repo` after provisioning.\n"]
    pub fn sync_repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_repo", self.base))
    }

    #[doc = "Get a reference to the value of field `sync_wait_secs` after provisioning.\n"]
    pub fn sync_wait_secs(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sync_wait_secs", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std
    ::option
    ::Option<terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics_gcp_service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oci: std
    ::option
    ::Option<terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Oci>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prevent_drift: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_format: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Git>,
                    >,
    ) -> Self {
        self.git = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metrics_gcp_service_account_email`.\n"]
    pub fn set_metrics_gcp_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.metrics_gcp_service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oci`.\n"]
    pub fn set_oci(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_Oci>,
                    >,
    ) -> Self {
        self.oci = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prevent_drift`.\n"]
    pub fn set_prevent_drift(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.prevent_drift = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_format`.\n"]
    pub fn set_source_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_format = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync {
    type O = terrars::BlockAssignable<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync {
    pub fn build(self) -> DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync {
        DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync {
            enabled: core::default::Default::default(),
            git: core::default::Default::default(),
            metrics_gcp_service_account_email: core::default::Default::default(),
            oci: core::default::Default::default(),
            prevent_drift: core::default::Default::default(),
            source_format: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSyncRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSyncRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSyncRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSyncRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSyncRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics_gcp_service_account_email` after provisioning.\n"]
    pub fn metrics_gcp_service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metrics_gcp_service_account_email", self.base))
    }

    #[doc = "Get a reference to the value of field `oci` after provisioning.\n"]
    pub fn oci(&self) -> terrars::ListRef<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync_OciRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oci", self.base))
    }

    #[doc = "Get a reference to the value of field `prevent_drift` after provisioning.\n"]
    pub fn prevent_drift(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prevent_drift", self.base))
    }

    #[doc = "Get a reference to the value of field `source_format` after provisioning.\n"]
    pub fn source_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_format", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_sync: std
    ::option
    ::Option<terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement {
    #[doc = "Set the field `config_sync`.\n"]
    pub fn set_config_sync(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSync>,
                    >,
    ) -> Self {
        self.config_sync = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement {
    type O = terrars::BlockAssignable<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement {
    pub fn build(self) -> DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement {
        DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement {
            config_sync: core::default::Default::default(),
            management: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_ConfigmanagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_ConfigmanagementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_ConfigmanagementRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_ConfigmanagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_ConfigmanagementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config_sync` after provisioning.\n"]
    pub fn config_sync(
        &self,
    ) -> terrars::ListRef<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement_ConfigSyncRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_sync", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Mesh {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Mesh {
    #[doc = "Set the field `management`.\n"]
    pub fn set_management(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.management = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Mesh {
    type O = terrars::BlockAssignable<DataGkeHubFeature_FleetDefaultMemberConfig_Mesh>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Mesh {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Mesh {
    pub fn build(self) -> DataGkeHubFeature_FleetDefaultMemberConfig_Mesh {
        DataGkeHubFeature_FleetDefaultMemberConfig_Mesh { management: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_MeshRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_MeshRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_MeshRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_MeshRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_MeshRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.management", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
    type O =
        terrars
        ::BlockAssignable<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
    pub fn build(
        self,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits {
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
    type O =
        terrars
        ::BlockAssignable<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
    pub fn build(
        self,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests {
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests,
        >,
    >,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Limits,
                        >,
                    >,
    ) -> Self {
        self.limits = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requests`.\n"]
    pub fn set_requests(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_Requests,
                        >,
                    >,
    ) -> Self {
        self.requests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
    type O =
        terrars
        ::BlockAssignable<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
    pub fn build(
        self,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources {
            limits: core::default::Default::default(),
            requests: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(
        &self,
    ) -> terrars
    ::ListRef<
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_LimitsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `requests` after provisioning.\n"]
    pub fn requests(
        &self,
    ) -> terrars
    ::ListRef<
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources_RequestsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.requests", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodToleration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effect: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodToleration {
    #[doc = "Set the field `effect`.\n"]
    pub fn set_effect(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.effect = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodToleration {
    type O =
        terrars
        ::BlockAssignable<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodToleration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodToleration {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodToleration {
    pub fn build(
        self,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodToleration {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodToleration {
            effect: core::default::Default::default(),
            key: core::default::Default::default(),
            operator: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `effect` after provisioning.\n"]
    pub fn effect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.effect", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_resources: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_affinity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pod_toleration: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodToleration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replica_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
    #[doc = "Set the field `component`.\n"]
    pub fn set_component(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_resources`.\n"]
    pub fn set_container_resources(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResources,
                        >,
                    >,
    ) -> Self {
        self.container_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_affinity`.\n"]
    pub fn set_pod_affinity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pod_affinity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pod_toleration`.\n"]
    pub fn set_pod_toleration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodToleration,
                        >,
                    >,
    ) -> Self {
        self.pod_toleration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replica_count`.\n"]
    pub fn set_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.replica_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
    type O =
        terrars
        ::BlockAssignable<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
    pub fn build(
        self,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs {
            component: core::default::Default::default(),
            container_resources: core::default::Default::default(),
            pod_affinity: core::default::Default::default(),
            pod_toleration: core::default::Default::default(),
            replica_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `container_resources` after provisioning.\n"]
    pub fn container_resources(
        &self,
    ) -> terrars
    ::ListRef<
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_ContainerResourcesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_resources", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_affinity` after provisioning.\n"]
    pub fn pod_affinity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pod_affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `pod_toleration` after provisioning.\n"]
    pub fn pod_toleration(
        &self,
    ) -> terrars
    ::ListRef<
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs_PodTolerationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pod_toleration", self.base))
    }

    #[doc = "Get a reference to the value of field `replica_count` after provisioning.\n"]
    pub fn replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replica_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_Monitoring {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backends: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_Monitoring {
    #[doc = "Set the field `backends`.\n"]
    pub fn set_backends(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.backends = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_Monitoring {
    type O =
        terrars
        ::BlockAssignable<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_Monitoring,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_Monitoring {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_Monitoring {
    pub fn build(
        self,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_Monitoring {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_Monitoring {
            backends: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_MonitoringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_MonitoringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_MonitoringRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_MonitoringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_MonitoringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backends` after provisioning.\n"]
    pub fn backends(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backends", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bundle: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exempted_namespaces: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
    #[doc = "Set the field `bundle`.\n"]
    pub fn set_bundle(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bundle = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exempted_namespaces`.\n"]
    pub fn set_exempted_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exempted_namespaces = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
    type O =
        terrars
        ::BlockAssignable<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
    pub fn build(
        self,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles {
            bundle: core::default::Default::default(),
            exempted_namespaces: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_BundlesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_BundlesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_BundlesRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_BundlesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_BundlesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bundle` after provisioning.\n"]
    pub fn bundle(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle", self.base))
    }

    #[doc = "Get a reference to the value of field `exempted_namespaces` after provisioning.\n"]
    pub fn exempted_namespaces(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exempted_namespaces", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    installation: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
    #[doc = "Set the field `installation`.\n"]
    pub fn set_installation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.installation = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
    type O =
        terrars
        ::BlockAssignable<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
    pub fn build(
        self,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary {
            installation: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `installation` after provisioning.\n"]
    pub fn installation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.installation", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bundles: std
    ::option
    ::Option<
        terrars
        ::SetField<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_library: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary,
        >,
    >,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent {
    #[doc = "Set the field `bundles`.\n"]
    pub fn set_bundles(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_Bundles,
                        >,
                    >,
    ) -> Self {
        self.bundles = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `template_library`.\n"]
    pub fn set_template_library(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibrary,
                        >,
                    >,
    ) -> Self {
        self.template_library = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent {
    type O =
        terrars
        ::BlockAssignable<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent {
    pub fn build(
        self,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent {
            bundles: core::default::Default::default(),
            template_library: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContentRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bundles` after provisioning.\n"]
    pub fn bundles(
        &self,
    ) -> terrars
    ::SetRef<
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_BundlesRef,
    > {
        terrars::SetRef::new(self.shared().clone(), format!("{}.bundles", self.base))
    }

    #[doc = "Get a reference to the value of field `template_library` after provisioning.\n"]
    pub fn template_library(
        &self,
    ) -> terrars
    ::ListRef<
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent_TemplateLibraryRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template_library", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_interval_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constraint_violation_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_configs: std
    ::option
    ::Option<
        terrars
        ::SetField<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exemptable_namespaces: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    install_spec: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_denies_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring: std
    ::option
    ::Option<
        terrars
        ::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_Monitoring>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mutation_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_content: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    referential_rules_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig {
    #[doc = "Set the field `audit_interval_seconds`.\n"]
    pub fn set_audit_interval_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.audit_interval_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `constraint_violation_limit`.\n"]
    pub fn set_constraint_violation_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.constraint_violation_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_configs`.\n"]
    pub fn set_deployment_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigs,
                        >,
                    >,
    ) -> Self {
        self.deployment_configs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exemptable_namespaces`.\n"]
    pub fn set_exemptable_namespaces(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exemptable_namespaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `install_spec`.\n"]
    pub fn set_install_spec(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.install_spec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_denies_enabled`.\n"]
    pub fn set_log_denies_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.log_denies_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_Monitoring,
                        >,
                    >,
    ) -> Self {
        self.monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mutation_enabled`.\n"]
    pub fn set_mutation_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.mutation_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_content`.\n"]
    pub fn set_policy_content(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContent,
                        >,
                    >,
    ) -> Self {
        self.policy_content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `referential_rules_enabled`.\n"]
    pub fn set_referential_rules_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.referential_rules_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig {
    type O =
        terrars
        ::BlockAssignable<DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig {
    pub fn build(self) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig {
            audit_interval_seconds: core::default::Default::default(),
            constraint_violation_limit: core::default::Default::default(),
            deployment_configs: core::default::Default::default(),
            exemptable_namespaces: core::default::Default::default(),
            install_spec: core::default::Default::default(),
            log_denies_enabled: core::default::Default::default(),
            monitoring: core::default::Default::default(),
            mutation_enabled: core::default::Default::default(),
            policy_content: core::default::Default::default(),
            referential_rules_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfigRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit_interval_seconds` after provisioning.\n"]
    pub fn audit_interval_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_interval_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `constraint_violation_limit` after provisioning.\n"]
    pub fn constraint_violation_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constraint_violation_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment_configs` after provisioning.\n"]
    pub fn deployment_configs(
        &self,
    ) -> terrars
    ::SetRef<
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_DeploymentConfigsRef,
    > {
        terrars::SetRef::new(self.shared().clone(), format!("{}.deployment_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `exemptable_namespaces` after provisioning.\n"]
    pub fn exemptable_namespaces(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exemptable_namespaces", self.base))
    }

    #[doc = "Get a reference to the value of field `install_spec` after provisioning.\n"]
    pub fn install_spec(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.install_spec", self.base))
    }

    #[doc = "Get a reference to the value of field `log_denies_enabled` after provisioning.\n"]
    pub fn log_denies_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_denies_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(
        &self,
    ) -> terrars
    ::ListRef<DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_MonitoringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }

    #[doc = "Get a reference to the value of field `mutation_enabled` after provisioning.\n"]
    pub fn mutation_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mutation_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_content` after provisioning.\n"]
    pub fn policy_content(
        &self,
    ) -> terrars
    ::ListRef<DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig_PolicyContentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_content", self.base))
    }

    #[doc = "Get a reference to the value of field `referential_rules_enabled` after provisioning.\n"]
    pub fn referential_rules_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.referential_rules_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_controller_hub_config: std
    ::option
    ::Option<
        terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller {
    #[doc = "Set the field `policy_controller_hub_config`.\n"]
    pub fn set_policy_controller_hub_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfig,
                        >,
                    >,
    ) -> Self {
        self.policy_controller_hub_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller {
    type O = terrars::BlockAssignable<DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller {
    pub fn build(self) -> DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller {
        DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller {
            policy_controller_hub_config: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig_PolicycontrollerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_FleetDefaultMemberConfig_PolicycontrollerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_FleetDefaultMemberConfig_PolicycontrollerRef {
        DataGkeHubFeature_FleetDefaultMemberConfig_PolicycontrollerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfig_PolicycontrollerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy_controller_hub_config` after provisioning.\n"]
    pub fn policy_controller_hub_config(
        &self,
    ) -> terrars::ListRef<DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller_PolicyControllerHubConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_controller_hub_config", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configmanagement: std
    ::option
    ::Option<terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh: std::option::Option<terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Mesh>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policycontroller: std
    ::option
    ::Option<terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller>>,
}

impl DataGkeHubFeature_FleetDefaultMemberConfig {
    #[doc = "Set the field `configmanagement`.\n"]
    pub fn set_configmanagement(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Configmanagement>>,
    ) -> Self {
        self.configmanagement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mesh`.\n"]
    pub fn set_mesh(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Mesh>>,
    ) -> Self {
        self.mesh = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policycontroller`.\n"]
    pub fn set_policycontroller(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_FleetDefaultMemberConfig_Policycontroller>>,
    ) -> Self {
        self.policycontroller = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_FleetDefaultMemberConfig {
    type O = terrars::BlockAssignable<DataGkeHubFeature_FleetDefaultMemberConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_FleetDefaultMemberConfig {}

impl BuildDataGkeHubFeature_FleetDefaultMemberConfig {
    pub fn build(self) -> DataGkeHubFeature_FleetDefaultMemberConfig {
        DataGkeHubFeature_FleetDefaultMemberConfig {
            configmanagement: core::default::Default::default(),
            mesh: core::default::Default::default(),
            policycontroller: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_FleetDefaultMemberConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_FleetDefaultMemberConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGkeHubFeature_FleetDefaultMemberConfigRef {
        DataGkeHubFeature_FleetDefaultMemberConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_FleetDefaultMemberConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configmanagement` after provisioning.\n"]
    pub fn configmanagement(&self) -> terrars::ListRef<DataGkeHubFeature_FleetDefaultMemberConfig_ConfigmanagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configmanagement", self.base))
    }

    #[doc = "Get a reference to the value of field `mesh` after provisioning.\n"]
    pub fn mesh(&self) -> terrars::ListRef<DataGkeHubFeature_FleetDefaultMemberConfig_MeshRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mesh", self.base))
    }

    #[doc = "Get a reference to the value of field `policycontroller` after provisioning.\n"]
    pub fn policycontroller(&self) -> terrars::ListRef<DataGkeHubFeature_FleetDefaultMemberConfig_PolicycontrollerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policycontroller", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_ResourceState {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    has_resources: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_ResourceState {
    #[doc = "Set the field `has_resources`.\n"]
    pub fn set_has_resources(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.has_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_ResourceState {
    type O = terrars::BlockAssignable<DataGkeHubFeature_ResourceState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_ResourceState {}

impl BuildDataGkeHubFeature_ResourceState {
    pub fn build(self) -> DataGkeHubFeature_ResourceState {
        DataGkeHubFeature_ResourceState {
            has_resources: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_ResourceStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_ResourceStateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGkeHubFeature_ResourceStateRef {
        DataGkeHubFeature_ResourceStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_ResourceStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `has_resources` after provisioning.\n"]
    pub fn has_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.has_resources", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec_Clusterupgrade_PostConditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    soaking: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_Spec_Clusterupgrade_PostConditions {
    #[doc = "Set the field `soaking`.\n"]
    pub fn set_soaking(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.soaking = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec_Clusterupgrade_PostConditions {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec_Clusterupgrade_PostConditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec_Clusterupgrade_PostConditions {}

impl BuildDataGkeHubFeature_Spec_Clusterupgrade_PostConditions {
    pub fn build(self) -> DataGkeHubFeature_Spec_Clusterupgrade_PostConditions {
        DataGkeHubFeature_Spec_Clusterupgrade_PostConditions { soaking: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_Spec_Clusterupgrade_PostConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_Spec_Clusterupgrade_PostConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_Spec_Clusterupgrade_PostConditionsRef {
        DataGkeHubFeature_Spec_Clusterupgrade_PostConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_Spec_Clusterupgrade_PostConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `soaking` after provisioning.\n"]
    pub fn soaking(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.soaking", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_Upgrade {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_Upgrade {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_Upgrade {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_Upgrade>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_Upgrade {}

impl BuildDataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_Upgrade {
    pub fn build(self) -> DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_Upgrade {
        DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_Upgrade {
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_UpgradeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_UpgradeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_UpgradeRef {
        DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_UpgradeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_UpgradeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_conditions: std::option::Option<terrars::ListField<DataGkeHubFeature_Spec_Clusterupgrade_PostConditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upgrade: std::option::Option<terrars::ListField<DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_Upgrade>>,
}

impl DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides {
    #[doc = "Set the field `post_conditions`.\n"]
    pub fn set_post_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_Spec_Clusterupgrade_PostConditions>>,
    ) -> Self {
        self.post_conditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upgrade`.\n"]
    pub fn set_upgrade(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_Upgrade>>,
    ) -> Self {
        self.upgrade = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides {}

impl BuildDataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides {
    pub fn build(self) -> DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides {
        DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides {
            post_conditions: core::default::Default::default(),
            upgrade: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverridesRef {
        DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `post_conditions` after provisioning.\n"]
    pub fn post_conditions(&self) -> terrars::ListRef<DataGkeHubFeature_Spec_Clusterupgrade_PostConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `upgrade` after provisioning.\n"]
    pub fn upgrade(&self) -> terrars::ListRef<DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides_UpgradeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upgrade", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec_Clusterupgrade {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gke_upgrade_overrides: std
    ::option
    ::Option<terrars::ListField<DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    post_conditions: std::option::Option<terrars::ListField<DataGkeHubFeature_Spec_Clusterupgrade_PostConditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upstream_fleets: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataGkeHubFeature_Spec_Clusterupgrade {
    #[doc = "Set the field `gke_upgrade_overrides`.\n"]
    pub fn set_gke_upgrade_overrides(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverrides>>,
    ) -> Self {
        self.gke_upgrade_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `post_conditions`.\n"]
    pub fn set_post_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_Spec_Clusterupgrade_PostConditions>>,
    ) -> Self {
        self.post_conditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upstream_fleets`.\n"]
    pub fn set_upstream_fleets(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.upstream_fleets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec_Clusterupgrade {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec_Clusterupgrade>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec_Clusterupgrade {}

impl BuildDataGkeHubFeature_Spec_Clusterupgrade {
    pub fn build(self) -> DataGkeHubFeature_Spec_Clusterupgrade {
        DataGkeHubFeature_Spec_Clusterupgrade {
            gke_upgrade_overrides: core::default::Default::default(),
            post_conditions: core::default::Default::default(),
            upstream_fleets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_Spec_ClusterupgradeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_Spec_ClusterupgradeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGkeHubFeature_Spec_ClusterupgradeRef {
        DataGkeHubFeature_Spec_ClusterupgradeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_Spec_ClusterupgradeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gke_upgrade_overrides` after provisioning.\n"]
    pub fn gke_upgrade_overrides(
        &self,
    ) -> terrars::ListRef<DataGkeHubFeature_Spec_Clusterupgrade_GkeUpgradeOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gke_upgrade_overrides", self.base))
    }

    #[doc = "Get a reference to the value of field `post_conditions` after provisioning.\n"]
    pub fn post_conditions(&self) -> terrars::ListRef<DataGkeHubFeature_Spec_Clusterupgrade_PostConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.post_conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `upstream_fleets` after provisioning.\n"]
    pub fn upstream_fleets(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upstream_fleets", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfig {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfig {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfig {}

impl BuildDataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfig {
    pub fn build(self) -> DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfig {
        DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfig {
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfigRef {
        DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfig {
    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfig {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfig {}

impl BuildDataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfig {
    pub fn build(self) -> DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfig {
        DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfig {
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfigRef {
        DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_config: std
    ::option
    ::Option<terrars::ListField<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet_scope_logs_config: std
    ::option
    ::Option<terrars::ListField<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfig>>,
}

impl DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig {
    #[doc = "Set the field `default_config`.\n"]
    pub fn set_default_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfig>>,
    ) -> Self {
        self.default_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fleet_scope_logs_config`.\n"]
    pub fn set_fleet_scope_logs_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfig>,
                    >,
    ) -> Self {
        self.fleet_scope_logs_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec_Fleetobservability_LoggingConfig {}

impl BuildDataGkeHubFeature_Spec_Fleetobservability_LoggingConfig {
    pub fn build(self) -> DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig {
        DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig {
            default_config: core::default::Default::default(),
            fleet_scope_logs_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_Spec_Fleetobservability_LoggingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_Spec_Fleetobservability_LoggingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_Spec_Fleetobservability_LoggingConfigRef {
        DataGkeHubFeature_Spec_Fleetobservability_LoggingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_Spec_Fleetobservability_LoggingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_config` after provisioning.\n"]
    pub fn default_config(
        &self,
    ) -> terrars::ListRef<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_DefaultConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_config", self.base))
    }

    #[doc = "Get a reference to the value of field `fleet_scope_logs_config` after provisioning.\n"]
    pub fn fleet_scope_logs_config(
        &self,
    ) -> terrars::ListRef<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig_FleetScopeLogsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleet_scope_logs_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec_Fleetobservability {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_config: std::option::Option<terrars::ListField<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig>>,
}

impl DataGkeHubFeature_Spec_Fleetobservability {
    #[doc = "Set the field `logging_config`.\n"]
    pub fn set_logging_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfig>>,
    ) -> Self {
        self.logging_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec_Fleetobservability {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec_Fleetobservability>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec_Fleetobservability {}

impl BuildDataGkeHubFeature_Spec_Fleetobservability {
    pub fn build(self) -> DataGkeHubFeature_Spec_Fleetobservability {
        DataGkeHubFeature_Spec_Fleetobservability { logging_config: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_Spec_FleetobservabilityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_Spec_FleetobservabilityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGkeHubFeature_Spec_FleetobservabilityRef {
        DataGkeHubFeature_Spec_FleetobservabilityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_Spec_FleetobservabilityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `logging_config` after provisioning.\n"]
    pub fn logging_config(&self) -> terrars::ListRef<DataGkeHubFeature_Spec_Fleetobservability_LoggingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec_Multiclusteringress {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_membership: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_Spec_Multiclusteringress {
    #[doc = "Set the field `config_membership`.\n"]
    pub fn set_config_membership(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.config_membership = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec_Multiclusteringress {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec_Multiclusteringress>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec_Multiclusteringress {}

impl BuildDataGkeHubFeature_Spec_Multiclusteringress {
    pub fn build(self) -> DataGkeHubFeature_Spec_Multiclusteringress {
        DataGkeHubFeature_Spec_Multiclusteringress { config_membership: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_Spec_MulticlusteringressRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_Spec_MulticlusteringressRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGkeHubFeature_Spec_MulticlusteringressRef {
        DataGkeHubFeature_Spec_MulticlusteringressRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_Spec_MulticlusteringressRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config_membership` after provisioning.\n"]
    pub fn config_membership(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_membership", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec_Rbacrolebindingactuation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_custom_roles: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataGkeHubFeature_Spec_Rbacrolebindingactuation {
    #[doc = "Set the field `allowed_custom_roles`.\n"]
    pub fn set_allowed_custom_roles(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_custom_roles = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec_Rbacrolebindingactuation {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec_Rbacrolebindingactuation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec_Rbacrolebindingactuation {}

impl BuildDataGkeHubFeature_Spec_Rbacrolebindingactuation {
    pub fn build(self) -> DataGkeHubFeature_Spec_Rbacrolebindingactuation {
        DataGkeHubFeature_Spec_Rbacrolebindingactuation { allowed_custom_roles: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_Spec_RbacrolebindingactuationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_Spec_RbacrolebindingactuationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGkeHubFeature_Spec_RbacrolebindingactuationRef {
        DataGkeHubFeature_Spec_RbacrolebindingactuationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_Spec_RbacrolebindingactuationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_custom_roles` after provisioning.\n"]
    pub fn allowed_custom_roles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_custom_roles", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec_Workloadidentity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope_tenancy_pool: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_Spec_Workloadidentity {
    #[doc = "Set the field `scope_tenancy_pool`.\n"]
    pub fn set_scope_tenancy_pool(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.scope_tenancy_pool = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec_Workloadidentity {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec_Workloadidentity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec_Workloadidentity {}

impl BuildDataGkeHubFeature_Spec_Workloadidentity {
    pub fn build(self) -> DataGkeHubFeature_Spec_Workloadidentity {
        DataGkeHubFeature_Spec_Workloadidentity { scope_tenancy_pool: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_Spec_WorkloadidentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_Spec_WorkloadidentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGkeHubFeature_Spec_WorkloadidentityRef {
        DataGkeHubFeature_Spec_WorkloadidentityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_Spec_WorkloadidentityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scope_tenancy_pool` after provisioning.\n"]
    pub fn scope_tenancy_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope_tenancy_pool", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    clusterupgrade: std::option::Option<terrars::ListField<DataGkeHubFeature_Spec_Clusterupgrade>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleetobservability: std::option::Option<terrars::ListField<DataGkeHubFeature_Spec_Fleetobservability>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiclusteringress: std::option::Option<terrars::ListField<DataGkeHubFeature_Spec_Multiclusteringress>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rbacrolebindingactuation: std::option::Option<terrars::ListField<DataGkeHubFeature_Spec_Rbacrolebindingactuation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workloadidentity: std::option::Option<terrars::ListField<DataGkeHubFeature_Spec_Workloadidentity>>,
}

impl DataGkeHubFeature_Spec {
    #[doc = "Set the field `clusterupgrade`.\n"]
    pub fn set_clusterupgrade(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_Spec_Clusterupgrade>>,
    ) -> Self {
        self.clusterupgrade = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fleetobservability`.\n"]
    pub fn set_fleetobservability(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_Spec_Fleetobservability>>,
    ) -> Self {
        self.fleetobservability = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multiclusteringress`.\n"]
    pub fn set_multiclusteringress(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_Spec_Multiclusteringress>>,
    ) -> Self {
        self.multiclusteringress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rbacrolebindingactuation`.\n"]
    pub fn set_rbacrolebindingactuation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_Spec_Rbacrolebindingactuation>>,
    ) -> Self {
        self.rbacrolebindingactuation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workloadidentity`.\n"]
    pub fn set_workloadidentity(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_Spec_Workloadidentity>>,
    ) -> Self {
        self.workloadidentity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_Spec {
    type O = terrars::BlockAssignable<DataGkeHubFeature_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_Spec {}

impl BuildDataGkeHubFeature_Spec {
    pub fn build(self) -> DataGkeHubFeature_Spec {
        DataGkeHubFeature_Spec {
            clusterupgrade: core::default::Default::default(),
            fleetobservability: core::default::Default::default(),
            multiclusteringress: core::default::Default::default(),
            rbacrolebindingactuation: core::default::Default::default(),
            workloadidentity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGkeHubFeature_SpecRef {
        DataGkeHubFeature_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `clusterupgrade` after provisioning.\n"]
    pub fn clusterupgrade(&self) -> terrars::ListRef<DataGkeHubFeature_Spec_ClusterupgradeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clusterupgrade", self.base))
    }

    #[doc = "Get a reference to the value of field `fleetobservability` after provisioning.\n"]
    pub fn fleetobservability(&self) -> terrars::ListRef<DataGkeHubFeature_Spec_FleetobservabilityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fleetobservability", self.base))
    }

    #[doc = "Get a reference to the value of field `multiclusteringress` after provisioning.\n"]
    pub fn multiclusteringress(&self) -> terrars::ListRef<DataGkeHubFeature_Spec_MulticlusteringressRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiclusteringress", self.base))
    }

    #[doc = "Get a reference to the value of field `rbacrolebindingactuation` after provisioning.\n"]
    pub fn rbacrolebindingactuation(&self) -> terrars::ListRef<DataGkeHubFeature_Spec_RbacrolebindingactuationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rbacrolebindingactuation", self.base))
    }

    #[doc = "Get a reference to the value of field `workloadidentity` after provisioning.\n"]
    pub fn workloadidentity(&self) -> terrars::ListRef<DataGkeHubFeature_Spec_WorkloadidentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workloadidentity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_State_State {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGkeHubFeature_State_State {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_State_State {
    type O = terrars::BlockAssignable<DataGkeHubFeature_State_State>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_State_State {}

impl BuildDataGkeHubFeature_State_State {
    pub fn build(self) -> DataGkeHubFeature_State_State {
        DataGkeHubFeature_State_State {
            code: core::default::Default::default(),
            description: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_State_StateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_State_StateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGkeHubFeature_State_StateRef {
        DataGkeHubFeature_State_StateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_State_StateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGkeHubFeature_State {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::ListField<DataGkeHubFeature_State_State>>,
}

impl DataGkeHubFeature_State {
    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::ListField<DataGkeHubFeature_State_State>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGkeHubFeature_State {
    type O = terrars::BlockAssignable<DataGkeHubFeature_State>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGkeHubFeature_State {}

impl BuildDataGkeHubFeature_State {
    pub fn build(self) -> DataGkeHubFeature_State {
        DataGkeHubFeature_State { state: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataGkeHubFeature_StateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGkeHubFeature_StateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGkeHubFeature_StateRef {
        DataGkeHubFeature_StateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGkeHubFeature_StateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::ListRef<DataGkeHubFeature_State_StateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.state", self.base))
    }
}
