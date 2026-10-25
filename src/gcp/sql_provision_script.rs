#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SqlProvisionScriptData_ {
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
    database: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_secret_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    script: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct SqlProvisionScript_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SqlProvisionScriptData_>,
}

#[derive(Clone)]
pub struct SqlProvisionScript(std::rc::Rc<SqlProvisionScript_>);

impl SqlProvisionScript {
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
        "Set the field `database`.\nThe name of the database to which Terraform connects. Changing\n\t\t\t\tthis forces Terraform to connect to the new database and run the script. This argument is\n\t\t\t\trequired for Postgres instances. It's optional for MySQL, but some of your queries may require\n\t\t\t\ta database. You can create and use a database in the script or explicitly reference a\n\t\t\t\tgoogle_sql_database."]
    pub fn set_database(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().database = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nThe deletion policy for the resources created by the script. The default is \"ABANDON\".\n\t\t\t\tIt must be \"ABANDON\" to allow Terraform to abandon the resources. If you want to delete resources, add statements\n\t\t\t\tin the script such as \"drop … if exists\"."]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the provision script."]
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
        "Set the field `password_secret_version`.\nThe resource name of the Secret Manager secret storing the password. The secret\n\t\t\t\tshould be a regional secret and stored in the exact same region as the Cloud SQL instance.\n\t\t\t\tFollow https://docs.cloud.google.com/secret-manager/regional-secrets/create-regional-secret.\n\t\t\t\tWhen user and password_secret_version are provided, the script is run using this user.\n\t\t\t\tOtherwise, the script is run using the identity account used to apply your Terraform config.\n\t\t\t\tChanging this field forces the script to be run again."]
    pub fn set_password_secret_version(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().password_secret_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user`.\nThe name of the built-in database user to authenticate as. For MySQL user,\n\t\t\t\tomit '@' and the hostname. The user should exist as a built-in user in the database.\n\t\t\t\tWhen user and password_secret_version are provided, the script is run using this user. Otherwise,\n\t\t\t\tthe script is run using the identity account used to apply your Terraform config.\n\t\t\t\tChanging this forces the script to be run using the new user."]
    pub fn set_user(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe name of the database to which Terraform connects. Changing\n\t\t\t\tthis forces Terraform to connect to the new database and run the script. This argument is\n\t\t\t\trequired for Postgres instances. It's optional for MySQL, but some of your queries may require\n\t\t\t\ta database. You can create and use a database in the script or explicitly reference a\n\t\t\t\tgoogle_sql_database."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThe deletion policy for the resources created by the script. The default is \"ABANDON\".\n\t\t\t\tIt must be \"ABANDON\" to allow Terraform to abandon the resources. If you want to delete resources, add statements\n\t\t\t\tin the script such as \"drop … if exists\"."]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the provision script."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL instance. Changing this forces the script to be run on the new instance."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nThe resource name of the Secret Manager secret storing the password. The secret\n\t\t\t\tshould be a regional secret and stored in the exact same region as the Cloud SQL instance.\n\t\t\t\tFollow https://docs.cloud.google.com/secret-manager/regional-secrets/create-regional-secret.\n\t\t\t\tWhen user and password_secret_version are provided, the script is run using this user.\n\t\t\t\tOtherwise, the script is run using the identity account used to apply your Terraform config.\n\t\t\t\tChanging this field forces the script to be run again."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `script` after provisioning.\nThe SQL script to provision database resources. Its execution time limit is 30 s.\n\t\t\t\tChanging this forces the script to be rerun. Make sure the script is idempotent.\n\t\t\t\tYou can use statements like \"create if not exists …\" or\n\t\t\t\t\"if not exists (select …) then … end if\" to prevent existence-related errors. If it's not\n\t\t\t\tpossible to make a statement idempotent, you can run it once and then remove it from this script."]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nThe name of the built-in database user to authenticate as. For MySQL user,\n\t\t\t\tomit '@' and the hostname. The user should exist as a built-in user in the database.\n\t\t\t\tWhen user and password_secret_version are provided, the script is run using this user. Otherwise,\n\t\t\t\tthe script is run using the identity account used to apply your Terraform config.\n\t\t\t\tChanging this forces the script to be run using the new user."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}

impl terrars::Referable for SqlProvisionScript {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SqlProvisionScript { }

impl terrars::ToListMappable for SqlProvisionScript {
    type O = terrars::ListRef<SqlProvisionScriptRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SqlProvisionScript_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_sql_provision_script".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSqlProvisionScript {
    pub tf_id: std::string::String,
    #[doc = "The name of the Cloud SQL instance. Changing this forces the script to be run on the new instance."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "The SQL script to provision database resources. Its execution time limit is 30 s.\n\t\t\t\tChanging this forces the script to be rerun. Make sure the script is idempotent.\n\t\t\t\tYou can use statements like \"create if not exists …\" or\n\t\t\t\t\"if not exists (select …) then … end if\" to prevent existence-related errors. If it's not\n\t\t\t\tpossible to make a statement idempotent, you can run it once and then remove it from this script."]
    pub script: terrars::PrimField<std::string::String>,
}

impl BuildSqlProvisionScript {
    pub fn build(self, stack: &mut terrars::Stack) -> SqlProvisionScript {
        let out = SqlProvisionScript(std::rc::Rc::new(SqlProvisionScript_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SqlProvisionScriptData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                database: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                password_secret_version: core::default::Default::default(),
                project: core::default::Default::default(),
                script: self.script,
                user: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SqlProvisionScriptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlProvisionScriptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SqlProvisionScriptRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database` after provisioning.\nThe name of the database to which Terraform connects. Changing\n\t\t\t\tthis forces Terraform to connect to the new database and run the script. This argument is\n\t\t\t\trequired for Postgres instances. It's optional for MySQL, but some of your queries may require\n\t\t\t\ta database. You can create and use a database in the script or explicitly reference a\n\t\t\t\tgoogle_sql_database."]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nThe deletion policy for the resources created by the script. The default is \"ABANDON\".\n\t\t\t\tIt must be \"ABANDON\" to allow Terraform to abandon the resources. If you want to delete resources, add statements\n\t\t\t\tin the script such as \"drop … if exists\"."]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the provision script."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL instance. Changing this forces the script to be run on the new instance."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password_secret_version` after provisioning.\nThe resource name of the Secret Manager secret storing the password. The secret\n\t\t\t\tshould be a regional secret and stored in the exact same region as the Cloud SQL instance.\n\t\t\t\tFollow https://docs.cloud.google.com/secret-manager/regional-secrets/create-regional-secret.\n\t\t\t\tWhen user and password_secret_version are provided, the script is run using this user.\n\t\t\t\tOtherwise, the script is run using the identity account used to apply your Terraform config.\n\t\t\t\tChanging this field forces the script to be run again."]
    pub fn password_secret_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_secret_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `script` after provisioning.\nThe SQL script to provision database resources. Its execution time limit is 30 s.\n\t\t\t\tChanging this forces the script to be rerun. Make sure the script is idempotent.\n\t\t\t\tYou can use statements like \"create if not exists …\" or\n\t\t\t\t\"if not exists (select …) then … end if\" to prevent existence-related errors. If it's not\n\t\t\t\tpossible to make a statement idempotent, you can run it once and then remove it from this script."]
    pub fn script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `user` after provisioning.\nThe name of the built-in database user to authenticate as. For MySQL user,\n\t\t\t\tomit '@' and the hostname. The user should exist as a built-in user in the database.\n\t\t\t\tWhen user and password_secret_version are provided, the script is run using this user. Otherwise,\n\t\t\t\tthe script is run using the identity account used to apply your Terraform config.\n\t\t\t\tChanging this forces the script to be run using the new user."]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}
