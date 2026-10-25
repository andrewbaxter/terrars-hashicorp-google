#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct VectorSearchDataObjectData_ {
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
    data: std::option::Option<terrars::PrimField<std::string::String>>,
    data_object_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    etag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VectorSearchDataObject_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vectors: std::option::Option<std::vec::Vec<VectorSearchDataObject_Vectors>>,
    dynamic: VectorSearchDataObjectDynamic,
}

struct VectorSearchDataObject_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VectorSearchDataObjectData_>,
}

#[derive(Clone)]
pub struct VectorSearchDataObject(std::rc::Rc<VectorSearchDataObject_>);

impl VectorSearchDataObject {
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
        "Set the field `data`.\nThe JSON data of the DataObject. Must be a JSON object whose field\nnames match the fields defined in the parent Collection's\n'data_schema'."]
    pub fn set_data(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `etag`.\nThe etag of the DataObject, used for optimistic concurrency\ncontrol on updates and deletes."]
    pub fn set_etag(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().etag = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VectorSearchDataObject_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vectors`.\n"]
    pub fn set_vectors(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VectorSearchDataObject_Vectors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vectors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vectors = std::option::Option::Some(d);
            },
        }
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
        "Get a reference to the value of field `data` after provisioning.\nThe JSON data of the DataObject. Must be a JSON object whose field\nnames match the fields defined in the parent Collection's\n'data_schema'."]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_object_id` after provisioning.\nID of the DataObject to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub fn data_object_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_object_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag of the DataObject, used for optimistic concurrency\ncontrol on updates and deletes."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update time stamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VectorSearchDataObject_TimeoutsRef {
        VectorSearchDataObject_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VectorSearchDataObject {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VectorSearchDataObject { }

impl terrars::ToListMappable for VectorSearchDataObject {
    type O = terrars::ListRef<VectorSearchDataObjectRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VectorSearchDataObject_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_vector_search_data_object".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVectorSearchDataObject {
    pub tf_id: std::string::String,
    #[doc = "The ID of the parent Collection."]
    pub collection_id: terrars::PrimField<std::string::String>,
    #[doc =
        "ID of the DataObject to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub data_object_id: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildVectorSearchDataObject {
    pub fn build(self, stack: &mut terrars::Stack) -> VectorSearchDataObject {
        let out = VectorSearchDataObject(std::rc::Rc::new(VectorSearchDataObject_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VectorSearchDataObjectData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                collection_id: self.collection_id,
                data: core::default::Default::default(),
                data_object_id: self.data_object_id,
                deletion_policy: core::default::Default::default(),
                etag: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vectors: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VectorSearchDataObjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchDataObjectRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VectorSearchDataObjectRef {
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
        "Get a reference to the value of field `data` after provisioning.\nThe JSON data of the DataObject. Must be a JSON object whose field\nnames match the fields defined in the parent Collection's\n'data_schema'."]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_object_id` after provisioning.\nID of the DataObject to create.\nThe id must be 1-63 characters long, and comply with\n[RFC1035](https://www.ietf.org/rfc/rfc1035.txt).\nSpecifically, it must be 1-63 characters long and match the regular\nexpression '[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?'."]
    pub fn data_object_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_object_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe etag of the DataObject, used for optimistic concurrency\ncontrol on updates and deletes."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\nIdentifier. name of resource"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n[Output only] Update time stamp"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VectorSearchDataObject_TimeoutsRef {
        VectorSearchDataObject_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VectorSearchDataObjectDynamic {
    vectors: std::option::Option<terrars::DynamicBlock<VectorSearchDataObject_Vectors>>,
}

#[derive(Serialize)]
pub struct VectorSearchDataObject_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VectorSearchDataObject_Timeouts {
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

impl terrars::ToListMappable for VectorSearchDataObject_Timeouts {
    type O = terrars::BlockAssignable<VectorSearchDataObject_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchDataObject_Timeouts {}

impl BuildVectorSearchDataObject_Timeouts {
    pub fn build(self) -> VectorSearchDataObject_Timeouts {
        VectorSearchDataObject_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VectorSearchDataObject_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchDataObject_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorSearchDataObject_TimeoutsRef {
        VectorSearchDataObject_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchDataObject_TimeoutsRef {
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

#[derive(Serialize)]
pub struct VectorSearchDataObject_Vectors_Dense {
    values: terrars::ListField<terrars::PrimField<f64>>,
}

impl VectorSearchDataObject_Vectors_Dense { }

impl terrars::ToListMappable for VectorSearchDataObject_Vectors_Dense {
    type O = terrars::BlockAssignable<VectorSearchDataObject_Vectors_Dense>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchDataObject_Vectors_Dense {
    #[doc = "The float values of the dense vector."]
    pub values: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildVectorSearchDataObject_Vectors_Dense {
    pub fn build(self) -> VectorSearchDataObject_Vectors_Dense {
        VectorSearchDataObject_Vectors_Dense { values: self.values }
    }
}

#[allow(dead_code)]
pub struct VectorSearchDataObject_Vectors_DenseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchDataObject_Vectors_DenseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorSearchDataObject_Vectors_DenseRef {
        VectorSearchDataObject_Vectors_DenseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchDataObject_Vectors_DenseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\nThe float values of the dense vector."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct VectorSearchDataObject_Vectors_Sparse {
    indices: terrars::ListField<terrars::PrimField<f64>>,
    values: terrars::ListField<terrars::PrimField<f64>>,
}

impl VectorSearchDataObject_Vectors_Sparse { }

impl terrars::ToListMappable for VectorSearchDataObject_Vectors_Sparse {
    type O = terrars::BlockAssignable<VectorSearchDataObject_Vectors_Sparse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchDataObject_Vectors_Sparse {
    #[doc = "The indices corresponding to the entries in 'values'. Must\nhave the same length as 'values'."]
    pub indices: terrars::ListField<terrars::PrimField<f64>>,
    #[doc = "The non-zero float values of the sparse vector."]
    pub values: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildVectorSearchDataObject_Vectors_Sparse {
    pub fn build(self) -> VectorSearchDataObject_Vectors_Sparse {
        VectorSearchDataObject_Vectors_Sparse {
            indices: self.indices,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct VectorSearchDataObject_Vectors_SparseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchDataObject_Vectors_SparseRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorSearchDataObject_Vectors_SparseRef {
        VectorSearchDataObject_Vectors_SparseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchDataObject_Vectors_SparseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `indices` after provisioning.\nThe indices corresponding to the entries in 'values'. Must\nhave the same length as 'values'."]
    pub fn indices(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.indices", self.base))
    }

    #[doc =
        "Get a reference to the value of field `values` after provisioning.\nThe non-zero float values of the sparse vector."]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct VectorSearchDataObject_VectorsDynamic {
    dense: std::option::Option<terrars::DynamicBlock<VectorSearchDataObject_Vectors_Dense>>,
    sparse: std::option::Option<terrars::DynamicBlock<VectorSearchDataObject_Vectors_Sparse>>,
}

#[derive(Serialize)]
pub struct VectorSearchDataObject_Vectors {
    field_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dense: std::option::Option<std::vec::Vec<VectorSearchDataObject_Vectors_Dense>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sparse: std::option::Option<std::vec::Vec<VectorSearchDataObject_Vectors_Sparse>>,
    dynamic: VectorSearchDataObject_VectorsDynamic,
}

impl VectorSearchDataObject_Vectors {
    #[doc = "Set the field `dense`.\n"]
    pub fn set_dense(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VectorSearchDataObject_Vectors_Dense>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dense = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dense = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sparse`.\n"]
    pub fn set_sparse(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VectorSearchDataObject_Vectors_Sparse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sparse = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sparse = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VectorSearchDataObject_Vectors {
    type O = terrars::BlockAssignable<VectorSearchDataObject_Vectors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVectorSearchDataObject_Vectors {
    #[doc = ""]
    pub field_name: terrars::PrimField<std::string::String>,
}

impl BuildVectorSearchDataObject_Vectors {
    pub fn build(self) -> VectorSearchDataObject_Vectors {
        VectorSearchDataObject_Vectors {
            field_name: self.field_name,
            dense: core::default::Default::default(),
            sparse: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VectorSearchDataObject_VectorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VectorSearchDataObject_VectorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VectorSearchDataObject_VectorsRef {
        VectorSearchDataObject_VectorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VectorSearchDataObject_VectorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field_name` after provisioning.\n"]
    pub fn field_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_name", self.base))
    }

    #[doc = "Get a reference to the value of field `dense` after provisioning.\n"]
    pub fn dense(&self) -> terrars::ListRef<VectorSearchDataObject_Vectors_DenseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dense", self.base))
    }

    #[doc = "Get a reference to the value of field `sparse` after provisioning.\n"]
    pub fn sparse(&self) -> terrars::ListRef<VectorSearchDataObject_Vectors_SparseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sparse", self.base))
    }
}
