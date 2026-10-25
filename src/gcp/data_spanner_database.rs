#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataSpannerDatabaseData_ {
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
    instance: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataSpannerDatabase_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSpannerDatabaseData_>,
}

#[derive(Clone)]
pub struct DataSpannerDatabase(std::rc::Rc<DataSpannerDatabase_>);

impl DataSpannerDatabase {
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
        "Get a reference to the value of field `database_dialect` after provisioning.\nThe dialect of the Cloud Spanner Database.\nIf it is not provided, \"GOOGLE_STANDARD_SQL\" will be used. Possible values: [\"GOOGLE_STANDARD_SQL\", \"POSTGRESQL\"]"]
    pub fn database_dialect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_dialect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ddl` after provisioning.\nAn optional list of DDL statements to run inside the database. Statements can create\ntables, indexes, etc.\n\nDuring creation these statements execute atomically with the creation of the database\nand if there is an error in any statement, the database is not created.\n\nTerraform does not perform drift detection on this field and assumes that the values\nrecorded in state are accurate. Limited updates to this field are supported, and\nnewly appended DDL statements can be executed in an update. However, modifications\nto prior statements will create a plan that marks the resource for recreation."]
    pub fn ddl(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ddl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_time_zone` after provisioning.\nThe default time zone for the database. The default time zone must be a valid name\nfrom the tz database. Default value is \"America/Los_angeles\"."]
    pub fn default_time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_time_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the database. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the database,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the database will fail.\nWhen the field is set to false, deleting the database is allowed."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_drop_protection` after provisioning.\nWhether drop protection is enabled for this database. Defaults to false.\nDrop protection is different from\nthe \"deletion_protection\" attribute in the following ways:\n(1) \"deletion_protection\" only protects the database from deletions in Terraform.\nwhereas setting “enableDropProtection” to true protects the database from deletions in all interfaces.\n(2) Setting \"enableDropProtection\" to true also prevents the deletion of the parent instance containing the database.\n\"deletion_protection\" attribute does not provide protection against the deletion of the parent instance."]
    pub fn enable_drop_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_drop_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_config` after provisioning.\nEncryption configuration for the database"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataSpannerDatabase_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the database on."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the database, which cannot be changed after the\ninstance is created. Values are of the form '[a-z][-_a-z0-9]*[a-z0-9]'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nAn explanation of the status of the database."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_retention_period` after provisioning.\nThe retention period for the database. The retention period must be between 1 hour\nand 7 days, and can be specified in days, hours, minutes, or seconds. For example,\nthe values 1d, 24h, 1440m, and 86400s are equivalent. Default value is 1h.\nIf this property is used, you must avoid adding new DDL statements to 'ddl' that\nupdate the database's version_retention_period."]
    pub fn version_retention_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_retention_period", self.extract_ref()))
    }
}

impl terrars::Referable for DataSpannerDatabase {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSpannerDatabase { }

impl terrars::ToListMappable for DataSpannerDatabase {
    type O = terrars::ListRef<DataSpannerDatabaseRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSpannerDatabase_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_spanner_database".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSpannerDatabase {
    pub tf_id: std::string::String,
    #[doc = "The instance to create the database on."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "A unique identifier for the database, which cannot be changed after the\ninstance is created. Values are of the form '[a-z][-_a-z0-9]*[a-z0-9]'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataSpannerDatabase {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSpannerDatabase {
        let out = DataSpannerDatabase(std::rc::Rc::new(DataSpannerDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSpannerDatabaseData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                instance: self.instance,
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSpannerDatabaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpannerDatabaseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSpannerDatabaseRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `database_dialect` after provisioning.\nThe dialect of the Cloud Spanner Database.\nIf it is not provided, \"GOOGLE_STANDARD_SQL\" will be used. Possible values: [\"GOOGLE_STANDARD_SQL\", \"POSTGRESQL\"]"]
    pub fn database_dialect(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_dialect", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ddl` after provisioning.\nAn optional list of DDL statements to run inside the database. Statements can create\ntables, indexes, etc.\n\nDuring creation these statements execute atomically with the creation of the database\nand if there is an error in any statement, the database is not created.\n\nTerraform does not perform drift detection on this field and assumes that the values\nrecorded in state are accurate. Limited updates to this field are supported, and\nnewly appended DDL statements can be executed in an update. However, modifications\nto prior statements will create a plan that marks the resource for recreation."]
    pub fn ddl(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ddl", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_time_zone` after provisioning.\nThe default time zone for the database. The default time zone must be a valid name\nfrom the tz database. Default value is \"America/Los_angeles\"."]
    pub fn default_time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_time_zone", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the database. Defaults to true.\nWhen a'terraform destroy' or 'terraform apply' would delete the database,\nthe command will fail if this field is not set to false in Terraform state.\nWhen the field is set to true or unset in Terraform state, a 'terraform apply'\nor 'terraform destroy' that would delete the database will fail.\nWhen the field is set to false, deleting the database is allowed."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_drop_protection` after provisioning.\nWhether drop protection is enabled for this database. Defaults to false.\nDrop protection is different from\nthe \"deletion_protection\" attribute in the following ways:\n(1) \"deletion_protection\" only protects the database from deletions in Terraform.\nwhereas setting “enableDropProtection” to true protects the database from deletions in all interfaces.\n(2) Setting \"enableDropProtection\" to true also prevents the deletion of the parent instance containing the database.\n\"deletion_protection\" attribute does not provide protection against the deletion of the parent instance."]
    pub fn enable_drop_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_drop_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `encryption_config` after provisioning.\nEncryption configuration for the database"]
    pub fn encryption_config(&self) -> terrars::ListRef<DataSpannerDatabase_EncryptionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe instance to create the database on."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA unique identifier for the database, which cannot be changed after the\ninstance is created. Values are of the form '[a-z][-_a-z0-9]*[a-z0-9]'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nAn explanation of the status of the database."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version_retention_period` after provisioning.\nThe retention period for the database. The retention period must be between 1 hour\nand 7 days, and can be specified in days, hours, minutes, or seconds. For example,\nthe values 1d, 24h, 1440m, and 86400s are equivalent. Default value is 1h.\nIf this property is used, you must avoid adding new DDL statements to 'ddl' that\nupdate the database's version_retention_period."]
    pub fn version_retention_period(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_retention_period", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSpannerDatabase_EncryptionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataSpannerDatabase_EncryptionConfig {
    #[doc = "Set the field `kms_key_name`.\n"]
    pub fn set_kms_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_names`.\n"]
    pub fn set_kms_key_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.kms_key_names = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSpannerDatabase_EncryptionConfig {
    type O = terrars::BlockAssignable<DataSpannerDatabase_EncryptionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSpannerDatabase_EncryptionConfig {}

impl BuildDataSpannerDatabase_EncryptionConfig {
    pub fn build(self) -> DataSpannerDatabase_EncryptionConfig {
        DataSpannerDatabase_EncryptionConfig {
            kms_key_name: core::default::Default::default(),
            kms_key_names: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSpannerDatabase_EncryptionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSpannerDatabase_EncryptionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSpannerDatabase_EncryptionConfigRef {
        DataSpannerDatabase_EncryptionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSpannerDatabase_EncryptionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_name` after provisioning.\n"]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_names` after provisioning.\n"]
    pub fn kms_key_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_key_names", self.base))
    }
}
