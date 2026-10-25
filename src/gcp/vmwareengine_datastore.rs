#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VmwareengineDatastoreData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs_datastore: std::option::Option<std::vec::Vec<VmwareengineDatastore_NfsDatastore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VmwareengineDatastore_Timeouts>,
    dynamic: VmwareengineDatastoreDynamic,
}

struct VmwareengineDatastore_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VmwareengineDatastoreData_>,
}

#[derive(Clone)]
pub struct VmwareengineDatastore(std::rc::Rc<VmwareengineDatastore_>);

impl VmwareengineDatastore {
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

    #[doc = "Set the field `description`.\nUser-provided description for this datastore"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `nfs_datastore`.\n"]
    pub fn set_nfs_datastore(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VmwareengineDatastore_NfsDatastore>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nfs_datastore = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nfs_datastore = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VmwareengineDatastore_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `nfs_datastore` after provisioning.\n"]
    pub fn nfs_datastore(&self) -> terrars::ListRef<VmwareengineDatastore_NfsDatastoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_datastore", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineDatastore_TimeoutsRef {
        VmwareengineDatastore_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VmwareengineDatastore {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VmwareengineDatastore { }

impl terrars::ToListMappable for VmwareengineDatastore {
    type O = terrars::ListRef<VmwareengineDatastoreRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VmwareengineDatastore_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vmwareengine_datastore".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVmwareengineDatastore {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "The user-provided identifier of the datastore to be created.\nThis identifier must be unique among each 'Datastore' within the parent\nand becomes the final token in the name URI.\nThe identifier must meet the following requirements:\n\n* Only contains 1-63 alphanumeric characters and hyphens\n* Begins with an alphabetical character\n* Ends with a non-hyphen character\n* Not formatted as a UUID\n* Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034)\n(section 3.5)"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildVmwareengineDatastore {
    pub fn build(self, stack: &mut terrars::Stack) -> VmwareengineDatastore {
        let out = VmwareengineDatastore(std::rc::Rc::new(VmwareengineDatastore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VmwareengineDatastoreData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                nfs_datastore: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VmwareengineDatastoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineDatastoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VmwareengineDatastoreRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `nfs_datastore` after provisioning.\n"]
    pub fn nfs_datastore(&self) -> terrars::ListRef<VmwareengineDatastore_NfsDatastoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_datastore", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VmwareengineDatastore_TimeoutsRef {
        VmwareengineDatastore_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VmwareengineDatastoreDynamic {
    nfs_datastore: std::option::Option<terrars::DynamicBlock<VmwareengineDatastore_NfsDatastore>>,
}

#[derive(Serialize)]
pub struct VmwareengineDatastore_NfsDatastore_GoogleFileService {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filestore_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    netapp_volume: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VmwareengineDatastore_NfsDatastore_GoogleFileService {
    #[doc =
        "Set the field `filestore_instance`.\nGoogle filestore instance resource name\ne.g. projects/my-project/locations/me-west1-b/instances/my-instance"]
    pub fn set_filestore_instance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.filestore_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `netapp_volume`.\nGoogle netapp volume resource name\ne.g. projects/my-project/locations/me-west1-b/volumes/my-volume"]
    pub fn set_netapp_volume(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.netapp_volume = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VmwareengineDatastore_NfsDatastore_GoogleFileService {
    type O = terrars::BlockAssignable<VmwareengineDatastore_NfsDatastore_GoogleFileService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineDatastore_NfsDatastore_GoogleFileService {}

impl BuildVmwareengineDatastore_NfsDatastore_GoogleFileService {
    pub fn build(self) -> VmwareengineDatastore_NfsDatastore_GoogleFileService {
        VmwareengineDatastore_NfsDatastore_GoogleFileService {
            filestore_instance: core::default::Default::default(),
            netapp_volume: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineDatastore_NfsDatastore_GoogleFileServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineDatastore_NfsDatastore_GoogleFileServiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareengineDatastore_NfsDatastore_GoogleFileServiceRef {
        VmwareengineDatastore_NfsDatastore_GoogleFileServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineDatastore_NfsDatastore_GoogleFileServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `filestore_instance` after provisioning.\nGoogle filestore instance resource name\ne.g. projects/my-project/locations/me-west1-b/instances/my-instance"]
    pub fn filestore_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filestore_instance", self.base))
    }

    #[doc =
        "Get a reference to the value of field `netapp_volume` after provisioning.\nGoogle netapp volume resource name\ne.g. projects/my-project/locations/me-west1-b/volumes/my-volume"]
    pub fn netapp_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.netapp_volume", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
    file_share: terrars::PrimField<std::string::String>,
    network: terrars::PrimField<std::string::String>,
    servers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl VmwareengineDatastore_NfsDatastore_ThirdPartyFileService { }

impl terrars::ToListMappable for VmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
    type O = terrars::BlockAssignable<VmwareengineDatastore_NfsDatastore_ThirdPartyFileService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
    #[doc = "Required\nMount Folder name"]
    pub file_share: terrars::PrimField<std::string::String>,
    #[doc =
        "Required to identify vpc peering used for NFS access\nnetwork name of NFS's vpc\ne.g. projects/project-id/global/networks/my-network_id"]
    pub network: terrars::PrimField<std::string::String>,
    #[doc =
        "Server IP addresses of the NFS file service.\nNFS v3, provide a single IP address or DNS name.\nMultiple servers can be supported in future when NFS 4.1 protocol support\nis enabled."]
    pub servers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildVmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
    pub fn build(self) -> VmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
        VmwareengineDatastore_NfsDatastore_ThirdPartyFileService {
            file_share: self.file_share,
            network: self.network,
            servers: self.servers,
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef {
        VmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_share` after provisioning.\nRequired\nMount Folder name"]
    pub fn file_share(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_share", self.base))
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nRequired to identify vpc peering used for NFS access\nnetwork name of NFS's vpc\ne.g. projects/project-id/global/networks/my-network_id"]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `servers` after provisioning.\nServer IP addresses of the NFS file service.\nNFS v3, provide a single IP address or DNS name.\nMultiple servers can be supported in future when NFS 4.1 protocol support\nis enabled."]
    pub fn servers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.servers", self.base))
    }
}

#[derive(Serialize, Default)]
struct VmwareengineDatastore_NfsDatastoreDynamic {
    google_file_service: std
    ::option
    ::Option<terrars::DynamicBlock<VmwareengineDatastore_NfsDatastore_GoogleFileService>>,
    third_party_file_service: std
    ::option
    ::Option<terrars::DynamicBlock<VmwareengineDatastore_NfsDatastore_ThirdPartyFileService>>,
}

#[derive(Serialize)]
pub struct VmwareengineDatastore_NfsDatastore {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_file_service: std::option::Option<std::vec::Vec<VmwareengineDatastore_NfsDatastore_GoogleFileService>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    third_party_file_service: std
    ::option
    ::Option<std::vec::Vec<VmwareengineDatastore_NfsDatastore_ThirdPartyFileService>>,
    dynamic: VmwareengineDatastore_NfsDatastoreDynamic,
}

impl VmwareengineDatastore_NfsDatastore {
    #[doc = "Set the field `google_file_service`.\n"]
    pub fn set_google_file_service(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VmwareengineDatastore_NfsDatastore_GoogleFileService>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_file_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_file_service = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `third_party_file_service`.\n"]
    pub fn set_third_party_file_service(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VmwareengineDatastore_NfsDatastore_ThirdPartyFileService>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.third_party_file_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.third_party_file_service = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VmwareengineDatastore_NfsDatastore {
    type O = terrars::BlockAssignable<VmwareengineDatastore_NfsDatastore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineDatastore_NfsDatastore {}

impl BuildVmwareengineDatastore_NfsDatastore {
    pub fn build(self) -> VmwareengineDatastore_NfsDatastore {
        VmwareengineDatastore_NfsDatastore {
            google_file_service: core::default::Default::default(),
            third_party_file_service: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineDatastore_NfsDatastoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineDatastore_NfsDatastoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareengineDatastore_NfsDatastoreRef {
        VmwareengineDatastore_NfsDatastoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineDatastore_NfsDatastoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `google_file_service` after provisioning.\n"]
    pub fn google_file_service(&self) -> terrars::ListRef<VmwareengineDatastore_NfsDatastore_GoogleFileServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_file_service", self.base))
    }

    #[doc = "Get a reference to the value of field `third_party_file_service` after provisioning.\n"]
    pub fn third_party_file_service(
        &self,
    ) -> terrars::ListRef<VmwareengineDatastore_NfsDatastore_ThirdPartyFileServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.third_party_file_service", self.base))
    }
}

#[derive(Serialize)]
pub struct VmwareengineDatastore_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VmwareengineDatastore_Timeouts {
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

impl terrars::ToListMappable for VmwareengineDatastore_Timeouts {
    type O = terrars::BlockAssignable<VmwareengineDatastore_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVmwareengineDatastore_Timeouts {}

impl BuildVmwareengineDatastore_Timeouts {
    pub fn build(self) -> VmwareengineDatastore_Timeouts {
        VmwareengineDatastore_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VmwareengineDatastore_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VmwareengineDatastore_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VmwareengineDatastore_TimeoutsRef {
        VmwareengineDatastore_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VmwareengineDatastore_TimeoutsRef {
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
