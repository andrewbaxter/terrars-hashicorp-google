#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataDiscoveryEngineDataStoreData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_store_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataDiscoveryEngineDataStore_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDiscoveryEngineDataStoreData_>,
}

#[derive(Clone)]
pub struct DataDiscoveryEngineDataStore(std::rc::Rc<DataDiscoveryEngineDataStore_>);

impl DataDiscoveryEngineDataStore {
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

    #[doc = "Set the field `data_store_id`.\nThe unique id of the data store."]
    pub fn set_data_store_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data_store_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `display_name`.\nThe display name of the data store. This field must be a UTF-8 encoded\nstring with a length limit of 128 characters."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `location`.\nThe geographic location where the data store should reside. The value can\nonly be one of \"global\", \"us\" and \"eu\"."]
    pub fn set_location(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `acl_enabled` after provisioning.\nImmutable. Whether data in the DataStore has ACL information. If set to 'true',\nthe source data must have ACL. ACL will be ingested when data is ingested by\nDocumentService.ImportDocuments methods. When ACL is enabled for the DataStore,\nDocument can't be accessed by calling DocumentService.GetDocument or\nDocumentService.ListDocuments. Currently ACL is only supported in the 'GENERIC'\nindustry vertical with non-'PUBLIC_WEBSITE' content config."]
    pub fn acl_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advanced_site_search_config` after provisioning.\nConfiguration data for advance site search."]
    pub fn advanced_site_search_config(
        &self,
    ) -> terrars::ListRef<DataDiscoveryEngineDataStore_AdvancedSiteSearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_site_search_config", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `document_processing_config` after provisioning.\nConfiguration for Document understanding and enrichment."]
    pub fn document_processing_config(
        &self,
    ) -> terrars::ListRef<DataDiscoveryEngineDataStore_DocumentProcessingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.document_processing_config", self.extract_ref()))
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
}

impl terrars::Referable for DataDiscoveryEngineDataStore {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDiscoveryEngineDataStore { }

impl terrars::ToListMappable for DataDiscoveryEngineDataStore {
    type O = terrars::ListRef<DataDiscoveryEngineDataStoreRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDiscoveryEngineDataStore_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_discovery_engine_data_store".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDiscoveryEngineDataStore {
    pub tf_id: std::string::String,
}

impl BuildDataDiscoveryEngineDataStore {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDiscoveryEngineDataStore {
        let out = DataDiscoveryEngineDataStore(std::rc::Rc::new(DataDiscoveryEngineDataStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDiscoveryEngineDataStoreData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                data_store_id: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDiscoveryEngineDataStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDiscoveryEngineDataStoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDiscoveryEngineDataStoreRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `acl_enabled` after provisioning.\nImmutable. Whether data in the DataStore has ACL information. If set to 'true',\nthe source data must have ACL. ACL will be ingested when data is ingested by\nDocumentService.ImportDocuments methods. When ACL is enabled for the DataStore,\nDocument can't be accessed by calling DocumentService.GetDocument or\nDocumentService.ListDocuments. Currently ACL is only supported in the 'GENERIC'\nindustry vertical with non-'PUBLIC_WEBSITE' content config."]
    pub fn acl_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acl_enabled", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `advanced_site_search_config` after provisioning.\nConfiguration data for advance site search."]
    pub fn advanced_site_search_config(
        &self,
    ) -> terrars::ListRef<DataDiscoveryEngineDataStore_AdvancedSiteSearchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_site_search_config", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `document_processing_config` after provisioning.\nConfiguration for Document understanding and enrichment."]
    pub fn document_processing_config(
        &self,
    ) -> terrars::ListRef<DataDiscoveryEngineDataStore_DocumentProcessingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.document_processing_config", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStore_AdvancedSiteSearchConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_automatic_refresh: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_initial_index: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDiscoveryEngineDataStore_AdvancedSiteSearchConfig {
    #[doc = "Set the field `disable_automatic_refresh`.\n"]
    pub fn set_disable_automatic_refresh(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_automatic_refresh = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_initial_index`.\n"]
    pub fn set_disable_initial_index(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_initial_index = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDiscoveryEngineDataStore_AdvancedSiteSearchConfig {
    type O = terrars::BlockAssignable<DataDiscoveryEngineDataStore_AdvancedSiteSearchConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDiscoveryEngineDataStore_AdvancedSiteSearchConfig {}

impl BuildDataDiscoveryEngineDataStore_AdvancedSiteSearchConfig {
    pub fn build(self) -> DataDiscoveryEngineDataStore_AdvancedSiteSearchConfig {
        DataDiscoveryEngineDataStore_AdvancedSiteSearchConfig {
            disable_automatic_refresh: core::default::Default::default(),
            disable_initial_index: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDiscoveryEngineDataStore_AdvancedSiteSearchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDiscoveryEngineDataStore_AdvancedSiteSearchConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDiscoveryEngineDataStore_AdvancedSiteSearchConfigRef {
        DataDiscoveryEngineDataStore_AdvancedSiteSearchConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDiscoveryEngineDataStore_AdvancedSiteSearchConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_automatic_refresh` after provisioning.\n"]
    pub fn disable_automatic_refresh(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_automatic_refresh", self.base))
    }

    #[doc = "Get a reference to the value of field `disable_initial_index` after provisioning.\n"]
    pub fn disable_initial_index(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_initial_index", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chunk_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_ancestor_headings: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
    #[doc = "Set the field `chunk_size`.\n"]
    pub fn set_chunk_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.chunk_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_ancestor_headings`.\n"]
    pub fn set_include_ancestor_headings(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_ancestor_headings = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {}

impl BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
    pub fn build(
        self,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig {
            chunk_size: core::default::Default::default(),
            include_ancestor_headings: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chunk_size` after provisioning.\n"]
    pub fn chunk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chunk_size", self.base))
    }

    #[doc = "Get a reference to the value of field `include_ancestor_headings` after provisioning.\n"]
    pub fn include_ancestor_headings(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_ancestor_headings", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    layout_based_chunking_config: std
    ::option
    ::Option<
        terrars
        ::ListField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig>,
    >,
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
    #[doc = "Set the field `layout_based_chunking_config`.\n"]
    pub fn set_layout_based_chunking_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfig,
                        >,
                    >,
    ) -> Self {
        self.layout_based_chunking_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
    type O = terrars::BlockAssignable<DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {}

impl BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
    pub fn build(self) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig {
            layout_based_chunking_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `layout_based_chunking_config` after provisioning.\n"]
    pub fn layout_based_chunking_config(
        &self,
    ) -> terrars
    ::ListRef<DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig_LayoutBasedChunkingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layout_based_chunking_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig { }

impl terrars
::ToListMappable for DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {}

impl BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {
    pub fn build(
        self,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig {}
    }
}

#[allow(dead_code)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
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

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
    #[doc = "Set the field `enable_get_processed_document`.\n"]
    pub fn set_enable_get_processed_document(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_get_processed_document = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_image_annotation`.\n"]
    pub fn set_enable_image_annotation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_image_annotation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_llm_layout_parsing`.\n"]
    pub fn set_enable_llm_layout_parsing(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_llm_layout_parsing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_table_annotation`.\n"]
    pub fn set_enable_table_annotation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_table_annotation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_html_classes`.\n"]
    pub fn set_exclude_html_classes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_html_classes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_html_elements`.\n"]
    pub fn set_exclude_html_elements(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_html_elements = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_html_ids`.\n"]
    pub fn set_exclude_html_ids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_html_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `structured_content_types`.\n"]
    pub fn set_structured_content_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.structured_content_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {}

impl BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
    pub fn build(
        self,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig {
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
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_get_processed_document` after provisioning.\n"]
    pub fn enable_get_processed_document(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_get_processed_document", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_image_annotation` after provisioning.\n"]
    pub fn enable_image_annotation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_image_annotation", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_llm_layout_parsing` after provisioning.\n"]
    pub fn enable_llm_layout_parsing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_llm_layout_parsing", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_table_annotation` after provisioning.\n"]
    pub fn enable_table_annotation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_table_annotation", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_html_classes` after provisioning.\n"]
    pub fn exclude_html_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_html_classes", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_html_elements` after provisioning.\n"]
    pub fn exclude_html_elements(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_html_elements", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_html_ids` after provisioning.\n"]
    pub fn exclude_html_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_html_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `structured_content_types` after provisioning.\n"]
    pub fn structured_content_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.structured_content_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_native_text: std::option::Option<terrars::PrimField<bool>>,
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
    #[doc = "Set the field `use_native_text`.\n"]
    pub fn set_use_native_text(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_native_text = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
    type O =
        terrars
        ::BlockAssignable<
            DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {}

impl BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
    pub fn build(
        self,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig {
            use_native_text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `use_native_text` after provisioning.\n"]
    pub fn use_native_text(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_native_text", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digital_parsing_config: std
    ::option
    ::Option<
        terrars
        ::ListField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    layout_parsing_config: std
    ::option
    ::Option<
        terrars
        ::ListField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocr_parsing_config: std
    ::option
    ::Option<
        terrars
        ::ListField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig>,
    >,
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
    #[doc = "Set the field `digital_parsing_config`.\n"]
    pub fn set_digital_parsing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig,
                        >,
                    >,
    ) -> Self {
        self.digital_parsing_config = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig,
                        >,
                    >,
    ) -> Self {
        self.layout_parsing_config = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig,
                        >,
                    >,
    ) -> Self {
        self.ocr_parsing_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
    type O = terrars::BlockAssignable<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {}

impl BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
    pub fn build(self) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig {
            digital_parsing_config: core::default::Default::default(),
            layout_parsing_config: core::default::Default::default(),
            ocr_parsing_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `digital_parsing_config` after provisioning.\n"]
    pub fn digital_parsing_config(
        &self,
    ) -> terrars
    ::ListRef<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.digital_parsing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `layout_parsing_config` after provisioning.\n"]
    pub fn layout_parsing_config(
        &self,
    ) -> terrars
    ::ListRef<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layout_parsing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ocr_parsing_config` after provisioning.\n"]
    pub fn ocr_parsing_config(
        &self,
    ) -> terrars
    ::ListRef<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ocr_parsing_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    digital_parsing_config: std
    ::option
    ::Option<
        terrars
        ::ListField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    layout_parsing_config: std
    ::option
    ::Option<
        terrars
        ::ListField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocr_parsing_config: std
    ::option
    ::Option<
        terrars
        ::ListField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig>,
    >,
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
    #[doc = "Set the field `digital_parsing_config`.\n"]
    pub fn set_digital_parsing_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfig,
                        >,
                    >,
    ) -> Self {
        self.digital_parsing_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_type`.\n"]
    pub fn set_file_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_type = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfig,
                        >,
                    >,
    ) -> Self {
        self.layout_parsing_config = std::option::Option::Some(v.into());
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
                        ::ListField<
                            DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfig,
                        >,
                    >,
    ) -> Self {
        self.ocr_parsing_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
    type O = terrars::BlockAssignable<DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {}

impl BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
    pub fn build(self) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides {
            digital_parsing_config: core::default::Default::default(),
            file_type: core::default::Default::default(),
            layout_parsing_config: core::default::Default::default(),
            ocr_parsing_config: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesRef {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `digital_parsing_config` after provisioning.\n"]
    pub fn digital_parsing_config(
        &self,
    ) -> terrars
    ::ListRef<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_DigitalParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.digital_parsing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `file_type` after provisioning.\n"]
    pub fn file_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_type", self.base))
    }

    #[doc = "Get a reference to the value of field `layout_parsing_config` after provisioning.\n"]
    pub fn layout_parsing_config(
        &self,
    ) -> terrars
    ::ListRef<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_LayoutParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layout_parsing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `ocr_parsing_config` after provisioning.\n"]
    pub fn ocr_parsing_config(
        &self,
    ) -> terrars
    ::ListRef<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig_OcrParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ocr_parsing_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    chunking_config: std
    ::option
    ::Option<terrars::ListField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_parsing_config: std
    ::option
    ::Option<terrars::ListField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parsing_config_overrides: std
    ::option
    ::Option<terrars::SetField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides>>,
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfig {
    #[doc = "Set the field `chunking_config`.\n"]
    pub fn set_chunking_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfig>>,
    ) -> Self {
        self.chunking_config = std::option::Option::Some(v.into());
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
                        terrars::ListField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfig>,
                    >,
    ) -> Self {
        self.default_parsing_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
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
                        ::SetField<DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverrides>,
                    >,
    ) -> Self {
        self.parsing_config_overrides = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDiscoveryEngineDataStore_DocumentProcessingConfig {
    type O = terrars::BlockAssignable<DataDiscoveryEngineDataStore_DocumentProcessingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig {}

impl BuildDataDiscoveryEngineDataStore_DocumentProcessingConfig {
    pub fn build(self) -> DataDiscoveryEngineDataStore_DocumentProcessingConfig {
        DataDiscoveryEngineDataStore_DocumentProcessingConfig {
            chunking_config: core::default::Default::default(),
            default_parsing_config: core::default::Default::default(),
            name: core::default::Default::default(),
            parsing_config_overrides: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDiscoveryEngineDataStore_DocumentProcessingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDiscoveryEngineDataStore_DocumentProcessingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDiscoveryEngineDataStore_DocumentProcessingConfigRef {
        DataDiscoveryEngineDataStore_DocumentProcessingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDiscoveryEngineDataStore_DocumentProcessingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chunking_config` after provisioning.\n"]
    pub fn chunking_config(
        &self,
    ) -> terrars::ListRef<DataDiscoveryEngineDataStore_DocumentProcessingConfig_ChunkingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.chunking_config", self.base))
    }

    #[doc = "Get a reference to the value of field `default_parsing_config` after provisioning.\n"]
    pub fn default_parsing_config(
        &self,
    ) -> terrars::ListRef<DataDiscoveryEngineDataStore_DocumentProcessingConfig_DefaultParsingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_parsing_config", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parsing_config_overrides` after provisioning.\n"]
    pub fn parsing_config_overrides(
        &self,
    ) -> terrars::SetRef<DataDiscoveryEngineDataStore_DocumentProcessingConfig_ParsingConfigOverridesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.parsing_config_overrides", self.base))
    }
}
