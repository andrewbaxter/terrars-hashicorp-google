#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IamWorkloadIdentityPoolManagedIdentityData_ {
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
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    workload_identity_pool_id: terrars::PrimField<std::string::String>,
    workload_identity_pool_managed_identity_id: terrars::PrimField<std::string::String>,
    workload_identity_pool_namespace_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attestation_rules: std::option::Option<std::vec::Vec<IamWorkloadIdentityPoolManagedIdentity_AttestationRules>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IamWorkloadIdentityPoolManagedIdentity_Timeouts>,
    dynamic: IamWorkloadIdentityPoolManagedIdentityDynamic,
}

struct IamWorkloadIdentityPoolManagedIdentity_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IamWorkloadIdentityPoolManagedIdentityData_>,
}

#[derive(Clone)]
pub struct IamWorkloadIdentityPoolManagedIdentity(std::rc::Rc<IamWorkloadIdentityPoolManagedIdentity_>);

impl IamWorkloadIdentityPoolManagedIdentity {
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

    #[doc = "Set the field `description`.\nA description of the managed identity. Cannot exceed 256 characters."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disabled`.\nWhether the managed identity is disabled. If disabled, credentials may no longer be issued for\nthe identity, however existing credentials will still be accepted until they expire."]
    pub fn set_disabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `attestation_rules`.\n"]
    pub fn set_attestation_rules(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<IamWorkloadIdentityPoolManagedIdentity_AttestationRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attestation_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attestation_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IamWorkloadIdentityPoolManagedIdentity_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the managed identity. Cannot exceed 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the managed identity is disabled. If disabled, credentials may no longer be issued for\nthe identity, however existing credentials will still be accepted until they expire."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the managed identity as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}/namespaces/{workload_identity_pool_namespace_id}/managedIdentities/{workload_identity_pool_managed_identity_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the managed identity.\n* 'ACTIVE': The managed identity is active.\n* 'DELETED': The managed identity is soft-deleted. Soft-deleted managed identities are\npermanently deleted after approximately 30 days. You can restore a soft-deleted managed\nidentity using UndeleteWorkloadIdentityPoolManagedIdentity. You cannot reuse the ID of a\nsoft-deleted managed identity until it is permanently deleted."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_identity_pool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_managed_identity_id` after provisioning.\nThe ID to use for the managed identity. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub fn workload_identity_pool_managed_identity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_managed_identity_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_namespace_id` after provisioning.\nThe ID to use for the namespace. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub fn workload_identity_pool_namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_namespace_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolManagedIdentity_TimeoutsRef {
        IamWorkloadIdentityPoolManagedIdentity_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for IamWorkloadIdentityPoolManagedIdentity {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IamWorkloadIdentityPoolManagedIdentity { }

impl terrars::ToListMappable for IamWorkloadIdentityPoolManagedIdentity {
    type O = terrars::ListRef<IamWorkloadIdentityPoolManagedIdentityRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IamWorkloadIdentityPoolManagedIdentity_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_iam_workload_identity_pool_managed_identity".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIamWorkloadIdentityPoolManagedIdentity {
    pub tf_id: std::string::String,
    #[doc =
        "The ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub workload_identity_pool_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the managed identity. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub workload_identity_pool_managed_identity_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID to use for the namespace. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub workload_identity_pool_namespace_id: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkloadIdentityPoolManagedIdentity {
    pub fn build(self, stack: &mut terrars::Stack) -> IamWorkloadIdentityPoolManagedIdentity {
        let out = IamWorkloadIdentityPoolManagedIdentity(std::rc::Rc::new(IamWorkloadIdentityPoolManagedIdentity_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IamWorkloadIdentityPoolManagedIdentityData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                disabled: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                workload_identity_pool_id: self.workload_identity_pool_id,
                workload_identity_pool_managed_identity_id: self.workload_identity_pool_managed_identity_id,
                workload_identity_pool_namespace_id: self.workload_identity_pool_namespace_id,
                attestation_rules: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IamWorkloadIdentityPoolManagedIdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolManagedIdentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IamWorkloadIdentityPoolManagedIdentityRef {
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
        "Get a reference to the value of field `description` after provisioning.\nA description of the managed identity. Cannot exceed 256 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disabled` after provisioning.\nWhether the managed identity is disabled. If disabled, credentials may no longer be issued for\nthe identity, however existing credentials will still be accepted until they expire."]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the managed identity as\n'projects/{project_number}/locations/global/workloadIdentityPools/{workload_identity_pool_id}/namespaces/{workload_identity_pool_namespace_id}/managedIdentities/{workload_identity_pool_managed_identity_id}'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe current state of the managed identity.\n* 'ACTIVE': The managed identity is active.\n* 'DELETED': The managed identity is soft-deleted. Soft-deleted managed identities are\npermanently deleted after approximately 30 days. You can restore a soft-deleted managed\nidentity using UndeleteWorkloadIdentityPoolManagedIdentity. You cannot reuse the ID of a\nsoft-deleted managed identity until it is permanently deleted."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_id` after provisioning.\nThe ID to use for the pool, which becomes the final component of the resource name. This\nvalue should be 4-32 characters, and may contain the characters [a-z0-9-]. The prefix\n'gcp-' is reserved for use by Google, and may not be specified."]
    pub fn workload_identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workload_identity_pool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_managed_identity_id` after provisioning.\nThe ID to use for the managed identity. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub fn workload_identity_pool_managed_identity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_managed_identity_id", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `workload_identity_pool_namespace_id` after provisioning.\nThe ID to use for the namespace. This value must:\n* contain at most 63 characters\n* contain only lowercase alphanumeric characters or '-'\n* start with an alphanumeric character\n* end with an alphanumeric character\n\nThe prefix 'gcp-' will be reserved for future uses."]
    pub fn workload_identity_pool_namespace_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.workload_identity_pool_namespace_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IamWorkloadIdentityPoolManagedIdentity_TimeoutsRef {
        IamWorkloadIdentityPoolManagedIdentity_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct IamWorkloadIdentityPoolManagedIdentityDynamic {
    attestation_rules: std
    ::option
    ::Option<terrars::DynamicBlock<IamWorkloadIdentityPoolManagedIdentity_AttestationRules>>,
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolManagedIdentity_AttestationRules {
    google_cloud_resource: terrars::PrimField<std::string::String>,
}

impl IamWorkloadIdentityPoolManagedIdentity_AttestationRules { }

impl terrars::ToListMappable for IamWorkloadIdentityPoolManagedIdentity_AttestationRules {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPoolManagedIdentity_AttestationRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPoolManagedIdentity_AttestationRules {
    #[doc =
        "A single workload operating on Google Cloud. For example:\n'//compute.googleapis.com/projects/123/uid/zones/us-central1-a/instances/12345678'."]
    pub google_cloud_resource: terrars::PrimField<std::string::String>,
}

impl BuildIamWorkloadIdentityPoolManagedIdentity_AttestationRules {
    pub fn build(self) -> IamWorkloadIdentityPoolManagedIdentity_AttestationRules {
        IamWorkloadIdentityPoolManagedIdentity_AttestationRules { google_cloud_resource: self.google_cloud_resource }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPoolManagedIdentity_AttestationRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolManagedIdentity_AttestationRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkloadIdentityPoolManagedIdentity_AttestationRulesRef {
        IamWorkloadIdentityPoolManagedIdentity_AttestationRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPoolManagedIdentity_AttestationRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `google_cloud_resource` after provisioning.\nA single workload operating on Google Cloud. For example:\n'//compute.googleapis.com/projects/123/uid/zones/us-central1-a/instances/12345678'."]
    pub fn google_cloud_resource(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.google_cloud_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct IamWorkloadIdentityPoolManagedIdentity_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IamWorkloadIdentityPoolManagedIdentity_Timeouts {
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

impl terrars::ToListMappable for IamWorkloadIdentityPoolManagedIdentity_Timeouts {
    type O = terrars::BlockAssignable<IamWorkloadIdentityPoolManagedIdentity_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIamWorkloadIdentityPoolManagedIdentity_Timeouts {}

impl BuildIamWorkloadIdentityPoolManagedIdentity_Timeouts {
    pub fn build(self) -> IamWorkloadIdentityPoolManagedIdentity_Timeouts {
        IamWorkloadIdentityPoolManagedIdentity_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IamWorkloadIdentityPoolManagedIdentity_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IamWorkloadIdentityPoolManagedIdentity_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> IamWorkloadIdentityPoolManagedIdentity_TimeoutsRef {
        IamWorkloadIdentityPoolManagedIdentity_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IamWorkloadIdentityPoolManagedIdentity_TimeoutsRef {
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
