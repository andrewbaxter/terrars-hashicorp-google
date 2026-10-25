#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataLossPreventionStoredInfoTypeData_ {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    parent: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_info_type_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoType_Dictionary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    large_custom_dictionary: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoType_LargeCustomDictionary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoType_Regex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataLossPreventionStoredInfoType_Timeouts>,
    dynamic: DataLossPreventionStoredInfoTypeDynamic,
}

struct DataLossPreventionStoredInfoType_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLossPreventionStoredInfoTypeData_>,
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

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionStoredInfoType_Dictionary>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionStoredInfoType_Regex>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataLossPreventionStoredInfoType_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn dictionary(&self) -> terrars::ListRef<DataLossPreventionStoredInfoType_DictionaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `large_custom_dictionary` after provisioning.\n"]
    pub fn large_custom_dictionary(&self) -> terrars::ListRef<DataLossPreventionStoredInfoType_LargeCustomDictionaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.large_custom_dictionary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::ListRef<DataLossPreventionStoredInfoType_RegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionStoredInfoType_TimeoutsRef {
        DataLossPreventionStoredInfoType_TimeoutsRef::new(
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
            data: std::cell::RefCell::new(DataLossPreventionStoredInfoTypeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn dictionary(&self) -> terrars::ListRef<DataLossPreventionStoredInfoType_DictionaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `large_custom_dictionary` after provisioning.\n"]
    pub fn large_custom_dictionary(&self) -> terrars::ListRef<DataLossPreventionStoredInfoType_LargeCustomDictionaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.large_custom_dictionary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::ListRef<DataLossPreventionStoredInfoType_RegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionStoredInfoType_TimeoutsRef {
        DataLossPreventionStoredInfoType_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionStoredInfoTypeDynamic {
    dictionary: std::option::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoType_Dictionary>>,
    large_custom_dictionary: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoType_LargeCustomDictionary>>,
    regex: std::option::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoType_Regex>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoType_Dictionary_CloudStoragePath {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoType_Dictionary_CloudStoragePath { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoType_Dictionary_CloudStoragePath {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoType_Dictionary_CloudStoragePath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoType_Dictionary_CloudStoragePath {
    #[doc =
        "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoType_Dictionary_CloudStoragePath {
    pub fn build(self) -> DataLossPreventionStoredInfoType_Dictionary_CloudStoragePath {
        DataLossPreventionStoredInfoType_Dictionary_CloudStoragePath { path: self.path }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionStoredInfoType_Dictionary_CloudStoragePathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoType_Dictionary_CloudStoragePathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoType_Dictionary_CloudStoragePathRef {
        DataLossPreventionStoredInfoType_Dictionary_CloudStoragePathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoType_Dictionary_CloudStoragePathRef {
    #[allow(dead_code)]
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
pub struct DataLossPreventionStoredInfoType_Dictionary_WordList {
    words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionStoredInfoType_Dictionary_WordList { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoType_Dictionary_WordList {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoType_Dictionary_WordList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoType_Dictionary_WordList {
    #[doc =
        "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLossPreventionStoredInfoType_Dictionary_WordList {
    pub fn build(self) -> DataLossPreventionStoredInfoType_Dictionary_WordList {
        DataLossPreventionStoredInfoType_Dictionary_WordList { words: self.words }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionStoredInfoType_Dictionary_WordListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoType_Dictionary_WordListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoType_Dictionary_WordListRef {
        DataLossPreventionStoredInfoType_Dictionary_WordListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoType_Dictionary_WordListRef {
    #[allow(dead_code)]
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
struct DataLossPreventionStoredInfoType_DictionaryDynamic {
    cloud_storage_path: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoType_Dictionary_CloudStoragePath>>,
    word_list: std::option::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoType_Dictionary_WordList>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoType_Dictionary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_path: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoType_Dictionary_CloudStoragePath>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    word_list: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoType_Dictionary_WordList>>,
    dynamic: DataLossPreventionStoredInfoType_DictionaryDynamic,
}

impl DataLossPreventionStoredInfoType_Dictionary {
    #[doc = "Set the field `cloud_storage_path`.\n"]
    pub fn set_cloud_storage_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionStoredInfoType_Dictionary_CloudStoragePath>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionStoredInfoType_Dictionary_WordList>>,
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

impl terrars::ToListMappable for DataLossPreventionStoredInfoType_Dictionary {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoType_Dictionary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoType_Dictionary {}

impl BuildDataLossPreventionStoredInfoType_Dictionary {
    pub fn build(self) -> DataLossPreventionStoredInfoType_Dictionary {
        DataLossPreventionStoredInfoType_Dictionary {
            cloud_storage_path: core::default::Default::default(),
            word_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionStoredInfoType_DictionaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoType_DictionaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoType_DictionaryRef {
        DataLossPreventionStoredInfoType_DictionaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoType_DictionaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]
    pub fn cloud_storage_path(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoType_Dictionary_CloudStoragePathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_path", self.base))
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(&self) -> terrars::ListRef<DataLossPreventionStoredInfoType_Dictionary_WordListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Field {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Field { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Field {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Field>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Field {
    #[doc = "Name describing the field."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Field {
    pub fn build(self) -> DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Field {
        DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Field { name: self.name }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_FieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_FieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_FieldRef {
        DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_FieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_FieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName describing the field."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Table {
    dataset_id: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    table_id: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Table { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Table {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Table>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Table {
    #[doc = "The dataset ID of the table."]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc = "The Google Cloud Platform project ID of the project containing the table."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc = "The name of the table."]
    pub table_id: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Table {
    pub fn build(self) -> DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Table {
        DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Table {
            dataset_id: self.dataset_id,
            project_id: self.project_id,
            table_id: self.table_id,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_TableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_TableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_TableRef {
        DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_TableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_TableRef {
    #[allow(dead_code)]
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
struct DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryFieldDynamic {
    field: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Field>>,
    table: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Table>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Field>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Table>>,
    dynamic: DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryFieldDynamic,
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Field>,
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
                        ::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_Table>,
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

impl terrars::ToListMappable for DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField {}

impl BuildDataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField {
    pub fn build(self) -> DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField {
        DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField {
            field: core::default::Default::default(),
            table: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryFieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryFieldRef {
        DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryFieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryFieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_FieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `table` after provisioning.\n"]
    pub fn table(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField_TableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSet {
    url: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSet { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSet {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSet {
    #[doc = "The url, in the format 'gs://<bucket>/<path>'. Trailing wildcard in the path is allowed."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSet {
    pub fn build(self) -> DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSet {
        DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSet { url: self.url }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSetRef {
        DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSetRef {
    #[allow(dead_code)]
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
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPath {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPath { }

impl terrars::ToListMappable for DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPath {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPath {
    #[doc =
        "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPath {
    pub fn build(self) -> DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPath {
        DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPath { path: self.path }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPathRef {
        DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPathRef {
    #[allow(dead_code)]
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
struct DataLossPreventionStoredInfoType_LargeCustomDictionaryDynamic {
    big_query_field: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField>>,
    cloud_storage_file_set: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSet>>,
    output_path: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPath>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    big_query_field: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_file_set: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_path: std::option::Option<std::vec::Vec<DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPath>>,
    dynamic: DataLossPreventionStoredInfoType_LargeCustomDictionaryDynamic,
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionary {
    #[doc = "Set the field `big_query_field`.\n"]
    pub fn set_big_query_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryField>,
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
                        ::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSet>,
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
                        terrars::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPath>,
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

impl terrars::ToListMappable for DataLossPreventionStoredInfoType_LargeCustomDictionary {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoType_LargeCustomDictionary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoType_LargeCustomDictionary {}

impl BuildDataLossPreventionStoredInfoType_LargeCustomDictionary {
    pub fn build(self) -> DataLossPreventionStoredInfoType_LargeCustomDictionary {
        DataLossPreventionStoredInfoType_LargeCustomDictionary {
            big_query_field: core::default::Default::default(),
            cloud_storage_file_set: core::default::Default::default(),
            output_path: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionStoredInfoType_LargeCustomDictionaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoType_LargeCustomDictionaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionStoredInfoType_LargeCustomDictionaryRef {
        DataLossPreventionStoredInfoType_LargeCustomDictionaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoType_LargeCustomDictionaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `big_query_field` after provisioning.\n"]
    pub fn big_query_field(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoType_LargeCustomDictionary_BigQueryFieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.big_query_field", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_storage_file_set` after provisioning.\n"]
    pub fn cloud_storage_file_set(
        &self,
    ) -> terrars::ListRef<DataLossPreventionStoredInfoType_LargeCustomDictionary_CloudStorageFileSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_file_set", self.base))
    }

    #[doc = "Get a reference to the value of field `output_path` after provisioning.\n"]
    pub fn output_path(&self) -> terrars::ListRef<DataLossPreventionStoredInfoType_LargeCustomDictionary_OutputPathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_path", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionStoredInfoType_Regex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionStoredInfoType_Regex {
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

impl terrars::ToListMappable for DataLossPreventionStoredInfoType_Regex {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoType_Regex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoType_Regex {
    #[doc =
        "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionStoredInfoType_Regex {
    pub fn build(self) -> DataLossPreventionStoredInfoType_Regex {
        DataLossPreventionStoredInfoType_Regex {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionStoredInfoType_RegexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoType_RegexRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionStoredInfoType_RegexRef {
        DataLossPreventionStoredInfoType_RegexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoType_RegexRef {
    #[allow(dead_code)]
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
pub struct DataLossPreventionStoredInfoType_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionStoredInfoType_Timeouts {
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

impl terrars::ToListMappable for DataLossPreventionStoredInfoType_Timeouts {
    type O = terrars::BlockAssignable<DataLossPreventionStoredInfoType_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionStoredInfoType_Timeouts {}

impl BuildDataLossPreventionStoredInfoType_Timeouts {
    pub fn build(self) -> DataLossPreventionStoredInfoType_Timeouts {
        DataLossPreventionStoredInfoType_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionStoredInfoType_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionStoredInfoType_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionStoredInfoType_TimeoutsRef {
        DataLossPreventionStoredInfoType_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionStoredInfoType_TimeoutsRef {
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
