#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataSecretManagerSecretsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataSecretManagerSecrets_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSecretManagerSecretsData_>,
}

#[derive(Clone)]
pub struct DataSecretManagerSecrets(std::rc::Rc<DataSecretManagerSecrets_>);

impl DataSecretManagerSecrets {
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

    #[doc =
        "Set the field `filter`.\nFilter string, adhering to the rules in List-operation filtering (https://cloud.google.com/secret-manager/docs/filtering).\nList only secrets matching the filter. If filter is empty, all secrets are listed."]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().filter = std::option::Option::Some(v.into());
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
        "Get a reference to the value of field `filter` after provisioning.\nFilter string, adhering to the rules in List-operation filtering (https://cloud.google.com/secret-manager/docs/filtering).\nList only secrets matching the filter. If filter is empty, all secrets are listed."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> terrars::ListRef<DataSecretManagerSecrets_SecretsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets", self.extract_ref()))
    }
}

impl terrars::Referable for DataSecretManagerSecrets {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSecretManagerSecrets { }

impl terrars::ToListMappable for DataSecretManagerSecrets {
    type O = terrars::ListRef<DataSecretManagerSecretsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSecretManagerSecrets_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_secret_manager_secrets".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecretManagerSecrets {
    pub tf_id: std::string::String,
}

impl BuildDataSecretManagerSecrets {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSecretManagerSecrets {
        let out = DataSecretManagerSecrets(std::rc::Rc::new(DataSecretManagerSecrets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSecretManagerSecretsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                filter: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecretManagerSecretsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSecretManagerSecretsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `filter` after provisioning.\nFilter string, adhering to the rules in List-operation filtering (https://cloud.google.com/secret-manager/docs/filtering).\nList only secrets matching the filter. If filter is empty, all secrets are listed."]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secrets` after provisioning.\n"]
    pub fn secrets(&self) -> terrars::ListRef<DataSecretManagerSecrets_SecretsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption {
    type O = terrars::BlockAssignable<DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption {}

impl BuildDataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption {
    pub fn build(self) -> DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption {
        DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption {
            kms_key_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryptionRef {
        DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecrets_Secrets_Replication_Auto {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_encryption: std
    ::option
    ::Option<terrars::ListField<DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption>>,
}

impl DataSecretManagerSecrets_Secrets_Replication_Auto {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption>,
                    >,
    ) -> Self {
        self.customer_managed_encryption = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecrets_Secrets_Replication_Auto {
    type O = terrars::BlockAssignable<DataSecretManagerSecrets_Secrets_Replication_Auto>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecrets_Secrets_Replication_Auto {}

impl BuildDataSecretManagerSecrets_Secrets_Replication_Auto {
    pub fn build(self) -> DataSecretManagerSecrets_Secrets_Replication_Auto {
        DataSecretManagerSecrets_Secrets_Replication_Auto {
            customer_managed_encryption: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSecretManagerSecrets_Secrets_Replication_AutoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecrets_Secrets_Replication_AutoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecrets_Secrets_Replication_AutoRef {
        DataSecretManagerSecrets_Secrets_Replication_AutoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecrets_Secrets_Replication_AutoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> terrars::ListRef<DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_managed_encryption", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecrets_Secrets_Replication_UserManaged_Replicas {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_encryption: std
    ::option
    ::Option<terrars::ListField<DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecrets_Secrets_Replication_UserManaged_Replicas {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryption>,
                    >,
    ) -> Self {
        self.customer_managed_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecrets_Secrets_Replication_UserManaged_Replicas {
    type O = terrars::BlockAssignable<DataSecretManagerSecrets_Secrets_Replication_UserManaged_Replicas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecrets_Secrets_Replication_UserManaged_Replicas {}

impl BuildDataSecretManagerSecrets_Secrets_Replication_UserManaged_Replicas {
    pub fn build(self) -> DataSecretManagerSecrets_Secrets_Replication_UserManaged_Replicas {
        DataSecretManagerSecrets_Secrets_Replication_UserManaged_Replicas {
            customer_managed_encryption: core::default::Default::default(),
            location: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSecretManagerSecrets_Secrets_Replication_UserManaged_ReplicasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecrets_Secrets_Replication_UserManaged_ReplicasRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecrets_Secrets_Replication_UserManaged_ReplicasRef {
        DataSecretManagerSecrets_Secrets_Replication_UserManaged_ReplicasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecrets_Secrets_Replication_UserManaged_ReplicasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> terrars::ListRef<DataSecretManagerSecrets_Secrets_Replication_Auto_CustomerManagedEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_managed_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecrets_Secrets_Replication_UserManaged {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std::option::Option<terrars::ListField<DataSecretManagerSecrets_Secrets_Replication_UserManaged_Replicas>>,
}

impl DataSecretManagerSecrets_Secrets_Replication_UserManaged {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSecretManagerSecrets_Secrets_Replication_UserManaged_Replicas>>,
    ) -> Self {
        self.replicas = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecrets_Secrets_Replication_UserManaged {
    type O = terrars::BlockAssignable<DataSecretManagerSecrets_Secrets_Replication_UserManaged>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecrets_Secrets_Replication_UserManaged {}

impl BuildDataSecretManagerSecrets_Secrets_Replication_UserManaged {
    pub fn build(self) -> DataSecretManagerSecrets_Secrets_Replication_UserManaged {
        DataSecretManagerSecrets_Secrets_Replication_UserManaged { replicas: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataSecretManagerSecrets_Secrets_Replication_UserManagedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecrets_Secrets_Replication_UserManagedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecrets_Secrets_Replication_UserManagedRef {
        DataSecretManagerSecrets_Secrets_Replication_UserManagedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecrets_Secrets_Replication_UserManagedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> terrars::ListRef<DataSecretManagerSecrets_Secrets_Replication_UserManaged_ReplicasRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecrets_Secrets_Replication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto: std::option::Option<terrars::ListField<DataSecretManagerSecrets_Secrets_Replication_Auto>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_managed: std::option::Option<terrars::ListField<DataSecretManagerSecrets_Secrets_Replication_UserManaged>>,
}

impl DataSecretManagerSecrets_Secrets_Replication {
    #[doc = "Set the field `auto`.\n"]
    pub fn set_auto(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecrets_Secrets_Replication_Auto>>,
    ) -> Self {
        self.auto = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_managed`.\n"]
    pub fn set_user_managed(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecrets_Secrets_Replication_UserManaged>>,
    ) -> Self {
        self.user_managed = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecrets_Secrets_Replication {
    type O = terrars::BlockAssignable<DataSecretManagerSecrets_Secrets_Replication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecrets_Secrets_Replication {}

impl BuildDataSecretManagerSecrets_Secrets_Replication {
    pub fn build(self) -> DataSecretManagerSecrets_Secrets_Replication {
        DataSecretManagerSecrets_Secrets_Replication {
            auto: core::default::Default::default(),
            user_managed: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSecretManagerSecrets_Secrets_ReplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecrets_Secrets_ReplicationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecrets_Secrets_ReplicationRef {
        DataSecretManagerSecrets_Secrets_ReplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecrets_Secrets_ReplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto` after provisioning.\n"]
    pub fn auto(&self) -> terrars::ListRef<DataSecretManagerSecrets_Secrets_Replication_AutoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto", self.base))
    }

    #[doc = "Get a reference to the value of field `user_managed` after provisioning.\n"]
    pub fn user_managed(&self) -> terrars::ListRef<DataSecretManagerSecrets_Secrets_Replication_UserManagedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_managed", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecrets_Secrets_Rotation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_rotation_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecrets_Secrets_Rotation {
    #[doc = "Set the field `next_rotation_time`.\n"]
    pub fn set_next_rotation_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_rotation_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rotation_period`.\n"]
    pub fn set_rotation_period(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rotation_period = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecrets_Secrets_Rotation {
    type O = terrars::BlockAssignable<DataSecretManagerSecrets_Secrets_Rotation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecrets_Secrets_Rotation {}

impl BuildDataSecretManagerSecrets_Secrets_Rotation {
    pub fn build(self) -> DataSecretManagerSecrets_Secrets_Rotation {
        DataSecretManagerSecrets_Secrets_Rotation {
            next_rotation_time: core::default::Default::default(),
            rotation_period: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSecretManagerSecrets_Secrets_RotationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecrets_Secrets_RotationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSecretManagerSecrets_Secrets_RotationRef {
        DataSecretManagerSecrets_Secrets_RotationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecrets_Secrets_RotationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `next_rotation_time` after provisioning.\n"]
    pub fn next_rotation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_rotation_time", self.base))
    }

    #[doc = "Get a reference to the value of field `rotation_period` after provisioning.\n"]
    pub fn rotation_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rotation_period", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecrets_Secrets_Topics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecrets_Secrets_Topics {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecrets_Secrets_Topics {
    type O = terrars::BlockAssignable<DataSecretManagerSecrets_Secrets_Topics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecrets_Secrets_Topics {}

impl BuildDataSecretManagerSecrets_Secrets_Topics {
    pub fn build(self) -> DataSecretManagerSecrets_Secrets_Topics {
        DataSecretManagerSecrets_Secrets_Topics { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataSecretManagerSecrets_Secrets_TopicsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecrets_Secrets_TopicsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSecretManagerSecrets_Secrets_TopicsRef {
        DataSecretManagerSecrets_Secrets_TopicsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecrets_Secrets_TopicsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecrets_Secrets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    effective_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expire_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication: std::option::Option<terrars::ListField<DataSecretManagerSecrets_Secrets_Replication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation: std::option::Option<terrars::ListField<DataSecretManagerSecrets_Secrets_Rotation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terraform_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topics: std::option::Option<terrars::ListField<DataSecretManagerSecrets_Secrets_Topics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_aliases: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_destroy_ttl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecrets_Secrets {
    #[doc = "Set the field `annotations`.\n"]
    pub fn set_annotations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_policy`.\n"]
    pub fn set_deletion_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deletion_protection`.\n"]
    pub fn set_deletion_protection(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `effective_annotations`.\n"]
    pub fn set_effective_annotations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.effective_annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `effective_labels`.\n"]
    pub fn set_effective_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.effective_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expire_time`.\n"]
    pub fn set_expire_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expire_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication`.\n"]
    pub fn set_replication(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecrets_Secrets_Replication>>,
    ) -> Self {
        self.replication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rotation`.\n"]
    pub fn set_rotation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecrets_Secrets_Rotation>>,
    ) -> Self {
        self.rotation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_id`.\n"]
    pub fn set_secret_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_type`.\n"]
    pub fn set_secret_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `terraform_labels`.\n"]
    pub fn set_terraform_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.terraform_labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topics`.\n"]
    pub fn set_topics(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecrets_Secrets_Topics>>,
    ) -> Self {
        self.topics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_aliases`.\n"]
    pub fn set_version_aliases(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.version_aliases = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_destroy_ttl`.\n"]
    pub fn set_version_destroy_ttl(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.version_destroy_ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecrets_Secrets {
    type O = terrars::BlockAssignable<DataSecretManagerSecrets_Secrets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecrets_Secrets {}

impl BuildDataSecretManagerSecrets_Secrets {
    pub fn build(self) -> DataSecretManagerSecrets_Secrets {
        DataSecretManagerSecrets_Secrets {
            annotations: core::default::Default::default(),
            create_time: core::default::Default::default(),
            deletion_policy: core::default::Default::default(),
            deletion_protection: core::default::Default::default(),
            effective_annotations: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            expire_time: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
            replication: core::default::Default::default(),
            rotation: core::default::Default::default(),
            secret_id: core::default::Default::default(),
            secret_type: core::default::Default::default(),
            tags: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
            topics: core::default::Default::default(),
            ttl: core::default::Default::default(),
            version_aliases: core::default::Default::default(),
            version_destroy_ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSecretManagerSecrets_SecretsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecrets_SecretsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSecretManagerSecrets_SecretsRef {
        DataSecretManagerSecrets_SecretsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecrets_SecretsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `annotations` after provisioning.\n"]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `deletion_protection` after provisioning.\n"]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `effective_annotations` after provisioning.\n"]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.base))
    }

    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\n"]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `expire_time` after provisioning.\n"]
    pub fn expire_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expire_time", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.base))
    }

    #[doc = "Get a reference to the value of field `replication` after provisioning.\n"]
    pub fn replication(&self) -> terrars::ListRef<DataSecretManagerSecrets_Secrets_ReplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication", self.base))
    }

    #[doc = "Get a reference to the value of field `rotation` after provisioning.\n"]
    pub fn rotation(&self) -> terrars::ListRef<DataSecretManagerSecrets_Secrets_RotationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_type` after provisioning.\n"]
    pub fn secret_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_type", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> terrars::ListRef<DataSecretManagerSecrets_Secrets_TopicsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.topics", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `version_aliases` after provisioning.\n"]
    pub fn version_aliases(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.version_aliases", self.base))
    }

    #[doc = "Get a reference to the value of field `version_destroy_ttl` after provisioning.\n"]
    pub fn version_destroy_ttl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_destroy_ttl", self.base))
    }
}
