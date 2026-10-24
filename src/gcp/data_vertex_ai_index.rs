use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataVertexAiIndexData {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
}

struct DataVertexAiIndex_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVertexAiIndexData>,
}

#[derive(Clone)]
pub struct DataVertexAiIndex(std::rc::Rc<DataVertexAiIndex_>);

impl DataVertexAiIndex {
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
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_indexes` after provisioning.\nThe pointers to DeployedIndexes created from this Index. An Index can be only deleted if all its DeployedIndexes had been undeployed first."]
    pub fn deployed_indexes(&self) -> terrars::ListRef<DataVertexAiIndexDeployedIndexesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployed_indexes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Index."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_stats` after provisioning.\nStats of the index resource."]
    pub fn index_stats(&self) -> terrars::ListRef<DataVertexAiIndexIndexStatsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_stats", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_update_method` after provisioning.\nThe update method to use with this Index. The value must be the followings. If not set, BATCH_UPDATE will be used by default.\n* BATCH_UPDATE: user can call indexes.patch with files on Cloud Storage of datapoints to update.\n* STREAM_UPDATE: user can call indexes.upsertDatapoints/DeleteDatapoints to update the Index and the updates will be applied in corresponding DeployedIndexes in nearly real-time."]
    pub fn index_update_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_update_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Indexes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nAn additional information about the Index"]
    pub fn metadata(&self) -> terrars::ListRef<DataVertexAiIndexMetadataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_schema_uri` after provisioning.\nPoints to a YAML file stored on Google Cloud Storage describing additional information about the Index, that is specific to it. Unset if the Index does not have any additional information."]
    pub fn metadata_schema_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_schema_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Index."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the index. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

impl terrars::Referable for DataVertexAiIndex {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVertexAiIndex { }

impl terrars::ToListMappable for DataVertexAiIndex {
    type O = terrars::ListRef<DataVertexAiIndexRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVertexAiIndex_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_vertex_ai_index".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVertexAiIndex {
    pub tf_id: std::string::String,
    #[doc = "The resource name of the Index."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The region of the index. eg us-central1"]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildDataVertexAiIndex {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVertexAiIndex {
        let out = DataVertexAiIndex(std::rc::Rc::new(DataVertexAiIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVertexAiIndexData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                region: self.region,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVertexAiIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVertexAiIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVertexAiIndexRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp of when the Index was created in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deployed_indexes` after provisioning.\nThe pointers to DeployedIndexes created from this Index. An Index can be only deleted if all its DeployedIndexes had been undeployed first."]
    pub fn deployed_indexes(&self) -> terrars::ListRef<DataVertexAiIndexDeployedIndexesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployed_indexes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description of the Index."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the Index. The name can be up to 128 characters long and can consist of any UTF-8 characters."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nUsed to perform consistent read-modify-write updates."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_stats` after provisioning.\nStats of the index resource."]
    pub fn index_stats(&self) -> terrars::ListRef<DataVertexAiIndexIndexStatsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_stats", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_update_method` after provisioning.\nThe update method to use with this Index. The value must be the followings. If not set, BATCH_UPDATE will be used by default.\n* BATCH_UPDATE: user can call indexes.patch with files on Cloud Storage of datapoints to update.\n* STREAM_UPDATE: user can call indexes.upsertDatapoints/DeleteDatapoints to update the Index and the updates will be applied in corresponding DeployedIndexes in nearly real-time."]
    pub fn index_update_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_update_method", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nThe labels with user-defined metadata to organize your Indexes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata` after provisioning.\nAn additional information about the Index"]
    pub fn metadata(&self) -> terrars::ListRef<DataVertexAiIndexMetadataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `metadata_schema_uri` after provisioning.\nPoints to a YAML file stored on Google Cloud Storage describing additional information about the Index, that is specific to it. Unset if the Index does not have any additional information."]
    pub fn metadata_schema_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_schema_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource name of the Index."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nThe region of the index. eg us-central1"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe timestamp of when the Index was last updated in RFC3339 UTC \"Zulu\" format, with nanosecond resolution and up to nine fractional digits."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVertexAiIndexDeployedIndexesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployed_index_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVertexAiIndexDeployedIndexesEl {
    #[doc = "Set the field `deployed_index_id`.\n"]
    pub fn set_deployed_index_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deployed_index_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index_endpoint`.\n"]
    pub fn set_index_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.index_endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVertexAiIndexDeployedIndexesEl {
    type O = terrars::BlockAssignable<DataVertexAiIndexDeployedIndexesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVertexAiIndexDeployedIndexesEl {}

impl BuildDataVertexAiIndexDeployedIndexesEl {
    pub fn build(self) -> DataVertexAiIndexDeployedIndexesEl {
        DataVertexAiIndexDeployedIndexesEl {
            deployed_index_id: core::default::Default::default(),
            index_endpoint: core::default::Default::default(),
        }
    }
}

pub struct DataVertexAiIndexDeployedIndexesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVertexAiIndexDeployedIndexesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVertexAiIndexDeployedIndexesElRef {
        DataVertexAiIndexDeployedIndexesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVertexAiIndexDeployedIndexesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployed_index_id` after provisioning.\n"]
    pub fn deployed_index_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployed_index_id", self.base))
    }

    #[doc = "Get a reference to the value of field `index_endpoint` after provisioning.\n"]
    pub fn index_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVertexAiIndexIndexStatsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shards_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vectors_count: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVertexAiIndexIndexStatsEl {
    #[doc = "Set the field `shards_count`.\n"]
    pub fn set_shards_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.shards_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vectors_count`.\n"]
    pub fn set_vectors_count(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vectors_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVertexAiIndexIndexStatsEl {
    type O = terrars::BlockAssignable<DataVertexAiIndexIndexStatsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVertexAiIndexIndexStatsEl {}

impl BuildDataVertexAiIndexIndexStatsEl {
    pub fn build(self) -> DataVertexAiIndexIndexStatsEl {
        DataVertexAiIndexIndexStatsEl {
            shards_count: core::default::Default::default(),
            vectors_count: core::default::Default::default(),
        }
    }
}

pub struct DataVertexAiIndexIndexStatsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVertexAiIndexIndexStatsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVertexAiIndexIndexStatsElRef {
        DataVertexAiIndexIndexStatsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVertexAiIndexIndexStatsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `shards_count` after provisioning.\n"]
    pub fn shards_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shards_count", self.base))
    }

    #[doc = "Get a reference to the value of field `vectors_count` after provisioning.\n"]
    pub fn vectors_count(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vectors_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {}

impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl { }

impl terrars::ToListMappable for DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {
    type O = terrars::BlockAssignable<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {}

impl BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {
    pub fn build(self) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl {}
    }
}

pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    leaf_node_embedding_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    leaf_nodes_to_search_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    #[doc = "Set the field `leaf_node_embedding_count`.\n"]
    pub fn set_leaf_node_embedding_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.leaf_node_embedding_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `leaf_nodes_to_search_percent`.\n"]
    pub fn set_leaf_nodes_to_search_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.leaf_nodes_to_search_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    type O = terrars::BlockAssignable<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {}

impl BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
    pub fn build(self) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl {
            leaf_node_embedding_count: core::default::Default::default(),
            leaf_nodes_to_search_percent: core::default::Default::default(),
        }
    }
}

pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `leaf_node_embedding_count` after provisioning.\n"]
    pub fn leaf_node_embedding_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.leaf_node_embedding_count", self.base))
    }

    #[doc = "Get a reference to the value of field `leaf_nodes_to_search_percent` after provisioning.\n"]
    pub fn leaf_nodes_to_search_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.leaf_nodes_to_search_percent", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    brute_force_config: std
    ::option
    ::Option<terrars::ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tree_ah_config: std
    ::option
    ::Option<terrars::ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl>>,
}

impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    #[doc = "Set the field `brute_force_config`.\n"]
    pub fn set_brute_force_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigEl>,
                    >,
    ) -> Self {
        self.brute_force_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tree_ah_config`.\n"]
    pub fn set_tree_ah_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigEl>>,
    ) -> Self {
        self.tree_ah_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    type O = terrars::BlockAssignable<DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {}

impl BuildDataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
    pub fn build(self) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl {
            brute_force_config: core::default::Default::default(),
            tree_ah_config: core::default::Default::default(),
        }
    }
}

