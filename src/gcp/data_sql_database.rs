use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataSqlDatabaseData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}

struct DataSqlDatabase_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSqlDatabaseData>,
}

#[derive(Clone)]
pub struct DataSqlDatabase(Rc<DataSqlDatabase_>);

impl DataSqlDatabase {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `charset` after provisioning.\nThe charset value. See MySQL's\n[Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)\nand Postgres' [Character Set Support](https://www.postgresql.org/docs/9.6/static/multibyte.html)\nfor more details and supported values. Postgres databases only support\na value of 'UTF8' at creation time."]
    pub fn charset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.charset", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `collation` after provisioning.\nThe collation value. See MySQL's\n[Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)\nand Postgres' [Collation Support](https://www.postgresql.org/docs/9.6/static/collation.html)\nfor more details and supported values. Postgres databases only support\na value of 'en_US.UTF8' at creation time."]
    pub fn collation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_policy` after provisioning.\nThe deletion policy for the database. Setting ABANDON allows the resource\nto be abandoned rather than deleted. This is useful for Postgres, where databases cannot be\ndeleted from the API if there are users other than cloudsqlsuperuser with access. Possible\nvalues are: \"ABANDON\", \"DELETE\". Defaults to \"DELETE\"."]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL instance. This does not include the project\nID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the database in the Cloud SQL instance.\nThis does not include the project ID or instance name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }
}

impl Referable for DataSqlDatabase {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSqlDatabase { }

impl ToListMappable for DataSqlDatabase {
    type O = ListRef<DataSqlDatabaseRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSqlDatabase_ {
    fn extract_datasource_type(&self) -> String {
        "google_sql_database".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSqlDatabase {
    pub tf_id: String,
    #[doc= "The name of the Cloud SQL instance. This does not include the project\nID."]
    pub instance: PrimField<String>,
    #[doc= "The name of the database in the Cloud SQL instance.\nThis does not include the project ID or instance name."]
    pub name: PrimField<String>,
}

impl BuildDataSqlDatabase {
    pub fn build(self, stack: &mut Stack) -> DataSqlDatabase {
        let out = DataSqlDatabase(Rc::new(DataSqlDatabase_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSqlDatabaseData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
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

pub struct DataSqlDatabaseRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSqlDatabaseRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSqlDatabaseRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `charset` after provisioning.\nThe charset value. See MySQL's\n[Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)\nand Postgres' [Character Set Support](https://www.postgresql.org/docs/9.6/static/multibyte.html)\nfor more details and supported values. Postgres databases only support\na value of 'UTF8' at creation time."]
    pub fn charset(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.charset", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `collation` after provisioning.\nThe collation value. See MySQL's\n[Supported Character Sets and Collations](https://dev.mysql.com/doc/refman/5.7/en/charset-charsets.html)\nand Postgres' [Collation Support](https://www.postgresql.org/docs/9.6/static/collation.html)\nfor more details and supported values. Postgres databases only support\na value of 'en_US.UTF8' at creation time."]
    pub fn collation(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `deletion_policy` after provisioning.\nThe deletion policy for the database. Setting ABANDON allows the resource\nto be abandoned rather than deleted. This is useful for Postgres, where databases cannot be\ndeleted from the API if there are users other than cloudsqlsuperuser with access. Possible\nvalues are: \"ABANDON\", \"DELETE\". Defaults to \"DELETE\"."]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance` after provisioning.\nThe name of the Cloud SQL instance. This does not include the project\nID."]
    pub fn instance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\nThe name of the database in the Cloud SQL instance.\nThis does not include the project ID or instance name."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }
}
