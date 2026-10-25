#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct IntegrationsClientData_ {
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
    create_sample_integrations: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_kms_config: std::option::Option<std::vec::Vec<IntegrationsClient_CloudKmsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<IntegrationsClient_Timeouts>,
    dynamic: IntegrationsClientDynamic,
}

struct IntegrationsClient_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IntegrationsClientData_>,
}

#[derive(Clone)]
pub struct IntegrationsClient(std::rc::Rc<IntegrationsClient_>);

impl IntegrationsClient {
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
        "Set the field `create_sample_integrations`.\nIndicates if sample integrations should be created along with provisioning."]
    pub fn set_create_sample_integrations(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_sample_integrations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `cloud_kms_config`.\n"]
    pub fn set_cloud_kms_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IntegrationsClient_CloudKmsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloud_kms_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloud_kms_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<IntegrationsClient_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `create_sample_integrations` after provisioning.\nIndicates if sample integrations should be created along with provisioning."]
    pub fn create_sample_integrations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_sample_integrations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation in which client needs to be provisioned."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_kms_config` after provisioning.\n"]
    pub fn cloud_kms_config(&self) -> terrars::ListRef<IntegrationsClient_CloudKmsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_kms_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationsClient_TimeoutsRef {
        IntegrationsClient_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for IntegrationsClient {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IntegrationsClient { }

impl terrars::ToListMappable for IntegrationsClient {
    type O = terrars::ListRef<IntegrationsClientRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IntegrationsClient_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_integrations_client".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIntegrationsClient {
    pub tf_id: std::string::String,
    #[doc = "Location in which client needs to be provisioned."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationsClient {
    pub fn build(self, stack: &mut terrars::Stack) -> IntegrationsClient {
        let out = IntegrationsClient(std::rc::Rc::new(IntegrationsClient_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IntegrationsClientData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                create_sample_integrations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                cloud_kms_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IntegrationsClientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsClientRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IntegrationsClientRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create_sample_integrations` after provisioning.\nIndicates if sample integrations should be created along with provisioning."]
    pub fn create_sample_integrations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_sample_integrations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nLocation in which client needs to be provisioned."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloud_kms_config` after provisioning.\n"]
    pub fn cloud_kms_config(&self) -> terrars::ListRef<IntegrationsClient_CloudKmsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_kms_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> IntegrationsClient_TimeoutsRef {
        IntegrationsClient_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct IntegrationsClientDynamic {
    cloud_kms_config: std::option::Option<terrars::DynamicBlock<IntegrationsClient_CloudKmsConfig>>,
}

#[derive(Serialize)]
pub struct IntegrationsClient_CloudKmsConfig {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_version: std::option::Option<terrars::PrimField<std::string::String>>,
    kms_location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    kms_ring: terrars::PrimField<std::string::String>,
}

impl IntegrationsClient_CloudKmsConfig {
    #[doc =
        "Set the field `key_version`.\nEach version of a key contains key material used for encryption or signing.\nA key's version is represented by an integer, starting at 1. To decrypt data\nor verify a signature, you must use the same key version that was used to\nencrypt or sign the data."]
    pub fn set_key_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_project_id`.\nThe Google Cloud project id of the project where the kms key stored. If empty,\nthe kms key is stored at the same project as customer's project and ecrypted\nwith CMEK, otherwise, the kms key is stored in the tenant project and\nencrypted with GMEK."]
    pub fn set_kms_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_project_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IntegrationsClient_CloudKmsConfig {
    type O = terrars::BlockAssignable<IntegrationsClient_CloudKmsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsClient_CloudKmsConfig {
    #[doc =
        "A Cloud KMS key is a named object containing one or more key versions, along\nwith metadata for the key. A key exists on exactly one key ring tied to a\nspecific location."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = "Location name of the key ring, e.g. \"us-west1\"."]
    pub kms_location: terrars::PrimField<std::string::String>,
    #[doc =
        "A key ring organizes keys in a specific Google Cloud location and allows you to\nmanage access control on groups of keys. A key ring's name does not need to be\nunique across a Google Cloud project, but must be unique within a given location."]
    pub kms_ring: terrars::PrimField<std::string::String>,
}

impl BuildIntegrationsClient_CloudKmsConfig {
    pub fn build(self) -> IntegrationsClient_CloudKmsConfig {
        IntegrationsClient_CloudKmsConfig {
            key: self.key,
            key_version: core::default::Default::default(),
            kms_location: self.kms_location,
            kms_project_id: core::default::Default::default(),
            kms_ring: self.kms_ring,
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsClient_CloudKmsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsClient_CloudKmsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationsClient_CloudKmsConfigRef {
        IntegrationsClient_CloudKmsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsClient_CloudKmsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nA Cloud KMS key is a named object containing one or more key versions, along\nwith metadata for the key. A key exists on exactly one key ring tied to a\nspecific location."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `key_version` after provisioning.\nEach version of a key contains key material used for encryption or signing.\nA key's version is represented by an integer, starting at 1. To decrypt data\nor verify a signature, you must use the same key version that was used to\nencrypt or sign the data."]
    pub fn key_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_location` after provisioning.\nLocation name of the key ring, e.g. \"us-west1\"."]
    pub fn kms_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_location", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_project_id` after provisioning.\nThe Google Cloud project id of the project where the kms key stored. If empty,\nthe kms key is stored at the same project as customer's project and ecrypted\nwith CMEK, otherwise, the kms key is stored in the tenant project and\nencrypted with GMEK."]
    pub fn kms_project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `kms_ring` after provisioning.\nA key ring organizes keys in a specific Google Cloud location and allows you to\nmanage access control on groups of keys. A key ring's name does not need to be\nunique across a Google Cloud project, but must be unique within a given location."]
    pub fn kms_ring(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_ring", self.base))
    }
}

#[derive(Serialize)]
pub struct IntegrationsClient_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IntegrationsClient_Timeouts {
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

impl terrars::ToListMappable for IntegrationsClient_Timeouts {
    type O = terrars::BlockAssignable<IntegrationsClient_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIntegrationsClient_Timeouts {}

impl BuildIntegrationsClient_Timeouts {
    pub fn build(self) -> IntegrationsClient_Timeouts {
        IntegrationsClient_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IntegrationsClient_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IntegrationsClient_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IntegrationsClient_TimeoutsRef {
        IntegrationsClient_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IntegrationsClient_TimeoutsRef {
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
