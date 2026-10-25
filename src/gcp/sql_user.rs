#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct SqlUserData_ {
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
    database_roles: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_policy: std::option::Option<std::vec::Vec<SqlUser_PasswordPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SqlUser_Timeouts>,
    dynamic: SqlUserDynamic,
}

struct SqlUser_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SqlUserData_>,
}

#[derive(Clone)]
pub struct SqlUser(std::rc::Rc<SqlUser_>);

impl SqlUser {
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
        "Set the field `database_roles`.\nA list of database roles to be assigned to the user. This option is only available for MySQL and PostgreSQL instances."]
    pub fn set_database_roles(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().database_roles = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `host`.\nThe host the user can connect from. This is only supported for MySQL instances. Don't set this field for PostgreSQL instances. Can be an IP address. Changing this forces a new resource to be created."]
    pub fn set_host(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password`.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn set_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `password_wo`.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn set_password_wo(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().password_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_wo_version`.\nThe version of the password_wo."]
    pub fn set_password_wo_version(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().password_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project`.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nThe user type. It determines the method to authenticate the user during login.\n\t\t\t\tThe default is the database's built-in user type."]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_policy`.\n"]
    pub fn set_password_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SqlUser_PasswordPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().password_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.password_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SqlUser_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `database_roles` after provisioning.\nA list of database roles to be assigned to the user. This option is only available for MySQL and PostgreSQL instances."]
    pub fn database_roles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_roles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe host the user can connect from. This is only supported for MySQL instances. Don't set this field for PostgreSQL instances. Can be an IP address. Changing this forces a new resource to be created."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iam_email` after provisioning.\nThe email address for MySQL IAM database users."]
    pub fn iam_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL instance. Changing this forces a new resource to be created."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the user. Changing this forces a new resource to be created."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password_wo` after provisioning.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password_wo_version` after provisioning.\nThe version of the password_wo."]
    pub fn password_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sql_server_user_details` after provisioning.\n"]
    pub fn sql_server_user_details(&self) -> terrars::ListRef<SqlUser_SqlServerUserDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server_user_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe user type. It determines the method to authenticate the user during login.\n\t\t\t\tThe default is the database's built-in user type."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_policy` after provisioning.\n"]
    pub fn password_policy(&self) -> terrars::ListRef<SqlUser_PasswordPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqlUser_TimeoutsRef {
        SqlUser_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SqlUser {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SqlUser { }

impl terrars::ToListMappable for SqlUser {
    type O = terrars::ListRef<SqlUserRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SqlUser_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_sql_user".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSqlUser {
    pub tf_id: std::string::String,
    #[doc = "The name of the Cloud SQL instance. Changing this forces a new resource to be created."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc = "The name of the user. Changing this forces a new resource to be created."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSqlUser {
    pub fn build(self, stack: &mut terrars::Stack) -> SqlUser {
        let out = SqlUser(std::rc::Rc::new(SqlUser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SqlUserData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                database_roles: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                host: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                name: self.name,
                password: core::default::Default::default(),
                password_wo: core::default::Default::default(),
                password_wo_version: core::default::Default::default(),
                project: core::default::Default::default(),
                type_: core::default::Default::default(),
                password_policy: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SqlUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SqlUserRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `database_roles` after provisioning.\nA list of database roles to be assigned to the user. This option is only available for MySQL and PostgreSQL instances."]
    pub fn database_roles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.database_roles", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `host` after provisioning.\nThe host the user can connect from. This is only supported for MySQL instances. Don't set this field for PostgreSQL instances. Can be an IP address. Changing this forces a new resource to be created."]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `iam_email` after provisioning.\nThe email address for MySQL IAM database users."]
    pub fn iam_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_email", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL instance. Changing this forces a new resource to be created."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the user. Changing this forces a new resource to be created."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password` after provisioning.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password_wo` after provisioning.\nThe password for the user. Can be updated. For Postgres instances this is a Required field, unless type is set to\n\t\t\t\teither CLOUD_IAM_USER or CLOUD_IAM_SERVICE_ACCOUNT."]
    pub fn password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `password_wo_version` after provisioning.\nThe version of the password_wo."]
    pub fn password_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_wo_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `project` after provisioning.\nThe ID of the project in which the resource belongs. If it is not provided, the provider project is used."]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sql_server_user_details` after provisioning.\n"]
    pub fn sql_server_user_details(&self) -> terrars::ListRef<SqlUser_SqlServerUserDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server_user_details", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe user type. It determines the method to authenticate the user during login.\n\t\t\t\tThe default is the database's built-in user type."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_policy` after provisioning.\n"]
    pub fn password_policy(&self) -> terrars::ListRef<SqlUser_PasswordPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SqlUser_TimeoutsRef {
        SqlUser_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SqlUserDynamic {
    password_policy: std::option::Option<terrars::DynamicBlock<SqlUser_PasswordPolicy>>,
}

#[derive(Serialize)]
pub struct SqlUser_SqlServerUserDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_roles: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl SqlUser_SqlServerUserDetails {
    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_roles`.\n"]
    pub fn set_server_roles(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.server_roles = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlUser_SqlServerUserDetails {
    type O = terrars::BlockAssignable<SqlUser_SqlServerUserDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlUser_SqlServerUserDetails {}

impl BuildSqlUser_SqlServerUserDetails {
    pub fn build(self) -> SqlUser_SqlServerUserDetails {
        SqlUser_SqlServerUserDetails {
            disabled: core::default::Default::default(),
            server_roles: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlUser_SqlServerUserDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlUser_SqlServerUserDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlUser_SqlServerUserDetailsRef {
        SqlUser_SqlServerUserDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlUser_SqlServerUserDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disabled", self.base))
    }

    #[doc = "Get a reference to the value of field `server_roles` after provisioning.\n"]
    pub fn server_roles(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_roles", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlUser_PasswordPolicy_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locked: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_expiration_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlUser_PasswordPolicy_Status {
    #[doc = "Set the field `locked`.\n"]
    pub fn set_locked(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.locked = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_expiration_time`.\n"]
    pub fn set_password_expiration_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_expiration_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlUser_PasswordPolicy_Status {
    type O = terrars::BlockAssignable<SqlUser_PasswordPolicy_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlUser_PasswordPolicy_Status {}

impl BuildSqlUser_PasswordPolicy_Status {
    pub fn build(self) -> SqlUser_PasswordPolicy_Status {
        SqlUser_PasswordPolicy_Status {
            locked: core::default::Default::default(),
            password_expiration_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlUser_PasswordPolicy_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlUser_PasswordPolicy_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlUser_PasswordPolicy_StatusRef {
        SqlUser_PasswordPolicy_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlUser_PasswordPolicy_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `locked` after provisioning.\n"]
    pub fn locked(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked", self.base))
    }

    #[doc = "Get a reference to the value of field `password_expiration_time` after provisioning.\n"]
    pub fn password_expiration_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_expiration_time", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlUser_PasswordPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_failed_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_failed_attempts_check: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_password_verification: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_expiration_duration: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlUser_PasswordPolicy {
    #[doc = "Set the field `allowed_failed_attempts`.\nNumber of failed attempts allowed before the user get locked."]
    pub fn set_allowed_failed_attempts(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.allowed_failed_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_failed_attempts_check`.\nIf true, the check that will lock user after too many failed login attempts will be enabled."]
    pub fn set_enable_failed_attempts_check(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_failed_attempts_check = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_password_verification`.\nIf true, the user must specify the current password before changing the password. This flag is supported only for MySQL."]
    pub fn set_enable_password_verification(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_password_verification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `password_expiration_duration`.\nPassword expiration duration with one week grace period."]
    pub fn set_password_expiration_duration(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.password_expiration_duration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SqlUser_PasswordPolicy {
    type O = terrars::BlockAssignable<SqlUser_PasswordPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlUser_PasswordPolicy {}

impl BuildSqlUser_PasswordPolicy {
    pub fn build(self) -> SqlUser_PasswordPolicy {
        SqlUser_PasswordPolicy {
            allowed_failed_attempts: core::default::Default::default(),
            enable_failed_attempts_check: core::default::Default::default(),
            enable_password_verification: core::default::Default::default(),
            password_expiration_duration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlUser_PasswordPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlUser_PasswordPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlUser_PasswordPolicyRef {
        SqlUser_PasswordPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlUser_PasswordPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_failed_attempts` after provisioning.\nNumber of failed attempts allowed before the user get locked."]
    pub fn allowed_failed_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allowed_failed_attempts", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_failed_attempts_check` after provisioning.\nIf true, the check that will lock user after too many failed login attempts will be enabled."]
    pub fn enable_failed_attempts_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_failed_attempts_check", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_password_verification` after provisioning.\nIf true, the user must specify the current password before changing the password. This flag is supported only for MySQL."]
    pub fn enable_password_verification(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_password_verification", self.base))
    }

    #[doc =
        "Get a reference to the value of field `password_expiration_duration` after provisioning.\nPassword expiration duration with one week grace period."]
    pub fn password_expiration_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_expiration_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::ListRef<SqlUser_PasswordPolicy_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct SqlUser_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SqlUser_Timeouts {
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

impl terrars::ToListMappable for SqlUser_Timeouts {
    type O = terrars::BlockAssignable<SqlUser_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSqlUser_Timeouts {}

impl BuildSqlUser_Timeouts {
    pub fn build(self) -> SqlUser_Timeouts {
        SqlUser_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SqlUser_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SqlUser_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SqlUser_TimeoutsRef {
        SqlUser_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SqlUser_TimeoutsRef {
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
