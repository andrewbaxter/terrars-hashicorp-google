use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataLossPreventionStoredInfoTypeData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_info_type_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoTypeDictionaryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    large_custom_dictionary: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoTypeRegexEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataLossPreventionStoredInfoTypeTimeoutsEl>,
    dynamic: DataLossPreventionStoredInfoTypeDynamic,
}

struct DataLossPreventionStoredInfoType_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLossPreventionStoredInfoTypeData>,
}

#[derive(Clone)]
pub struct DataLossPreventionStoredInfoType(std::rc::Rc<DataLossPreventionStoredInfoType_>);

impl DataLossPreventionStoredInfoType {
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

    #[doc = "Set the field `description`.\nA description of the info type."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser set display name of the info type."]
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
        "Set the field `stored_info_type_id`.\nThe storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is 100\ncharacters. Can be empty to allow the system to generate one."]
    pub fn set_stored_info_type_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().stored_info_type_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dictionary`.\n"]
    pub fn set_dictionary(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dictionary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dictionary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `large_custom_dictionary`.\n"]
    pub fn set_large_custom_dictionary(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().large_custom_dictionary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.large_custom_dictionary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionStoredInfoTypeRegexEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataLossPreventionStoredInfoTypeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the info type."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser set display name of the info type."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the info type. Set by the server."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the info type in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stored_info_type_id` after provisioning.\nThe storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is 100\ncharacters. Can be empty to allow the system to generate one."]
    pub fn stored_info_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stored_info_type_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]
    pub fn dictionary(&self) -> terrars::ListRef<DataLossPreventionStoredInfoTypeDictionaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `large_custom_dictionary` after provisioning.\n"]
    pub fn large_custom_dictionary(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.large_custom_dictionary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::ListRef<DataLossPreventionStoredInfoTypeRegexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionStoredInfoTypeTimeoutsElRef {
        DataLossPreventionStoredInfoTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataLossPreventionStoredInfoType {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataLossPreventionStoredInfoType { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoType {
    type O = terrars::ListRef<DataLossPreventionStoredInfoTypeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataLossPreventionStoredInfoType_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_data_loss_prevention_stored_info_type".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLossPreventionStoredInfoType {
    pub tf_id: std::string::String,
    #[doc =
        "The parent of the info type in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoType {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLossPreventionStoredInfoType {
        let out = DataLossPreventionStoredInfoType(std::rc::Rc::new(DataLossPreventionStoredInfoType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLossPreventionStoredInfoTypeData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: self.parent,
                stored_info_type_id: core::default::Default::default(),
                dictionary: core::default::Default::default(),
                large_custom_dictionary: core::default::Default::default(),
                regex: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataLossPreventionStoredInfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLossPreventionStoredInfoTypeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the info type."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser set display name of the info type."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the info type. Set by the server."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the info type in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `stored_info_type_id` after provisioning.\nThe storedInfoType ID can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is 100\ncharacters. Can be empty to allow the system to generate one."]
    pub fn stored_info_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stored_info_type_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]
    pub fn dictionary(&self) -> terrars::ListRef<DataLossPreventionStoredInfoTypeDictionaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `large_custom_dictionary` after provisioning.\n"]
    pub fn large_custom_dictionary(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.large_custom_dictionary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::ListRef<DataLossPreventionStoredInfoTypeRegexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionStoredInfoTypeTimeoutsElRef {
        DataLossPreventionStoredInfoTypeTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl {
    #[doc =
        "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl {
        DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl { path: self.path }
    }
}

pub struct DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef {
        DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nA url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeDictionaryElWordListEl {
    words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionStoredInfoTypeDictionaryElWordListEl { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoTypeDictionaryElWordListEl {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryElWordListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoTypeDictionaryElWordListEl {
    #[doc =
        "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLossPreventionStoredInfoTypeDictionaryElWordListEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeDictionaryElWordListEl {
        DataLossPreventionStoredInfoTypeDictionaryElWordListEl { words: self.words }
    }
}

pub struct DataLossPreventionStoredInfoTypeDictionaryElWordListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeDictionaryElWordListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoTypeDictionaryElWordListElRef {
        DataLossPreventionStoredInfoTypeDictionaryElWordListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoTypeDictionaryElWordListElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `words` after provisioning.\nWords or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub fn words(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.words", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionStoredInfoTypeDictionaryElDynamic {
    cloud_storage_path: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl>>,
    word_list: std::option::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoTypeDictionaryElWordListEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeDictionaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_path: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    word_list: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoTypeDictionaryElWordListEl>>,
    dynamic: DataLossPreventionStoredInfoTypeDictionaryElDynamic,
}

impl DataLossPreventionStoredInfoTypeDictionaryEl {
    #[doc = "Set the field `cloud_storage_path`.\n"]
    pub fn set_cloud_storage_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `word_list`.\n"]
    pub fn set_word_list(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryElWordListEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.word_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.word_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionStoredInfoTypeDictionaryEl {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoTypeDictionaryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoTypeDictionaryEl {}

impl BuildDataLossPreventionStoredInfoTypeDictionaryEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeDictionaryEl {
        DataLossPreventionStoredInfoTypeDictionaryEl {
            cloud_storage_path: core::default::Default::default(),
            word_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionStoredInfoTypeDictionaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeDictionaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoTypeDictionaryElRef {
        DataLossPreventionStoredInfoTypeDictionaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoTypeDictionaryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]
    pub fn cloud_storage_path(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoTypeDictionaryElCloudStoragePathElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_path", self.base))
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(&self) -> terrars::ListRef<DataLossPreventionStoredInfoTypeDictionaryElWordListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl {
    #[doc = "Name describing the field."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl { name: self.name }
    }
}

pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName describing the field."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    table_id: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {
    #[doc = "The dataset ID of the table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The Google Cloud Platform project ID of the project containing the table."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the table."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}

pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dataset_id` after provisioning.\nThe dataset ID of the table."]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nThe Google Cloud Platform project ID of the project containing the table."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc = "Get a reference to the value of field `table_id` after provisioning.\nThe name of the table."]
    pub fn table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElDynamic {
    field: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl>>,
    table: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl>>,
    dynamic: DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElDynamic,
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `table`.\n"]
    pub fn set_table(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {}

impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl {
            field: core::default::Default::default(),
            table: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElFieldElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl {
    url: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl {
    #[doc = "The url, in the format 'gs://<bucket>/<path>'. Trailing wildcard in the path is allowed."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl { url: self.url }
    }
}

pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nThe url, in the format 'gs://<bucket>/<path>'. Trailing wildcard in the path is allowed."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl {
    #[doc =
        "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl { path: self.path }
    }
}

pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nA url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElDynamic {
    big_query_field: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl>>,
    cloud_storage_file_set: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl>>,
    output_path: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    big_query_field: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_file_set: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_path: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl>>,
    dynamic: DataLossPreventionStoredInfoTypeLargeCustomDictionaryElDynamic,
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
    #[doc = "Set the field `big_query_field`.\n"]
    pub fn set_big_query_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.big_query_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.big_query_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloud_storage_file_set`.\n"]
    pub fn set_cloud_storage_file_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_storage_file_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_storage_file_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_path`.\n"]
    pub fn set_output_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_path = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {}

impl BuildDataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl {
            big_query_field: core::default::Default::default(),
            cloud_storage_file_set: core::default::Default::default(),
            output_path: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef {
        DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoTypeLargeCustomDictionaryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `big_query_field` after provisioning.\n"]
    pub fn big_query_field(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElBigQueryFieldElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.big_query_field", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_storage_file_set` after provisioning.\n"]
    pub fn cloud_storage_file_set(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElCloudStorageFileSetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_file_set", self.base))
    }

    #[doc = "Get a reference to the value of field `output_path` after provisioning.\n"]
    pub fn output_path(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoTypeLargeCustomDictionaryElOutputPathElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeRegexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoTypeRegexEl {
    #[doc =
        "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
    pub fn set_group_indexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.group_indexes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionStoredInfoTypeRegexEl {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoTypeRegexEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoTypeRegexEl {
    #[doc =
        "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoTypeRegexEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeRegexEl {
        DataLossPreventionStoredInfoTypeRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

pub struct DataLossPreventionStoredInfoTypeRegexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeRegexElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionStoredInfoTypeRegexElRef {
        DataLossPreventionStoredInfoTypeRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoTypeRegexElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `group_indexes` after provisioning.\nThe index of the submatch to extract as findings. When not specified, the entire match is returned. No more than 3 may be included."]
    pub fn group_indexes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_indexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nPattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoTypeTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionStoredInfoTypeTimeoutsEl {
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

impl terrars::ToListMappable for DataLossPreventionStoredInfoTypeTimeoutsEl {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoTypeTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoTypeTimeoutsEl {}

impl BuildDataLossPreventionStoredInfoTypeTimeoutsEl {
    pub fn build(self) -> DataLossPreventionStoredInfoTypeTimeoutsEl {
        DataLossPreventionStoredInfoTypeTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionStoredInfoTypeTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoTypeTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionStoredInfoTypeTimeoutsElRef {
        DataLossPreventionStoredInfoTypeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoTypeTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct DataLossPreventionStoredInfoTypeDynamic {
    dictionary: std::option::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoTypeDictionaryEl>>,
    large_custom_dictionary: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoTypeLargeCustomDictionaryEl>>,
    regex: std::option::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoTypeRegexEl>>,
}