pub struct DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
        DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `brute_force_config` after provisioning.\n"]
    pub fn brute_force_config(
        &self,
    ) -> terrars::ListRef<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElBruteForceConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.brute_force_config", self.base))
    }

    #[doc = "Get a reference to the value of field `tree_ah_config` after provisioning.\n"]
    pub fn tree_ah_config(
        &self,
    ) -> terrars::ListRef<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElTreeAhConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tree_ah_config", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVertexAiIndexMetadataElConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm_config: std::option::Option<terrars::ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approximate_neighbors_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distance_measure_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_norm_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shard_size: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVertexAiIndexMetadataElConfigEl {
    #[doc = "Set the field `algorithm_config`.\n"]
    pub fn set_algorithm_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVertexAiIndexMetadataElConfigElAlgorithmConfigEl>>,
    ) -> Self {
        self.algorithm_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `approximate_neighbors_count`.\n"]
    pub fn set_approximate_neighbors_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.approximate_neighbors_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dimensions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `distance_measure_type`.\n"]
    pub fn set_distance_measure_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.distance_measure_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `feature_norm_type`.\n"]
    pub fn set_feature_norm_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feature_norm_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shard_size`.\n"]
    pub fn set_shard_size(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.shard_size = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVertexAiIndexMetadataElConfigEl {
    type O = terrars::BlockAssignable<DataVertexAiIndexMetadataElConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVertexAiIndexMetadataElConfigEl {}

impl BuildDataVertexAiIndexMetadataElConfigEl {
    pub fn build(self) -> DataVertexAiIndexMetadataElConfigEl {
        DataVertexAiIndexMetadataElConfigEl {
            algorithm_config: core::default::Default::default(),
            approximate_neighbors_count: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            distance_measure_type: core::default::Default::default(),
            feature_norm_type: core::default::Default::default(),
            shard_size: core::default::Default::default(),
        }
    }
}

pub struct DataVertexAiIndexMetadataElConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVertexAiIndexMetadataElConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVertexAiIndexMetadataElConfigElRef {
        DataVertexAiIndexMetadataElConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVertexAiIndexMetadataElConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm_config` after provisioning.\n"]
    pub fn algorithm_config(&self) -> terrars::ListRef<DataVertexAiIndexMetadataElConfigElAlgorithmConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.algorithm_config", self.base))
    }

    #[doc = "Get a reference to the value of field `approximate_neighbors_count` after provisioning.\n"]
    pub fn approximate_neighbors_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approximate_neighbors_count", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `distance_measure_type` after provisioning.\n"]
    pub fn distance_measure_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distance_measure_type", self.base))
    }

    #[doc = "Get a reference to the value of field `feature_norm_type` after provisioning.\n"]
    pub fn feature_norm_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_norm_type", self.base))
    }

    #[doc = "Get a reference to the value of field `shard_size` after provisioning.\n"]
    pub fn shard_size(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.shard_size", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVertexAiIndexMetadataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config: std::option::Option<terrars::ListField<DataVertexAiIndexMetadataElConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contents_delta_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_complete_overwrite: std::option::Option<terrars::PrimField<bool>>,
}

