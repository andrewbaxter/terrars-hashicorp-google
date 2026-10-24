use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataSecretManagerSecretsData {
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
    data: std::cell::RefCell<DataSecretManagerSecretsData>,
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
    pub fn secrets(&self) -> terrars::ListRef<DataSecretManagerSecretsSecretsElRef> {
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
            data: std::cell::RefCell::new(DataSecretManagerSecretsData {
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
    pub fn secrets(&self) -> terrars::ListRef<DataSecretManagerSecretsSecretsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {
    type O =
        terrars::BlockAssignable<DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {}

impl BuildDataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {
        DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef {
        DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationElAutoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_encryption: std
    ::option
    ::Option<terrars::ListField<DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl>>,
}

impl DataSecretManagerSecretsSecretsElReplicationElAutoEl {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionEl>,
                    >,
    ) -> Self {
        self.customer_managed_encryption = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretsSecretsElReplicationElAutoEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretsSecretsElReplicationElAutoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretsSecretsElReplicationElAutoEl {}

impl BuildDataSecretManagerSecretsSecretsElReplicationElAutoEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElReplicationElAutoEl {
        DataSecretManagerSecretsSecretsElReplicationElAutoEl {
            customer_managed_encryption: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretsSecretsElReplicationElAutoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretsSecretsElReplicationElAutoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecretsSecretsElReplicationElAutoElRef {
        DataSecretManagerSecretsSecretsElReplicationElAutoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretsSecretsElReplicationElAutoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> terrars::ListRef<DataSecretManagerSecretsSecretsElReplicationElAutoElCustomerManagedEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_managed_encryption", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    type O =
        terrars
        ::BlockAssignable<
            DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {}

impl BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
    pub fn build(
        self,
    ) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
        DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl {
            kms_key_name: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
        DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_encryption: std
    ::option
    ::Option<
        terrars
        ::ListField<DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {
    #[doc = "Set the field `customer_managed_encryption`.\n"]
    pub fn set_customer_managed_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionEl,
                        >,
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

impl terrars::ToListMappable for DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {}

impl BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {
        DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl {
            customer_managed_encryption: core::default::Default::default(),
            location: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef {
        DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_encryption` after provisioning.\n"]
    pub fn customer_managed_encryption(
        &self,
    ) -> terrars
    ::ListRef<DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElCustomerManagedEncryptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_managed_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replicas: std
    ::option
    ::Option<terrars::ListField<DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl>>,
}

impl DataSecretManagerSecretsSecretsElReplicationElUserManagedEl {
    #[doc = "Set the field `replicas`.\n"]
    pub fn set_replicas(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasEl>>,
    ) -> Self {
        self.replicas = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretsSecretsElReplicationElUserManagedEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretsSecretsElReplicationElUserManagedEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedEl {}

impl BuildDataSecretManagerSecretsSecretsElReplicationElUserManagedEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedEl {
        DataSecretManagerSecretsSecretsElReplicationElUserManagedEl { replicas: core::default::Default::default() }
    }
}

pub struct DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef {
        DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replicas` after provisioning.\n"]
    pub fn replicas(&self) -> terrars::ListRef<DataSecretManagerSecretsSecretsElReplicationElUserManagedElReplicasElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replicas", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElReplicationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto: std::option::Option<terrars::ListField<DataSecretManagerSecretsSecretsElReplicationElAutoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_managed: std::option::Option<terrars::ListField<DataSecretManagerSecretsSecretsElReplicationElUserManagedEl>>,
}

impl DataSecretManagerSecretsSecretsElReplicationEl {
    #[doc = "Set the field `auto`.\n"]
    pub fn set_auto(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecretsSecretsElReplicationElAutoEl>>,
    ) -> Self {
        self.auto = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_managed`.\n"]
    pub fn set_user_managed(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecretsSecretsElReplicationElUserManagedEl>>,
    ) -> Self {
        self.user_managed = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretsSecretsElReplicationEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretsSecretsElReplicationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretsSecretsElReplicationEl {}

impl BuildDataSecretManagerSecretsSecretsElReplicationEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElReplicationEl {
        DataSecretManagerSecretsSecretsElReplicationEl {
            auto: core::default::Default::default(),
            user_managed: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretsSecretsElReplicationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretsSecretsElReplicationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecretsSecretsElReplicationElRef {
        DataSecretManagerSecretsSecretsElReplicationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretsSecretsElReplicationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto` after provisioning.\n"]
    pub fn auto(&self) -> terrars::ListRef<DataSecretManagerSecretsSecretsElReplicationElAutoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto", self.base))
    }

    #[doc = "Get a reference to the value of field `user_managed` after provisioning.\n"]
    pub fn user_managed(&self) -> terrars::ListRef<DataSecretManagerSecretsSecretsElReplicationElUserManagedElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_managed", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsElRotationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_rotation_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation_period: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecretsSecretsElRotationEl {
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

impl terrars::ToListMappable for DataSecretManagerSecretsSecretsElRotationEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretsSecretsElRotationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretsSecretsElRotationEl {}

impl BuildDataSecretManagerSecretsSecretsElRotationEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElRotationEl {
        DataSecretManagerSecretsSecretsElRotationEl {
            next_rotation_time: core::default::Default::default(),
            rotation_period: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretsSecretsElRotationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretsSecretsElRotationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecretManagerSecretsSecretsElRotationElRef {
        DataSecretManagerSecretsSecretsElRotationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretsSecretsElRotationElRef {
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
pub struct DataSecretManagerSecretsSecretsElTopicsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecretManagerSecretsSecretsElTopicsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecretManagerSecretsSecretsElTopicsEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretsSecretsElTopicsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretsSecretsElTopicsEl {}

impl BuildDataSecretManagerSecretsSecretsElTopicsEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsElTopicsEl {
        DataSecretManagerSecretsSecretsElTopicsEl { name: core::default::Default::default() }
    }
}

pub struct DataSecretManagerSecretsSecretsElTopicsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretsSecretsElTopicsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSecretManagerSecretsSecretsElTopicsElRef {
        DataSecretManagerSecretsSecretsElTopicsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretsSecretsElTopicsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecretManagerSecretsSecretsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
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
    replication: std::option::Option<terrars::ListField<DataSecretManagerSecretsSecretsElReplicationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rotation: std::option::Option<terrars::ListField<DataSecretManagerSecretsSecretsElRotationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terraform_labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topics: std::option::Option<terrars::ListField<DataSecretManagerSecretsSecretsElTopicsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_aliases: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataSecretManagerSecretsSecretsEl {
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
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecretsSecretsElReplicationEl>>,
    ) -> Self {
        self.replication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rotation`.\n"]
    pub fn set_rotation(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecretsSecretsElRotationEl>>,
    ) -> Self {
        self.rotation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_id`.\n"]
    pub fn set_secret_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_id = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::ListField<DataSecretManagerSecretsSecretsElTopicsEl>>,
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
}

impl terrars::ToListMappable for DataSecretManagerSecretsSecretsEl {
    type O = terrars::BlockAssignable<DataSecretManagerSecretsSecretsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecretManagerSecretsSecretsEl {}

impl BuildDataSecretManagerSecretsSecretsEl {
    pub fn build(self) -> DataSecretManagerSecretsSecretsEl {
        DataSecretManagerSecretsSecretsEl {
            annotations: core::default::Default::default(),
            create_time: core::default::Default::default(),
            effective_annotations: core::default::Default::default(),
            effective_labels: core::default::Default::default(),
            expire_time: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            project: core::default::Default::default(),
            replication: core::default::Default::default(),
            rotation: core::default::Default::default(),
            secret_id: core::default::Default::default(),
            terraform_labels: core::default::Default::default(),
            topics: core::default::Default::default(),
            ttl: core::default::Default::default(),
            version_aliases: core::default::Default::default(),
        }
    }
}

pub struct DataSecretManagerSecretsSecretsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecretManagerSecretsSecretsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSecretManagerSecretsSecretsElRef {
        DataSecretManagerSecretsSecretsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecretManagerSecretsSecretsElRef {
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
    pub fn replication(&self) -> terrars::ListRef<DataSecretManagerSecretsSecretsElReplicationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication", self.base))
    }

    #[doc = "Get a reference to the value of field `rotation` after provisioning.\n"]
    pub fn rotation(&self) -> terrars::ListRef<DataSecretManagerSecretsSecretsElRotationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rotation", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_id` after provisioning.\n"]
    pub fn secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_id", self.base))
    }

    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\n"]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.base))
    }

    #[doc = "Get a reference to the value of field `topics` after provisioning.\n"]
    pub fn topics(&self) -> terrars::ListRef<DataSecretManagerSecretsSecretsElTopicsElRef> {
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
}
