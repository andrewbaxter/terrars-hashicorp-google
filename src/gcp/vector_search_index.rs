#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VectorSearchIndexData_ {
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
    collection_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distance_metric: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    index_field: terrars::PrimField<std::string::String>,
    index_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_wait: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    store_fields: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dedicated_infrastructure: std::option::Option<std::vec::Vec<VectorSearchIndex_DedicatedInfrastructure>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dense_scann: std::option::Option<std::vec::Vec<VectorSearchIndex_DenseScann>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VectorSearchIndex_Timeouts>,
    dynamic: VectorSearchIndexDynamic,
}

struct VectorSearchIndex_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VectorSearchIndexData_>,
}

#[derive(Clone)]
pub struct VectorSearchIndex(std::rc::Rc<VectorSearchIndex_>);

impl VectorSearchIndex {
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

    #[doc = "Set the field `description`.\nUser-specified description of the index"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser-specified display name of the index"]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `distance_metric`.\nDistance metric used for indexing. If not specified, will default to\n'DOT_PRODUCT'. Possible values: [\"DOT_PRODUCT\", \"COSINE_DISTANCE\"]"]
    pub fn set_distance_metric(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().distance_metric = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_fields`.\nThe fields to push into the index to enable fast ANN inline filtering."]
    pub fn set_filter_fields(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().filter_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `skip_wait`.\nWhether to skip waiting for the index to be created. Building an ANN\nindex over a large collection can take several hours; setting 'skip_wait'\nto 'true' causes Terraform to return as soon as the creation request is\naccepted rather than blocking until the long-running build completes\n(which can exceed CI/CD timeouts or short-lived Terraform runner\ncredentials). Leave this set to 'false' (the default) if downstream\nresources or workloads depend on the index being ready to serve queries\nimmediately after 'terraform apply', or if you want Terraform to fail\nsynchronously when an asynchronous index build fails."]
    pub fn set_skip_wait(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_wait = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `store_fields`.\nThe fields to push into the index to enable inline data retrieval."]
    pub fn set_store_fields(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().store_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dedicated_infrastructure`.\n"]
    pub fn set_dedicated_infrastructure(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VectorSearchIndex_DedicatedInfrastructure>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dedicated_infrastructure = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dedicated_infrastructure = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dense_scann`.\n"]
    pub fn set_dense_scann(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VectorSearchIndex_DenseScann>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dense_scann = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dense_scann = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VectorSearchIndex_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nThe ID of the parent Collection."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create time stamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-specified description of the index"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-specified display name of the index"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `distance_metric` after provisioning.\nDistance metric used for indexing. If not specified, will default to\n'DOT_PRODUCT'. Possible values: [\"DOT_PRODUCT\", \"COSINE_DISTANCE\"]"]
    pub fn distance_metric(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distance_metric", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter_fields` after provisioning.\nThe fields to push into the index to enable fast ANN inline filtering."]
    pub fn filter_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_field` after provisioning.\nThe collection schema field to index."]
    pub fn index_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_field", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_id` after provisioning.\nID of the Index to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub fn index_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_wait` after provisioning.\nWhether to skip waiting for the index to be created. Building an ANN\nindex over a large collection can take several hours; setting 'skip_wait'\nto 'true' causes Terraform to return as soon as the creation request is\naccepted rather than blocking until the long-running build completes\n(which can exceed CI/CD timeouts or short-lived Terraform runner\ncredentials). Leave this set to 'false' (the default) if downstream\nresources or workloads depend on the index being ready to serve queries\nimmediately after 'terraform apply', or if you want Terraform to fail\nsynchronously when an asynchronous index build fails."]
    pub fn skip_wait(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_wait", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `store_fields` after provisioning.\nThe fields to push into the index to enable inline data retrieval."]
    pub fn store_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.store_fields", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update time stamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_infrastructure` after provisioning.\n"]
    pub fn dedicated_infrastructure(&self) -> terrars::ListRef<VectorSearchIndex_DedicatedInfrastructureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_infrastructure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dense_scann` after provisioning.\n"]
    pub fn dense_scann(&self) -> terrars::ListRef<VectorSearchIndex_DenseScannRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dense_scann", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VectorSearchIndex_TimeoutsRef {
        VectorSearchIndex_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VectorSearchIndex {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VectorSearchIndex { }

impl terrars::ToListMappable for VectorSearchIndex {
    type O = terrars::ListRef<VectorSearchIndexRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VectorSearchIndex_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vector_search_index".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVectorSearchIndex {
    pub tf_id: std::string::String,
    #[doc = "The ID of the parent Collection."]
    pub collection_id: terrars::PrimField<std::string::String>,
    #[doc = "The collection schema field to index."]
    pub index_field: terrars::PrimField<std::string::String>,
    #[doc =
        "ID of the Index to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub index_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildVectorSearchIndex {
    pub fn build(self, stack: &mut terrars::Stack) -> VectorSearchIndex {
        let out = VectorSearchIndex(std::rc::Rc::new(VectorSearchIndex_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VectorSearchIndexData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                collection_id: self.collection_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                distance_metric: core::default::Default::default(),
                filter_fields: core::default::Default::default(),
                id: core::default::Default::default(),
                index_field: self.index_field,
                index_id: self.index_id,
                labels: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                skip_wait: core::default::Default::default(),
                store_fields: core::default::Default::default(),
                dedicated_infrastructure: core::default::Default::default(),
                dense_scann: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VectorSearchIndexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchIndexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VectorSearchIndexRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `collection_id` after provisioning.\nThe ID of the parent Collection."]
    pub fn collection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n[Output only] Create time stamp"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-specified description of the index"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-specified display name of the index"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `distance_metric` after provisioning.\nDistance metric used for indexing. If not specified, will default to\n'DOT_PRODUCT'. Possible values: [\"DOT_PRODUCT\", \"COSINE_DISTANCE\"]"]
    pub fn distance_metric(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distance_metric", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `filter_fields` after provisioning.\nThe fields to push into the index to enable fast ANN inline filtering."]
    pub fn filter_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_fields", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_field` after provisioning.\nThe collection schema field to index."]
    pub fn index_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_field", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `index_id` after provisioning.\nID of the Index to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub fn index_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels as key value pairs.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `skip_wait` after provisioning.\nWhether to skip waiting for the index to be created. Building an ANN\nindex over a large collection can take several hours; setting 'skip_wait'\nto 'true' causes Terraform to return as soon as the creation request is\naccepted rather than blocking until the long-running build completes\n(which can exceed CI/CD timeouts or short-lived Terraform runner\ncredentials). Leave this set to 'false' (the default) if downstream\nresources or workloads depend on the index being ready to serve queries\nimmediately after 'terraform apply', or if you want Terraform to fail\nsynchronously when an asynchronous index build fails."]
    pub fn skip_wait(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_wait", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `store_fields` after provisioning.\nThe fields to push into the index to enable inline data retrieval."]
    pub fn store_fields(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.store_fields", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update time stamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_infrastructure` after provisioning.\n"]
    pub fn dedicated_infrastructure(&self) -> terrars::ListRef<VectorSearchIndex_DedicatedInfrastructureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_infrastructure", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dense_scann` after provisioning.\n"]
    pub fn dense_scann(&self) -> terrars::ListRef<VectorSearchIndex_DenseScannRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dense_scann", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VectorSearchIndex_TimeoutsRef {
        VectorSearchIndex_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VectorSearchIndexDynamic {
    dedicated_infrastructure: std::option::Option<terrars::DynamicBlock<VectorSearchIndex_DedicatedInfrastructure>>,
    dense_scann: std::option::Option<terrars::DynamicBlock<VectorSearchIndex_DenseScann>>,
}

#[derive(Serialize)]
pub struct VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_replica_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_replica_count: std::option::Option<terrars::PrimField<f64>>,
}