impl DataVertexAiIndexMetadataEl {
    #[doc = "Set the field `config`.\n"]
    pub fn set_config(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVertexAiIndexMetadataElConfigEl>>,
    ) -> Self {
        self.config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `contents_delta_uri`.\n"]
    pub fn set_contents_delta_uri(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.contents_delta_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_complete_overwrite`.\n"]
    pub fn set_is_complete_overwrite(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_complete_overwrite = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVertexAiIndexMetadataEl {
    type O = terrars::BlockAssignable<DataVertexAiIndexMetadataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVertexAiIndexMetadataEl {}

impl BuildDataVertexAiIndexMetadataEl {
    pub fn build(self) -> DataVertexAiIndexMetadataEl {
        DataVertexAiIndexMetadataEl {
            config: core::default::Default::default(),
            contents_delta_uri: core::default::Default::default(),
            is_complete_overwrite: core::default::Default::default(),
        }
    }
}

pub struct DataVertexAiIndexMetadataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVertexAiIndexMetadataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVertexAiIndexMetadataElRef {
        DataVertexAiIndexMetadataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVertexAiIndexMetadataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `config` after provisioning.\n"]
    pub fn config(&self) -> terrars::ListRef<DataVertexAiIndexMetadataElConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config", self.base))
    }

    #[doc = "Get a reference to the value of field `contents_delta_uri` after provisioning.\n"]
    pub fn contents_delta_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contents_delta_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `is_complete_overwrite` after provisioning.\n"]
    pub fn is_complete_overwrite(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_complete_overwrite", self.base))
    }
}
