#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataBackupDrManagementServerData_ {
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
}

struct DataBackupDrManagementServer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBackupDrManagementServerData_>,
}

#[derive(Clone)]
pub struct DataBackupDrManagementServer(std::rc::Rc<DataBackupDrManagementServer_>);

impl DataBackupDrManagementServer {
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
        "Get a reference to the value of field `location` after provisioning.\nThe location for the management server (management console)"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management_uri` after provisioning.\nThe management console URI"]
    pub fn management_uri(&self) -> terrars::ListRef<DataBackupDrManagementServer_ManagementUriRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of management server (management console)"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nNetwork details to create management server (management console)."]
    pub fn networks(&self) -> terrars::ListRef<DataBackupDrManagementServer_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_id` after provisioning.\nThe oauth2ClientId of management console."]
    pub fn oauth2_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of management server (management console). Default value: \"BACKUP_RESTORE\" Possible values: [\"BACKUP_RESTORE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataBackupDrManagementServer {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBackupDrManagementServer { }

impl terrars::ToListMappable for DataBackupDrManagementServer {
    type O = terrars::ListRef<DataBackupDrManagementServerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBackupDrManagementServer_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_backup_dr_management_server".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBackupDrManagementServer {
    pub tf_id: std::string::String,
    #[doc = "The location for the management server (management console)"]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDataBackupDrManagementServer {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBackupDrManagementServer {
        let out = DataBackupDrManagementServer(std::rc::Rc::new(DataBackupDrManagementServer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBackupDrManagementServerData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBackupDrManagementServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrManagementServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBackupDrManagementServerRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
        "Get a reference to the value of field `location` after provisioning.\nThe location for the management server (management console)"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `management_uri` after provisioning.\nThe management console URI"]
    pub fn management_uri(&self) -> terrars::ListRef<DataBackupDrManagementServer_ManagementUriRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of management server (management console)"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `networks` after provisioning.\nNetwork details to create management server (management console)."]
    pub fn networks(&self) -> terrars::ListRef<DataBackupDrManagementServer_NetworksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.networks", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `oauth2_client_id` after provisioning.\nThe oauth2ClientId of management console."]
    pub fn oauth2_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_client_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of management server (management console). Default value: \"BACKUP_RESTORE\" Possible values: [\"BACKUP_RESTORE\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrManagementServer_ManagementUri {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    web_ui: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrManagementServer_ManagementUri {
    #[doc = "Set the field `api`.\n"]
    pub fn set_api(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.api = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `web_ui`.\n"]
    pub fn set_web_ui(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.web_ui = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrManagementServer_ManagementUri {
    type O = terrars::BlockAssignable<DataBackupDrManagementServer_ManagementUri>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrManagementServer_ManagementUri {}

impl BuildDataBackupDrManagementServer_ManagementUri {
    pub fn build(self) -> DataBackupDrManagementServer_ManagementUri {
        DataBackupDrManagementServer_ManagementUri {
            api: core::default::Default::default(),
            web_ui: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrManagementServer_ManagementUriRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrManagementServer_ManagementUriRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupDrManagementServer_ManagementUriRef {
        DataBackupDrManagementServer_ManagementUriRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrManagementServer_ManagementUriRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api` after provisioning.\n"]
    pub fn api(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api", self.base))
    }

    #[doc = "Get a reference to the value of field `web_ui` after provisioning.\n"]
    pub fn web_ui(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.web_ui", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupDrManagementServer_Networks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peering_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupDrManagementServer_Networks {
    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peering_mode`.\n"]
    pub fn set_peering_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peering_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupDrManagementServer_Networks {
    type O = terrars::BlockAssignable<DataBackupDrManagementServer_Networks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupDrManagementServer_Networks {}

impl BuildDataBackupDrManagementServer_Networks {
    pub fn build(self) -> DataBackupDrManagementServer_Networks {
        DataBackupDrManagementServer_Networks {
            network: core::default::Default::default(),
            peering_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupDrManagementServer_NetworksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupDrManagementServer_NetworksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupDrManagementServer_NetworksRef {
        DataBackupDrManagementServer_NetworksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupDrManagementServer_NetworksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `peering_mode` after provisioning.\n"]
    pub fn peering_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peering_mode", self.base))
    }
}
