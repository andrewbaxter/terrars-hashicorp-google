#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataVmwareengineDatastoreData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataVmwareengineDatastore_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVmwareengineDatastoreData_>,
}

#[derive(Clone)]
pub struct DataVmwareengineDatastore(std::rc::Rc<DataVmwareengineDatastore_>);

impl DataVmwareengineDatastore {
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
        "Get a reference to the value of field `clusters` after provisioning.\nClusters to which the datastore is attached."]
    pub fn clusters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this datastore"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe user-provided identifier of the datastore to be created.\nThis identifier must be unique among each 'Datastore' within the parent\nand becomes the final token in the name URI.\nThe identifier must meet the following requirements:\n\n* Only contains 1-63 alphanumeric characters and hyphens\n* Begins with an alphabetical character\n* Ends with a non-hyphen character\n* Not formatted as a UUID\n* Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034)\n(section 3.5)"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nfs_datastore` after provisioning.\nThe NFS datastore configuration."]
    pub fn nfs_datastore(&self) -> terrars::ListRef<DataVmwareengineDatastore_NfsDatastoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_datastore", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the Datastore.\nPossible values:\nCREATING\nACTIVE\nUPDATING\nDELETING\nSOFT_DELETING\nSOFT_DELETED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nLast update time of this resource."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataVmwareengineDatastore {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVmwareengineDatastore { }

impl terrars::ToListMappable for DataVmwareengineDatastore {
    type O = terrars::ListRef<DataVmwareengineDatastoreRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVmwareengineDatastore_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_vmwareengine_datastore".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVmwareengineDatastore {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The user-provided identifier of the datastore to be created.\nThis identifier must be unique among each 'Datastore' within the parent\nand becomes the final token in the name URI.\nThe identifier must meet the following requirements:\n\n* Only contains 1-63 alphanumeric characters and hyphens\n* Begins with an alphabetical character\n* Ends with a non-hyphen character\n* Not formatted as a UUID\n* Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034)\n(section 3.5)"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataVmwareengineDatastore {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVmwareengineDatastore {
        let out = DataVmwareengineDatastore(std::rc::Rc::new(DataVmwareengineDatastore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVmwareengineDatastoreData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVmwareengineDatastoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineDatastoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVmwareengineDatastoreRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `clusters` after provisioning.\nClusters to which the datastore is attached."]
    pub fn clusters(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.clusters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nCreation time of this resource."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description for this datastore"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe user-provided identifier of the datastore to be created.\nThis identifier must be unique among each 'Datastore' within the parent\nand becomes the final token in the name URI.\nThe identifier must meet the following requirements:\n\n* Only contains 1-63 alphanumeric characters and hyphens\n* Begins with an alphabetical character\n* Ends with a non-hyphen character\n* Not formatted as a UUID\n* Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034)\n(section 3.5)"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `nfs_datastore` after provisioning.\nThe NFS datastore configuration."]
    pub fn nfs_datastore(&self) -> terrars::ListRef<DataVmwareengineDatastore_NfsDatastoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_datastore", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the Datastore.\nPossible values:\nCREATING\nACTIVE\nUPDATING\nDELETING\nSOFT_DELETING\nSOFT_DELETED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nSystem-generated unique identifier for the resource."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nLast update time of this resource."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineDatastore_NfsDatastore_GoogleFileService {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filestore_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    netapp_volume: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVmwareengineDatastore_NfsDatastore_GoogleFileService {
    #[doc = "Set the field `filestore_instance`.\n"]
    pub fn set_filestore_instance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filestore_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `netapp_volume`.\n"]
    pub fn set_netapp_volume(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.netapp_volume = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineDatastore_NfsDatastore_GoogleFileService {
    type O = terrars::BlockAssignable<DataVmwareengineDatastore_NfsDatastore_GoogleFileService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineDatastore_NfsDatastore_GoogleFileService {}

impl BuildDataVmwareengineDatastore_NfsDatastore_GoogleFileService {
    pub fn build(self) -> DataVmwareengineDatastore_NfsDatastore_GoogleFileService {
        DataVmwareengineDatastore_NfsDatastore_GoogleFileService {
            filestore_instance: core::default::Default::default(),
            netapp_volume: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineDatastore_NfsDatastore_GoogleFileServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineDatastore_NfsDatastore_GoogleFileServiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineDatastore_NfsDatastore_GoogleFileServiceRef {
        DataVmwareengineDatastore_NfsDatastore_GoogleFileServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineDatastore_NfsDatastore_GoogleFileServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filestore_instance` after provisioning.\n"]
    pub fn filestore_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filestore_instance", self.base))
    }

    #[doc = "Get a reference to the value of field `netapp_volume` after provisioning.\n"]
    pub fn netapp_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.netapp_volume", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_share: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    servers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
    #[doc = "Set the field `file_share`.\n"]
    pub fn set_file_share(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_share = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network`.\n"]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `servers`.\n"]
    pub fn set_servers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.servers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
    type O = terrars::BlockAssignable<DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineDatastore_NfsDatastore_ThirdPartyFileService {}

impl BuildDataVmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
    pub fn build(self) -> DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
        DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
            file_share: core::default::Default::default(),
            network: core::default::Default::default(),
            servers: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef {
        DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_share` after provisioning.\n"]
    pub fn file_share(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_share", self.base))
    }

    #[doc = "Get a reference to the value of field `network` after provisioning.\n"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc = "Get a reference to the value of field `servers` after provisioning.\n"]
    pub fn servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVmwareengineDatastore_NfsDatastore {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_file_service: std
    ::option
    ::Option<terrars::ListField<DataVmwareengineDatastore_NfsDatastore_GoogleFileService>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    third_party_file_service: std
    ::option
    ::Option<terrars::ListField<DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileService>>,
}

impl DataVmwareengineDatastore_NfsDatastore {
    #[doc = "Set the field `google_file_service`.\n"]
    pub fn set_google_file_service(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVmwareengineDatastore_NfsDatastore_GoogleFileService>>,
    ) -> Self {
        self.google_file_service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `third_party_file_service`.\n"]
    pub fn set_third_party_file_service(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileService>>,
    ) -> Self {
        self.third_party_file_service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVmwareengineDatastore_NfsDatastore {
    type O = terrars::BlockAssignable<DataVmwareengineDatastore_NfsDatastore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVmwareengineDatastore_NfsDatastore {}

impl BuildDataVmwareengineDatastore_NfsDatastore {
    pub fn build(self) -> DataVmwareengineDatastore_NfsDatastore {
        DataVmwareengineDatastore_NfsDatastore {
            google_file_service: core::default::Default::default(),
            third_party_file_service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVmwareengineDatastore_NfsDatastoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVmwareengineDatastore_NfsDatastoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVmwareengineDatastore_NfsDatastoreRef {
        DataVmwareengineDatastore_NfsDatastoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVmwareengineDatastore_NfsDatastoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `google_file_service` after provisioning.\n"]
    pub fn google_file_service(&self) -> terrars::ListRef<DataVmwareengineDatastore_NfsDatastore_GoogleFileServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_file_service", self.base))
    }

    #[doc = "Get a reference to the value of field `third_party_file_service` after provisioning.\n"]
    pub fn third_party_file_service(
        &self,
    ) -> terrars::ListRef<DataVmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.third_party_file_service", self.base))
    }
}
