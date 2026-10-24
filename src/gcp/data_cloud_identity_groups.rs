use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCloudIdentityGroupsData {
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
    parent: terrars::PrimField<std::string::String>,
}

struct DataCloudIdentityGroups_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudIdentityGroupsData>,
}

#[derive(Clone)]
pub struct DataCloudIdentityGroups(std::rc::Rc<DataCloudIdentityGroups_>);

impl DataCloudIdentityGroups {
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

    #[doc = "Get a reference to the value of field `groups` after provisioning.\nList of Cloud Identity groups."]
    pub fn groups(&self) -> terrars::ListRef<DataCloudIdentityGroupsGroupsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the entity under which this Group resides in the\nCloud Identity resource hierarchy.\n\nMust be of the form identitysources/{identity_source_id} for external-identity-mapped\ngroups or customers/{customer_id} for Google Groups."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudIdentityGroups {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudIdentityGroups { }

impl terrars::ToListMappable for DataCloudIdentityGroups {
    type O = terrars::ListRef<DataCloudIdentityGroupsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudIdentityGroups_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "google_cloud_identity_groups".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudIdentityGroups {
    pub tf_id: std::string::String,
    #[doc =
        "The resource name of the entity under which this Group resides in the\nCloud Identity resource hierarchy.\n\nMust be of the form identitysources/{identity_source_id} for external-identity-mapped\ngroups or customers/{customer_id} for Google Groups."]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudIdentityGroups {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudIdentityGroups {
        let out = DataCloudIdentityGroups(std::rc::Rc::new(DataCloudIdentityGroups_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCloudIdentityGroupsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                parent: self.parent,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudIdentityGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudIdentityGroupsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\nList of Cloud Identity groups."]
    pub fn groups(&self) -> terrars::ListRef<DataCloudIdentityGroupsGroupsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe resource name of the entity under which this Group resides in the\nCloud Identity resource hierarchy.\n\nMust be of the form identitysources/{identity_source_id} for external-identity-mapped\ngroups or customers/{customer_id} for Google Groups."]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {}

impl BuildDataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
    pub fn build(self) -> DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
        DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl {
            id: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}

pub struct DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef {
        DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupsGroupsElGroupKeyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupsGroupsElGroupKeyEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityGroupsGroupsElGroupKeyEl {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupsGroupsElGroupKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupsGroupsElGroupKeyEl {}

impl BuildDataCloudIdentityGroupsGroupsElGroupKeyEl {
    pub fn build(self) -> DataCloudIdentityGroupsGroupsElGroupKeyEl {
        DataCloudIdentityGroupsGroupsElGroupKeyEl {
            id: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}

pub struct DataCloudIdentityGroupsGroupsElGroupKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupsGroupsElGroupKeyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudIdentityGroupsGroupsElGroupKeyElRef {
        DataCloudIdentityGroupsGroupsElGroupKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupsGroupsElGroupKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudIdentityGroupsGroupsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_group_keys: std::option::Option<terrars::ListField<DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_key: std::option::Option<terrars::ListField<DataCloudIdentityGroupsGroupsElGroupKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_group_config: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parent: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudIdentityGroupsGroupsEl {
    #[doc = "Set the field `additional_group_keys`.\n"]
    pub fn set_additional_group_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudIdentityGroupsGroupsElAdditionalGroupKeysEl>>,
    ) -> Self {
        self.additional_group_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_key`.\n"]
    pub fn set_group_key(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataCloudIdentityGroupsGroupsElGroupKeyEl>>,
    ) -> Self {
        self.group_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_group_config`.\n"]
    pub fn set_initial_group_config(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.initial_group_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parent`.\n"]
    pub fn set_parent(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update_time`.\n"]
    pub fn set_update_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudIdentityGroupsGroupsEl {
    type O = terrars::BlockAssignable<DataCloudIdentityGroupsGroupsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudIdentityGroupsGroupsEl {}

impl BuildDataCloudIdentityGroupsGroupsEl {
    pub fn build(self) -> DataCloudIdentityGroupsGroupsEl {
        DataCloudIdentityGroupsGroupsEl {
            additional_group_keys: core::default::Default::default(),
            create_time: core::default::Default::default(),
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            group_key: core::default::Default::default(),
            initial_group_config: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            parent: core::default::Default::default(),
            update_time: core::default::Default::default(),
        }
    }
}

pub struct DataCloudIdentityGroupsGroupsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudIdentityGroupsGroupsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCloudIdentityGroupsGroupsElRef {
        DataCloudIdentityGroupsGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudIdentityGroupsGroupsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_group_keys` after provisioning.\n"]
    pub fn additional_group_keys(&self) -> terrars::ListRef<DataCloudIdentityGroupsGroupsElAdditionalGroupKeysElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_group_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `group_key` after provisioning.\n"]
    pub fn group_key(&self) -> terrars::ListRef<DataCloudIdentityGroupsGroupsElGroupKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_key", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_group_config` after provisioning.\n"]
    pub fn initial_group_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_group_config", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `parent` after provisioning.\n"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.base))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.base))
    }
}
