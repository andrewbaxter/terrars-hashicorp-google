#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DiscoveryEngineDataStoreData_ {
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
    acl_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_advanced_site_search: std::option::Option<terrars::PrimField<bool>>,
    data_store_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    industry_vertical: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_default_schema_creation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    solution_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_site_search_config: std::option::Option<std::vec::Vec<DiscoveryEngineDataStore_AdvancedSiteSearchConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    document_processing_config: std::option::Option<std::vec::Vec<DiscoveryEngineDataStore_DocumentProcessingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DiscoveryEngineDataStore_Timeouts>,
    dynamic: DiscoveryEngineDataStoreDynamic,
}

struct DiscoveryEngineDataStore_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DiscoveryEngineDataStoreData_>,
}

#[derive(Clone)]
pub struct DiscoveryEngineDataStore(std::rc::Rc<DiscoveryEngineDataStore_>);

impl DiscoveryEngineDataStore {
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
        "Set the field `acl_enabled`.\nImmutable. Whether data in the DataStore has ACL information. If set to 'true',\nthe source data must have ACL. ACL will be ingested when data is ingested by\nDocumentService.ImportDocuments methods. When ACL is enabled for the DataStore,\nDocument can't be accessed by calling DocumentService.GetDocument or\nDocumentService.ListDocuments. Currently ACL is only supported in the 'GENERIC'\nindustry vertical with non-'PUBLIC_WEBSITE' content config."]
    pub fn set_acl_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().acl_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `content_config`.\nThe content config of the data store. Possible values: [\"NO_CONTENT\", \"CONTENT_REQUIRED\", \"PUBLIC_WEBSITE\"]"]
    pub fn set_content_config(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().content_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `create_advanced_site_search`.\nIf true, an advanced data store for site search will be created. If the\ndata store is not configured as site search (GENERIC vertical and\nPUBLIC_WEBSITE contentConfig), this flag will be ignored."]
    pub fn set_create_advanced_site_search(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().create_advanced_site_search = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `kms_key_name`.\nKMS key resource name which will be used to encrypt resources:\n'/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}'\nThe KMS key to be used to protect this DataStore at creation time. Must be\nset for requests that need to comply with CMEK Org Policy protections.\nIf this field is set and processed successfully, the DataStore will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn set_kms_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_default_schema_creation`.\nA boolean flag indicating whether to skip the default schema creation for\nthe data store. Only enable this flag if you are certain that the default\nschema is incompatible with your use case.\nIf set to true, you must manually create a schema for the data store\nbefore any documents can be ingested.\nThis flag cannot be specified if 'data_store.starting_schema' is\nspecified."]
    pub fn set_skip_default_schema_creation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_default_schema_creation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `solution_types`.\nThe solutions that the data store enrolls. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub fn set_solution_types(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().solution_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_site_search_config`.\n"]
    pub fn set_advanced_site_search_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineDataStore_AdvancedSiteSearchConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_site_search_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_site_search_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `document_processing_config`.\n"]
    pub fn set_document_processing_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DiscoveryEngineDataStore_DocumentProcessingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().document_processing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.document_processing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DiscoveryEngineDataStore_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `acl_enabled` after provisioning.\nImmutable. Whether data in the DataStore has ACL information. If set to 'true',\nthe source data must have ACL. ACL will be ingested when data is ingested by\nDocumentService.ImportDocuments methods. When ACL is enabled for the DataStore,\nDocument can't be accessed by calling DocumentService.GetDocument or\nDocumentService.ListDocuments. Currently ACL is only supported in the 'GENERIC'\nindustry vertical with non-'PUBLIC_WEBSITE' content config."]
    pub fn acl_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_config` after provisioning.\nThe content config of the data store. Possible values: [\"NO_CONTENT\", \"CONTENT_REQUIRED\", \"PUBLIC_WEBSITE\"]"]
    pub fn content_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_advanced_site_search` after provisioning.\nIf true, an advanced data store for site search will be created. If the\ndata store is not configured as site search (GENERIC vertical and\nPUBLIC_WEBSITE contentConfig), this flag will be ignored."]
    pub fn create_advanced_site_search(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_advanced_site_search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the DataStore was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_store_id` after provisioning.\nThe unique id of the data store."]
    pub fn data_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_schema_id` after provisioning.\nThe id of the default Schema associated with this data store."]
    pub fn default_schema_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_schema_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the data store. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the data store registers. Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub fn industry_vertical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.industry_vertical", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nKMS key resource name which will be used to encrypt resources:\n'/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}'\nThe KMS key to be used to protect this DataStore at creation time. Must be\nset for requests that need to comply with CMEK Org Policy protections.\nIf this field is set and processed successfully, the DataStore will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the data store. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_default_schema_creation` after provisioning.\nA boolean flag indicating whether to skip the default schema creation for\nthe data store. Only enable this flag if you are certain that the default\nschema is incompatible with your use case.\nIf set to true, you must manually create a schema for the data store\nbefore any documents can be ingested.\nThis flag cannot be specified if 'data_store.starting_schema' is\nspecified."]
    pub fn skip_default_schema_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_default_schema_creation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `solution_types` after provisioning.\nThe solutions that the data store enrolls. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub fn solution_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.solution_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_site_search_config` after provisioning.\n"]
    pub fn advanced_site_search_config(&self) -> terrars::ListRef<DiscoveryEngineDataStore_AdvancedSiteSearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_site_search_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_processing_config` after provisioning.\n"]
    pub fn document_processing_config(&self) -> terrars::ListRef<DiscoveryEngineDataStore_DocumentProcessingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.document_processing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineDataStore_TimeoutsRef {
        DiscoveryEngineDataStore_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DiscoveryEngineDataStore {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DiscoveryEngineDataStore { }

impl terrars::ToListMappable for DiscoveryEngineDataStore {
    type O = terrars::ListRef<DiscoveryEngineDataStoreRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DiscoveryEngineDataStore_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_discovery_engine_data_store".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDiscoveryEngineDataStore {
    pub tf_id: std::string::String,
    #[doc = "The unique id of the data store."]
    pub data_store_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The display name of the data store. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The industry vertical that the data store registers. Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub industry_vertical: terrars::PrimField<std::string::String>,
    #[doc =
        "The geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineDataStore {
    pub fn build(self, stack: &mut terrars::Stack) -> DiscoveryEngineDataStore {
        let out = DiscoveryEngineDataStore(std::rc::Rc::new(DiscoveryEngineDataStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DiscoveryEngineDataStoreData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                acl_enabled: core::default::Default::default(),
                content_config: core::default::Default::default(),
                create_advanced_site_search: core::default::Default::default(),
                data_store_id: self.data_store_id,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                industry_vertical: self.industry_vertical,
                kms_key_name: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                skip_default_schema_creation: core::default::Default::default(),
                solution_types: core::default::Default::default(),
                advanced_site_search_config: core::default::Default::default(),
                document_processing_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DiscoveryEngineDataStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataStoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DiscoveryEngineDataStoreRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `acl_enabled` after provisioning.\nImmutable. Whether data in the DataStore has ACL information. If set to 'true',\nthe source data must have ACL. ACL will be ingested when data is ingested by\nDocumentService.ImportDocuments methods. When ACL is enabled for the DataStore,\nDocument can't be accessed by calling DocumentService.GetDocument or\nDocumentService.ListDocuments. Currently ACL is only supported in the 'GENERIC'\nindustry vertical with non-'PUBLIC_WEBSITE' content config."]
    pub fn acl_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `content_config` after provisioning.\nThe content config of the data store. Possible values: [\"NO_CONTENT\", \"CONTENT_REQUIRED\", \"PUBLIC_WEBSITE\"]"]
    pub fn content_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_advanced_site_search` after provisioning.\nIf true, an advanced data store for site search will be created. If the\ndata store is not configured as site search (GENERIC vertical and\nPUBLIC_WEBSITE contentConfig), this flag will be ignored."]
    pub fn create_advanced_site_search(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_advanced_site_search", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTimestamp when the DataStore was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_store_id` after provisioning.\nThe unique id of the data store."]
    pub fn data_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_schema_id` after provisioning.\nThe id of the default Schema associated with this data store."]
    pub fn default_schema_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_schema_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the data store. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `industry_vertical` after provisioning.\nThe industry vertical that the data store registers. Possible values: [\"GENERIC\", \"MEDIA\", \"HEALTHCARE_FHIR\"]"]
    pub fn industry_vertical(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.industry_vertical", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nKMS key resource name which will be used to encrypt resources:\n'/{project}/locations/{location}/keyRings/{keyRing}/cryptoKeys/{keyId}'\nThe KMS key to be used to protect this DataStore at creation time. Must be\nset for requests that need to comply with CMEK Org Policy protections.\nIf this field is set and processed successfully, the DataStore will be\nprotected by the KMS key, as indicated in the cmek_config field."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique full resource name of the data store. Values are of the format\n'projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}'.\nThis field must be a UTF-8 encoded string with a length limit of 1024\ncharacters."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_default_schema_creation` after provisioning.\nA boolean flag indicating whether to skip the default schema creation for\nthe data store. Only enable this flag if you are certain that the default\nschema is incompatible with your use case.\nIf set to true, you must manually create a schema for the data store\nbefore any documents can be ingested.\nThis flag cannot be specified if 'data_store.starting_schema' is\nspecified."]
    pub fn skip_default_schema_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_default_schema_creation", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `solution_types` after provisioning.\nThe solutions that the data store enrolls. Possible values: [\"SOLUTION_TYPE_RECOMMENDATION\", \"SOLUTION_TYPE_SEARCH\", \"SOLUTION_TYPE_CHAT\", \"SOLUTION_TYPE_GENERATIVE_CHAT\"]"]
    pub fn solution_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.solution_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `advanced_site_search_config` after provisioning.\n"]
    pub fn advanced_site_search_config(&self) -> terrars::ListRef<DiscoveryEngineDataStore_AdvancedSiteSearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_site_search_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `document_processing_config` after provisioning.\n"]
    pub fn document_processing_config(&self) -> terrars::ListRef<DiscoveryEngineDataStore_DocumentProcessingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.document_processing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DiscoveryEngineDataStore_TimeoutsRef {
        DiscoveryEngineDataStore_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineDataStoreDynamic {
    advanced_site_search_config: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineDataStore_AdvancedSiteSearchConfig>>,
    document_processing_config: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineDataStore_DocumentProcessingConfig>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataStore_AdvancedSiteSearchConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_automatic_refresh: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_initial_index: std::option::Option<terrars::PrimField<bool>>,
}

impl DiscoveryEngineDataStore_AdvancedSiteSearchConfig {
    #[doc =
        "Set the field `disable_automatic_refresh`.\nIf set true, automatic refresh is disabled for the DataStore."]
    pub fn set_disable_automatic_refresh(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_automatic_refresh = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_initial_index`.\nIf set true, initial indexing is disabled for the DataStore."]
    pub fn set_disable_initial_index(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_initial_index = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataStore_AdvancedSiteSearchConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineDataStore_AdvancedSiteSearchConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataStore_AdvancedSiteSearchConfig {}

impl BuildDiscoveryEngineDataStore_AdvancedSiteSearchConfig {
    pub fn build(self) -> DiscoveryEngineDataStore_AdvancedSiteSearchConfig {
        DiscoveryEngineDataStore_AdvancedSiteSearchConfig {
            disable_automatic_refresh: core::default::Default::default(),
            disable_initial_index: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataStore_AdvancedSiteSearchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataStore_AdvancedSiteSearchConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineDataStore_AdvancedSiteSearchConfigRef {
        DiscoveryEngineDataStore_AdvancedSiteSearchConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataStore_AdvancedSiteSearchConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_automatic_refresh` after provisioning.\nIf set true, automatic refresh is disabled for the DataStore."]
    pub fn disable_automatic_refresh(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_automatic_refresh", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_initial_index` after provisioning.\nIf set true, initial indexing is disabled for the DataStore."]
    pub fn disable_initial_index(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_initial_index", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chunk_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_ancestor_headings: std::option::Option<terrars::PrimField<bool>>,
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
    #[doc =
        "Set the field `chunk_size`.\nThe token size limit for each chunk.\nSupported values: 100-500 (inclusive). Default value: 500."]
    pub fn set_chunk_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.chunk_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_ancestor_headings`.\nWhether to include appending different levels of headings to chunks from the middle of the document to prevent context loss.\nDefault value: False."]
    pub fn set_include_ancestor_headings(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_ancestor_headings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
    type O =
        terrars
        ::BlockAssignable<DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {}

impl BuildDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
    pub fn build(self) -> DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
        DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
            chunk_size: core::default::Default::default(),
            include_ancestor_headings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef {
        DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `chunk_size` after provisioning.\nThe token size limit for each chunk.\nSupported values: 100-500 (inclusive). Default value: 500."]
    pub fn chunk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chunk_size", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_ancestor_headings` after provisioning.\nWhether to include appending different levels of headings to chunks from the middle of the document to prevent context loss.\nDefault value: False."]
    pub fn include_ancestor_headings(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_ancestor_headings", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigDynamic {
    layout_based_chunking_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig>,
    >,
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    layout_based_chunking_config: std
    ::option
    ::Option<
        std::vec::Vec<DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig>,
    >,
    dynamic: DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigDynamic,
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
    #[doc = "Set the field `layout_based_chunking_config`.\n"]
    pub fn set_layout_based_chunking_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.layout_based_chunking_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.layout_based_chunking_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {}

impl BuildDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
    pub fn build(self) -> DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
        DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
            layout_based_chunking_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef {
        DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `layout_based_chunking_config` after provisioning.\n"]
    pub fn layout_based_chunking_config(
        &self,
    ) -> terrars
    ::ListRef<DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layout_based_chunking_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig { }

impl terrars
::ToListMappable for DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {
    type O =
        terrars
        ::BlockAssignable<
            DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {}

impl BuildDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {
    pub fn build(
        self,
    ) -> DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {
        DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {}
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef {
        DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_get_processed_document: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_image_annotation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_llm_layout_parsing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_table_annotation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_html_classes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_html_elements: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_html_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    structured_content_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
    #[doc =
        "Set the field `enable_get_processed_document`.\nIf true, the processed document will be made available for the GetProcessedDocument API."]
    pub fn set_enable_get_processed_document(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_get_processed_document = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_image_annotation`.\nIf true, the LLM based annotation is added to the image during parsing."]
    pub fn set_enable_image_annotation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_image_annotation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_llm_layout_parsing`.\nIf true, the pdf layout will be refined using an LLM."]
    pub fn set_enable_llm_layout_parsing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_llm_layout_parsing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_table_annotation`.\nIf true, the LLM based annotation is added to the table during parsing."]
    pub fn set_enable_table_annotation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_table_annotation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_html_classes`.\nList of HTML classes to exclude from the parsed content."]
    pub fn set_exclude_html_classes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_html_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_html_elements`.\nList of HTML elements to exclude from the parsed content."]
    pub fn set_exclude_html_elements(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_html_elements = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_html_ids`.\nList of HTML ids to exclude from the parsed content."]
    pub fn set_exclude_html_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_html_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `structured_content_types`.\nContains the required structure types to extract from the document. Supported values: 'shareholder-structure'."]
    pub fn set_structured_content_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.structured_content_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
    type O =
        terrars
        ::BlockAssignable<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {}

impl BuildDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
    pub fn build(self) -> DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
        DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
            enable_get_processed_document: core::default::Default::default(),
            enable_image_annotation: core::default::Default::default(),
            enable_llm_layout_parsing: core::default::Default::default(),
            enable_table_annotation: core::default::Default::default(),
            exclude_html_classes: core::default::Default::default(),
            exclude_html_elements: core::default::Default::default(),
            exclude_html_ids: core::default::Default::default(),
            structured_content_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef {
        DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_get_processed_document` after provisioning.\nIf true, the processed document will be made available for the GetProcessedDocument API."]
    pub fn enable_get_processed_document(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_get_processed_document", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_image_annotation` after provisioning.\nIf true, the LLM based annotation is added to the image during parsing."]
    pub fn enable_image_annotation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_image_annotation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_llm_layout_parsing` after provisioning.\nIf true, the pdf layout will be refined using an LLM."]
    pub fn enable_llm_layout_parsing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_llm_layout_parsing", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_table_annotation` after provisioning.\nIf true, the LLM based annotation is added to the table during parsing."]
    pub fn enable_table_annotation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_table_annotation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_html_classes` after provisioning.\nList of HTML classes to exclude from the parsed content."]
    pub fn exclude_html_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_html_classes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_html_elements` after provisioning.\nList of HTML elements to exclude from the parsed content."]
    pub fn exclude_html_elements(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_html_elements", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_html_ids` after provisioning.\nList of HTML ids to exclude from the parsed content."]
    pub fn exclude_html_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_html_ids", self.base))
    }

    #[doc =
        "Get a reference to the value of field `structured_content_types` after provisioning.\nContains the required structure types to extract from the document. Supported values: 'shareholder-structure'."]
    pub fn structured_content_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.structured_content_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_native_text: std::option::Option<terrars::PrimField<bool>>,
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
    #[doc =
        "Set the field `use_native_text`.\nIf true, will use native text instead of OCR text on pages containing native text."]
    pub fn set_use_native_text(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_native_text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
    type O =
        terrars
        ::BlockAssignable<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {}

impl BuildDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
    pub fn build(self) -> DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
        DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
            use_native_text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef {
        DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `use_native_text` after provisioning.\nIf true, will use native text instead of OCR text on pages containing native text."]
    pub fn use_native_text(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_native_text", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigDynamic {
    digital_parsing_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig>,
    >,
    layout_parsing_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig>,
    >,
    ocr_parsing_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig>,
    >,
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digital_parsing_config: std
    ::option
    ::Option<
        std::vec::Vec<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    layout_parsing_config: std
    ::option
    ::Option<
        std::vec::Vec<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocr_parsing_config: std
    ::option
    ::Option<std::vec::Vec<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig>>,
    dynamic: DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigDynamic,
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
    #[doc = "Set the field `digital_parsing_config`.\n"]
    pub fn set_digital_parsing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.digital_parsing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.digital_parsing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `layout_parsing_config`.\n"]
    pub fn set_layout_parsing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.layout_parsing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.layout_parsing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ocr_parsing_config`.\n"]
    pub fn set_ocr_parsing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ocr_parsing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ocr_parsing_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {}

impl BuildDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
    pub fn build(self) -> DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
        DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
            digital_parsing_config: core::default::Default::default(),
            layout_parsing_config: core::default::Default::default(),
            ocr_parsing_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef {
        DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `digital_parsing_config` after provisioning.\n"]
    pub fn digital_parsing_config(
        &self,
    ) -> terrars
    ::ListRef<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.digital_parsing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `layout_parsing_config` after provisioning.\n"]
    pub fn layout_parsing_config(
        &self,
    ) -> terrars
    ::ListRef<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layout_parsing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ocr_parsing_config` after provisioning.\n"]
    pub fn ocr_parsing_config(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ocr_parsing_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesDynamic {
    digital_parsing_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig>,
    >,
    layout_parsing_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig>,
    >,
    ocr_parsing_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig>,
    >,
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
    file_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digital_parsing_config: std
    ::option
    ::Option<
        std::vec::Vec<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    layout_parsing_config: std
    ::option
    ::Option<
        std::vec::Vec<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocr_parsing_config: std
    ::option
    ::Option<std::vec::Vec<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig>>,
    dynamic: DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesDynamic,
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
    #[doc = "Set the field `digital_parsing_config`.\n"]
    pub fn set_digital_parsing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.digital_parsing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.digital_parsing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `layout_parsing_config`.\n"]
    pub fn set_layout_parsing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.layout_parsing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.layout_parsing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ocr_parsing_config`.\n"]
    pub fn set_ocr_parsing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ocr_parsing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ocr_parsing_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
    type O = terrars::BlockAssignable<DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
    #[doc = ""]
    pub file_type: terrars::PrimField<std::string::String>,
}

impl BuildDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
    pub fn build(self) -> DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
        DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
            file_type: self.file_type,
            digital_parsing_config: core::default::Default::default(),
            layout_parsing_config: core::default::Default::default(),
            ocr_parsing_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesRef {
        DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\n"]
    pub fn file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }

    #[doc = "Get a reference to the value of field `digital_parsing_config` after provisioning.\n"]
    pub fn digital_parsing_config(
        &self,
    ) -> terrars
    ::ListRef<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.digital_parsing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `layout_parsing_config` after provisioning.\n"]
    pub fn layout_parsing_config(
        &self,
    ) -> terrars
    ::ListRef<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layout_parsing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ocr_parsing_config` after provisioning.\n"]
    pub fn ocr_parsing_config(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ocr_parsing_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct DiscoveryEngineDataStore_DocumentProcessingConfigDynamic {
    chunking_config: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig>>,
    default_parsing_config: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig>>,
    parsing_config_overrides: std
    ::option
    ::Option<terrars::DynamicBlock<DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides>>,
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chunking_config: std::option::Option<std::vec::Vec<DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_parsing_config: std
    ::option
    ::Option<std::vec::Vec<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parsing_config_overrides: std
    ::option
    ::Option<std::vec::Vec<DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides>>,
    dynamic: DiscoveryEngineDataStore_DocumentProcessingConfigDynamic,
}

impl DiscoveryEngineDataStore_DocumentProcessingConfig {
    #[doc = "Set the field `chunking_config`.\n"]
    pub fn set_chunking_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.chunking_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.chunking_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_parsing_config`.\n"]
    pub fn set_default_parsing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_parsing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_parsing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parsing_config_overrides`.\n"]
    pub fn set_parsing_config_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parsing_config_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parsing_config_overrides = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DiscoveryEngineDataStore_DocumentProcessingConfig {
    type O = terrars::BlockAssignable<DiscoveryEngineDataStore_DocumentProcessingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataStore_DocumentProcessingConfig {}

impl BuildDiscoveryEngineDataStore_DocumentProcessingConfig {
    pub fn build(self) -> DiscoveryEngineDataStore_DocumentProcessingConfig {
        DiscoveryEngineDataStore_DocumentProcessingConfig {
            chunking_config: core::default::Default::default(),
            default_parsing_config: core::default::Default::default(),
            parsing_config_overrides: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataStore_DocumentProcessingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataStore_DocumentProcessingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DiscoveryEngineDataStore_DocumentProcessingConfigRef {
        DiscoveryEngineDataStore_DocumentProcessingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataStore_DocumentProcessingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe full resource name of the Document Processing Config. Format:\n'projects/{project}/locations/{location}/collections/{collection_id}/dataStores/{data_store_id}/documentProcessingConfig'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `chunking_config` after provisioning.\n"]
    pub fn chunking_config(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chunking_config", self.base))
    }

    #[doc = "Get a reference to the value of field `default_parsing_config` after provisioning.\n"]
    pub fn default_parsing_config(
        &self,
    ) -> terrars::ListRef<DiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_parsing_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DiscoveryEngineDataStore_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DiscoveryEngineDataStore_Timeouts {
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

impl terrars::ToListMappable for DiscoveryEngineDataStore_Timeouts {
    type O = terrars::BlockAssignable<DiscoveryEngineDataStore_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDiscoveryEngineDataStore_Timeouts {}

impl BuildDiscoveryEngineDataStore_Timeouts {
    pub fn build(self) -> DiscoveryEngineDataStore_Timeouts {
        DiscoveryEngineDataStore_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DiscoveryEngineDataStore_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DiscoveryEngineDataStore_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DiscoveryEngineDataStore_TimeoutsRef {
        DiscoveryEngineDataStore_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DiscoveryEngineDataStore_TimeoutsRef {
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