impl VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpec {
    #[doc =
        "Set the field `max_replica_count`.\nThe maximum number of replicas. Must be >= 'min_replica_count'\nand <= '1000'. If not set or set to '0', defaults to the greater\nof 'min_replica_count' and '2' (or '5' for the v1beta version)."]
    pub fn set_max_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_replica_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_replica_count`.\nThe minimum number of replicas. If not set or set to '0', defaults\nto '2'. Must be >= '1' and <= '1000'."]
    pub fn set_min_replica_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_replica_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpec {
    type O = terrars::BlockAssignable<VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchIndex_DedicatedInfrastructure_AutoscalingSpec {}

impl BuildVectorSearchIndex_DedicatedInfrastructure_AutoscalingSpec {
    pub fn build(self) -> VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpec {
        VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpec {
            max_replica_count: core::default::Default::default(),
            min_replica_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpecRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpecRef {
        VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_replica_count` after provisioning.\nThe maximum number of replicas. Must be >= 'min_replica_count'\nand <= '1000'. If not set or set to '0', defaults to the greater\nof 'min_replica_count' and '2' (or '5' for the v1beta version)."]
    pub fn max_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_replica_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_replica_count` after provisioning.\nThe minimum number of replicas. If not set or set to '0', defaults\nto '2'. Must be >= '1' and <= '1000'."]
    pub fn min_replica_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_replica_count", self.base))
    }
}

#[derive(Serialize, Default)]
struct VectorSearchIndex_DedicatedInfrastructureDynamic {
    autoscaling_spec: std
    ::option
    ::Option<terrars::DynamicBlock<VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpec>>,
}

#[derive(Serialize)]
pub struct VectorSearchIndex_DedicatedInfrastructure {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_spec: std::option::Option<std::vec::Vec<VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpec>>,
    dynamic: VectorSearchIndex_DedicatedInfrastructureDynamic,
}

impl VectorSearchIndex_DedicatedInfrastructure {
    #[doc =
        "Set the field `mode`.\nMode of the dedicated infrastructure. Defaults to 'PERFORMANCE_OPTIMIZED'. Possible values: [\"MODE_UNSPECIFIED\", \"STORAGE_OPTIMIZED\", \"PERFORMANCE_OPTIMIZED\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_spec`.\n"]
    pub fn set_autoscaling_spec(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.autoscaling_spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.autoscaling_spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VectorSearchIndex_DedicatedInfrastructure {
    type O = terrars::BlockAssignable<VectorSearchIndex_DedicatedInfrastructure>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchIndex_DedicatedInfrastructure {}

impl BuildVectorSearchIndex_DedicatedInfrastructure {
    pub fn build(self) -> VectorSearchIndex_DedicatedInfrastructure {
        VectorSearchIndex_DedicatedInfrastructure {
            mode: core::default::Default::default(),
            autoscaling_spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VectorSearchIndex_DedicatedInfrastructureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchIndex_DedicatedInfrastructureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorSearchIndex_DedicatedInfrastructureRef {
        VectorSearchIndex_DedicatedInfrastructureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchIndex_DedicatedInfrastructureRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nMode of the dedicated infrastructure. Defaults to 'PERFORMANCE_OPTIMIZED'. Possible values: [\"MODE_UNSPECIFIED\", \"STORAGE_OPTIMIZED\", \"PERFORMANCE_OPTIMIZED\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `autoscaling_spec` after provisioning.\n"]
    pub fn autoscaling_spec(&self) -> terrars::ListRef<VectorSearchIndex_DedicatedInfrastructure_AutoscalingSpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.autoscaling_spec", self.base))
    }
}

#[derive(Serialize)]
pub struct VectorSearchIndex_DenseScann {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    feature_norm_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VectorSearchIndex_DenseScann {
    #[doc =
        "Set the field `feature_norm_type`.\nFeature norm type for the ScaNN index. Possible values: [\"FEATURE_NORM_TYPE_UNSPECIFIED\", \"NONE\", \"UNIT_L2_NORM\"]"]
    pub fn set_feature_norm_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.feature_norm_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VectorSearchIndex_DenseScann {
    type O = terrars::BlockAssignable<VectorSearchIndex_DenseScann>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchIndex_DenseScann {}

impl BuildVectorSearchIndex_DenseScann {
    pub fn build(self) -> VectorSearchIndex_DenseScann {
        VectorSearchIndex_DenseScann { feature_norm_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct VectorSearchIndex_DenseScannRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchIndex_DenseScannRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorSearchIndex_DenseScannRef {
        VectorSearchIndex_DenseScannRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchIndex_DenseScannRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `feature_norm_type` after provisioning.\nFeature norm type for the ScaNN index. Possible values: [\"FEATURE_NORM_TYPE_UNSPECIFIED\", \"NONE\", \"UNIT_L2_NORM\"]"]
    pub fn feature_norm_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature_norm_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VectorSearchIndex_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VectorSearchIndex_Timeouts {
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

impl terrars::ToListMappable for VectorSearchIndex_Timeouts {
    type O = terrars::BlockAssignable<VectorSearchIndex_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchIndex_Timeouts {}

impl BuildVectorSearchIndex_Timeouts {
    pub fn build(self) -> VectorSearchIndex_Timeouts {
        VectorSearchIndex_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VectorSearchIndex_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchIndex_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorSearchIndex_TimeoutsRef {
        VectorSearchIndex_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchIndex_TimeoutsRef {
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
