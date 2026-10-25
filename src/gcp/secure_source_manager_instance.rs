#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SecureSourceManagerInstanceData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_config: std::option::Option<std::vec::Vec<SecureSourceManagerInstance_PrivateConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SecureSourceManagerInstance_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workforce_identity_federation_config: std
    ::option
    ::Option<std::vec::Vec<SecureSourceManagerInstance_WorkforceIdentityFederationConfig>>,
    dynamic: SecureSourceManagerInstanceDynamic,
}

struct SecureSourceManagerInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecureSourceManagerInstanceData_>,
}

#[derive(Clone)]
pub struct SecureSourceManagerInstance(std::rc::Rc<SecureSourceManagerInstance_>);

impl SecureSourceManagerInstance {
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
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key`.\nCustomer-managed encryption key name, in the format projects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub fn set_kms_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels as key value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `private_config`.\n"]
    pub fn set_private_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecureSourceManagerInstance_PrivateConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SecureSourceManagerInstance_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `workforce_identity_federation_config`.\n"]
    pub fn set_workforce_identity_federation_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecureSourceManagerInstance_WorkforceIdentityFederationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workforce_identity_federation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workforce_identity_federation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host_config` after provisioning.\nA list of hostnames for this instance."]
    pub fn host_config(&self) -> terrars::ListRef<SecureSourceManagerInstance_HostConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe name for the Instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nCustomer-managed encryption key name, in the format projects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Instance."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the Instance."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_note` after provisioning.\nProvides information about the current instance state."]
    pub fn state_note(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_note", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_config` after provisioning.\n"]
    pub fn private_config(&self) -> terrars::ListRef<SecureSourceManagerInstance_PrivateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerInstance_TimeoutsRef {
        SecureSourceManagerInstance_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workforce_identity_federation_config` after provisioning.\n"]
    pub fn workforce_identity_federation_config(
        &self,
    ) -> terrars::ListRef<SecureSourceManagerInstance_WorkforceIdentityFederationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.workforce_identity_federation_config", self.extract_ref()),
        )
    }
}

impl terrars::Referable for SecureSourceManagerInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecureSourceManagerInstance { }

impl terrars::ToListMappable for SecureSourceManagerInstance {
    type O = terrars::ListRef<SecureSourceManagerInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecureSourceManagerInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_secure_source_manager_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecureSourceManagerInstance {
    pub tf_id: std::string::String,
    #[doc = "The name for the Instance."]
    pub instance_id: terrars::PrimField<std::string::String>,
    #[doc = "The location for the Instance."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildSecureSourceManagerInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> SecureSourceManagerInstance {
        let out = SecureSourceManagerInstance(std::rc::Rc::new(SecureSourceManagerInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecureSourceManagerInstanceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                kms_key: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                private_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                workforce_identity_federation_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecureSourceManagerInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecureSourceManagerInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime the Instance was created in UTC."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"PREVENT\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host_config` after provisioning.\nA list of hostnames for this instance."]
    pub fn host_config(&self) -> terrars::ListRef<SecureSourceManagerInstance_HostConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.host_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\nThe name for the Instance."]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key` after provisioning.\nCustomer-managed encryption key name, in the format projects/*/locations/*/keyRings/*/cryptoKeys/*."]
    pub fn kms_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location for the Instance."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name for the Instance."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the Instance."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_note` after provisioning.\nProvides information about the current instance state."]
    pub fn state_note(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_note", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime the Instance was updated in UTC."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_config` after provisioning.\n"]
    pub fn private_config(&self) -> terrars::ListRef<SecureSourceManagerInstance_PrivateConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecureSourceManagerInstance_TimeoutsRef {
        SecureSourceManagerInstance_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `workforce_identity_federation_config` after provisioning.\n"]
    pub fn workforce_identity_federation_config(
        &self,
    ) -> terrars::ListRef<SecureSourceManagerInstance_WorkforceIdentityFederationConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.workforce_identity_federation_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct SecureSourceManagerInstanceDynamic {
    private_config: std::option::Option<terrars::DynamicBlock<SecureSourceManagerInstance_PrivateConfig>>,
    workforce_identity_federation_config: std
    ::option
    ::Option<terrars::DynamicBlock<SecureSourceManagerInstance_WorkforceIdentityFederationConfig>>,
}

#[derive(Serialize)]
pub struct SecureSourceManagerInstance_HostConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_http: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git_ssh: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    html: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecureSourceManagerInstance_HostConfig {
    #[doc = "Set the field `api`.\n"]
    pub fn set_api(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git_http`.\n"]
    pub fn set_git_http(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.git_http = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git_ssh`.\n"]
    pub fn set_git_ssh(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.git_ssh = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `html`.\n"]
    pub fn set_html(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.html = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecureSourceManagerInstance_HostConfig {
    type O = terrars::BlockAssignable<SecureSourceManagerInstance_HostConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecureSourceManagerInstance_HostConfig {}

impl BuildSecureSourceManagerInstance_HostConfig {
    pub fn build(self) -> SecureSourceManagerInstance_HostConfig {
        SecureSourceManagerInstance_HostConfig {
            api: core::default::Default::default(),
            git_http: core::default::Default::default(),
            git_ssh: core::default::Default::default(),
            html: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecureSourceManagerInstance_HostConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerInstance_HostConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecureSourceManagerInstance_HostConfigRef {
        SecureSourceManagerInstance_HostConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecureSourceManagerInstance_HostConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api` after provisioning.\n"]
    pub fn api(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api", self.base))
    }

    #[doc = "Get a reference to the value of field `git_http` after provisioning.\n"]
    pub fn git_http(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_http", self.base))
    }

    #[doc = "Get a reference to the value of field `git_ssh` after provisioning.\n"]
    pub fn git_ssh(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_ssh", self.base))
    }

    #[doc = "Get a reference to the value of field `html` after provisioning.\n"]
    pub fn html(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }
}

#[derive(Serialize)]
pub struct SecureSourceManagerInstance_PrivateConfig_CustomHostConfig {
    api: terrars::PrimField<std::string::String>,
    git_http: terrars::PrimField<std::string::String>,
    git_ssh: terrars::PrimField<std::string::String>,
    html: terrars::PrimField<std::string::String>,
}

impl SecureSourceManagerInstance_PrivateConfig_CustomHostConfig { }

impl terrars::ToListMappable for SecureSourceManagerInstance_PrivateConfig_CustomHostConfig {
    type O = terrars::BlockAssignable<SecureSourceManagerInstance_PrivateConfig_CustomHostConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecureSourceManagerInstance_PrivateConfig_CustomHostConfig {
    #[doc = "API hostname."]
    pub api: terrars::PrimField<std::string::String>,
    #[doc = "Git HTTP hostname."]
    pub git_http: terrars::PrimField<std::string::String>,
    #[doc = "Git SSH hostname."]
    pub git_ssh: terrars::PrimField<std::string::String>,
    #[doc = "HTML hostname."]
    pub html: terrars::PrimField<std::string::String>,
}

impl BuildSecureSourceManagerInstance_PrivateConfig_CustomHostConfig {
    pub fn build(self) -> SecureSourceManagerInstance_PrivateConfig_CustomHostConfig {
        SecureSourceManagerInstance_PrivateConfig_CustomHostConfig {
            api: self.api,
            git_http: self.git_http,
            git_ssh: self.git_ssh,
            html: self.html,
        }
    }
}

#[allow(dead_code)]
pub struct SecureSourceManagerInstance_PrivateConfig_CustomHostConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerInstance_PrivateConfig_CustomHostConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecureSourceManagerInstance_PrivateConfig_CustomHostConfigRef {
        SecureSourceManagerInstance_PrivateConfig_CustomHostConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecureSourceManagerInstance_PrivateConfig_CustomHostConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api` after provisioning.\nAPI hostname."]
    pub fn api(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api", self.base))
    }

    #[doc = "Get a reference to the value of field `git_http` after provisioning.\nGit HTTP hostname."]
    pub fn git_http(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_http", self.base))
    }

    #[doc = "Get a reference to the value of field `git_ssh` after provisioning.\nGit SSH hostname."]
    pub fn git_ssh(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.git_ssh", self.base))
    }

    #[doc = "Get a reference to the value of field `html` after provisioning.\nHTML hostname."]
    pub fn html(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.html", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecureSourceManagerInstance_PrivateConfigDynamic {
    custom_host_config: std
    ::option
    ::Option<terrars::DynamicBlock<SecureSourceManagerInstance_PrivateConfig_CustomHostConfig>>,
}

#[derive(Serialize)]
pub struct SecureSourceManagerInstance_PrivateConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ca_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    is_private: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    psc_allowed_projects: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_host_config: std::option::Option<std::vec::Vec<SecureSourceManagerInstance_PrivateConfig_CustomHostConfig>>,
    dynamic: SecureSourceManagerInstance_PrivateConfigDynamic,
}

impl SecureSourceManagerInstance_PrivateConfig {
    #[doc =
        "Set the field `ca_pool`.\nCA pool resource, resource must in the format of 'projects/{project}/locations/{location}/caPools/{ca_pool}'."]
    pub fn set_ca_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ca_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `psc_allowed_projects`.\nOptional. Additional allowed projects for setting up PSC connections.\nInstance host project is automatically allowed and does not need to be included in this list."]
    pub fn set_psc_allowed_projects(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.psc_allowed_projects = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_host_config`.\n"]
    pub fn set_custom_host_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecureSourceManagerInstance_PrivateConfig_CustomHostConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_host_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_host_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecureSourceManagerInstance_PrivateConfig {
    type O = terrars::BlockAssignable<SecureSourceManagerInstance_PrivateConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecureSourceManagerInstance_PrivateConfig {
    #[doc = "'Indicate if it's private instance.'"]
    pub is_private: terrars::PrimField<bool>,
}

impl BuildSecureSourceManagerInstance_PrivateConfig {
    pub fn build(self) -> SecureSourceManagerInstance_PrivateConfig {
        SecureSourceManagerInstance_PrivateConfig {
            ca_pool: core::default::Default::default(),
            is_private: self.is_private,
            psc_allowed_projects: core::default::Default::default(),
            custom_host_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecureSourceManagerInstance_PrivateConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerInstance_PrivateConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecureSourceManagerInstance_PrivateConfigRef {
        SecureSourceManagerInstance_PrivateConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecureSourceManagerInstance_PrivateConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ca_pool` after provisioning.\nCA pool resource, resource must in the format of 'projects/{project}/locations/{location}/caPools/{ca_pool}'."]
    pub fn ca_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_pool", self.base))
    }

    #[doc =
        "Get a reference to the value of field `http_service_attachment` after provisioning.\nService Attachment for HTTP, resource is in the format of 'projects/{project}/regions/{region}/serviceAttachments/{service_attachment}'."]
    pub fn http_service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_service_attachment", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_private` after provisioning.\n'Indicate if it's private instance.'"]
    pub fn is_private(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_private", self.base))
    }

    #[doc =
        "Get a reference to the value of field `psc_allowed_projects` after provisioning.\nOptional. Additional allowed projects for setting up PSC connections.\nInstance host project is automatically allowed and does not need to be included in this list."]
    pub fn psc_allowed_projects(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.psc_allowed_projects", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssh_service_attachment` after provisioning.\nService Attachment for SSH, resource is in the format of 'projects/{project}/regions/{region}/serviceAttachments/{service_attachment}'."]
    pub fn ssh_service_attachment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssh_service_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_host_config` after provisioning.\n"]
    pub fn custom_host_config(&self) -> terrars::ListRef<SecureSourceManagerInstance_PrivateConfig_CustomHostConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_host_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SecureSourceManagerInstance_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecureSourceManagerInstance_Timeouts {
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

impl terrars::ToListMappable for SecureSourceManagerInstance_Timeouts {
    type O = terrars::BlockAssignable<SecureSourceManagerInstance_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecureSourceManagerInstance_Timeouts {}

impl BuildSecureSourceManagerInstance_Timeouts {
    pub fn build(self) -> SecureSourceManagerInstance_Timeouts {
        SecureSourceManagerInstance_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecureSourceManagerInstance_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerInstance_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecureSourceManagerInstance_TimeoutsRef {
        SecureSourceManagerInstance_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecureSourceManagerInstance_TimeoutsRef {
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
pub struct SecureSourceManagerInstance_WorkforceIdentityFederationConfig {
    enabled: terrars::PrimField<bool>,
}

impl SecureSourceManagerInstance_WorkforceIdentityFederationConfig { }

impl terrars::ToListMappable for SecureSourceManagerInstance_WorkforceIdentityFederationConfig {
    type O = terrars::BlockAssignable<SecureSourceManagerInstance_WorkforceIdentityFederationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecureSourceManagerInstance_WorkforceIdentityFederationConfig {
    #[doc = "'Whether Workforce Identity Federation is enabled.'"]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildSecureSourceManagerInstance_WorkforceIdentityFederationConfig {
    pub fn build(self) -> SecureSourceManagerInstance_WorkforceIdentityFederationConfig {
        SecureSourceManagerInstance_WorkforceIdentityFederationConfig { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct SecureSourceManagerInstance_WorkforceIdentityFederationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecureSourceManagerInstance_WorkforceIdentityFederationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecureSourceManagerInstance_WorkforceIdentityFederationConfigRef {
        SecureSourceManagerInstance_WorkforceIdentityFederationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecureSourceManagerInstance_WorkforceIdentityFederationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\n'Whether Workforce Identity Federation is enabled.'"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}
