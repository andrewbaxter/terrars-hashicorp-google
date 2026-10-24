use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataLossPreventionInspectTemplateData {
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
    template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_config: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataLossPreventionInspectTemplateTimeoutsEl>,
    dynamic: DataLossPreventionInspectTemplateDynamic,
}

struct DataLossPreventionInspectTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLossPreventionInspectTemplateData>,
}

#[derive(Clone)]
pub struct DataLossPreventionInspectTemplate(std::rc::Rc<DataLossPreventionInspectTemplate_>);

impl DataLossPreventionInspectTemplate {
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

    #[doc = "Set the field `description`.\nA description of the inspect template."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser set display name of the inspect template."]
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
        "Set the field `template_id`.\nThe template id can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is\n100 characters. Can be empty to allow the system to generate one."]
    pub fn set_template_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().template_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inspect_config`.\n"]
    pub fn set_inspect_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inspect_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inspect_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataLossPreventionInspectTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the inspect template."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser set display name of the inspect template."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the inspect template. Set by the server."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the inspect template in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `template_id` after provisioning.\nThe template id can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is\n100 characters. Can be empty to allow the system to generate one."]
    pub fn template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inspect_config` after provisioning.\n"]
    pub fn inspect_config(&self) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionInspectTemplateTimeoutsElRef {
        DataLossPreventionInspectTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataLossPreventionInspectTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataLossPreventionInspectTemplate { }

impl terrars::ToListMappable for DataLossPreventionInspectTemplate {
    type O = terrars::ListRef<DataLossPreventionInspectTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataLossPreventionInspectTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_data_loss_prevention_inspect_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLossPreventionInspectTemplate {
    pub tf_id: std::string::String,
    #[doc =
        "The parent of the inspect template in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub parent: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLossPreventionInspectTemplate {
        let out = DataLossPreventionInspectTemplate(std::rc::Rc::new(DataLossPreventionInspectTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLossPreventionInspectTemplateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                parent: self.parent,
                template_id: core::default::Default::default(),
                inspect_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataLossPreventionInspectTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLossPreventionInspectTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the inspect template."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser set display name of the inspect template."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the inspect template. Set by the server."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parent` after provisioning.\nThe parent of the inspect template in any of the following formats:\n\n* 'projects/{{project}}'\n* 'projects/{{project}}/locations/{{location}}'\n* 'organizations/{{organization_id}}'\n* 'organizations/{{organization_id}}/locations/{{location}}'"]
    pub fn parent(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parent", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `template_id` after provisioning.\nThe template id can contain uppercase and lowercase letters, numbers, and hyphens;\nthat is, it must match the regular expression: [a-zA-Z\\d-_]+. The maximum length is\n100 characters. Can be empty to allow the system to generate one."]
    pub fn template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inspect_config` after provisioning.\n"]
    pub fn inspect_config(&self) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionInspectTemplateTimeoutsElRef {
        DataLossPreventionInspectTemplateTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl { }

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
    #[doc =
        "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl {
            path: self.path,
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef {
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
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
    words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl { }

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
    #[doc =
        "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl { words: self.words }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef {
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
struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElDynamic {
    cloud_storage_path: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl,
        >,
    >,
    word_list: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_path: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    word_list: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {
    #[doc = "Set the field `cloud_storage_path`.\n"]
    pub fn set_cloud_storage_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathEl,
                        >,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListEl,
                        >,
                    >,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl {
            cloud_storage_path: core::default::Default::default(),
            word_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]
    pub fn cloud_storage_path(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElCloudStoragePathElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_path", self.base))
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElWordListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl { }

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl {
            score: self.score,
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl>,
    >,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names\nlisted at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names\nlisted at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElSensitivityScoreElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
    #[doc =
        "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef {
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
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl { }

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl { score: self.score }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl { }

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl {
    #[doc =
        "Resource name of the requested StoredInfoType, for example 'organizations/433245324/storedInfoTypes/432452342'\nor 'projects/project-id/storedInfoTypes/432452342'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl { name: self.name }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nResource name of the requested StoredInfoType, for example 'organizations/433245324/storedInfoTypes/432452342'\nor 'projects/project-id/storedInfoTypes/432452342'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl { }

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {
    type O =
        terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl {}
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDynamic {
    dictionary: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl>>,
    info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl>>,
    regex: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl>>,
    sensitivity_score: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl>,
    >,
    stored_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl>>,
    surrogate_type: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surrogate_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
    #[doc =
        "Set the field `exclusion_type`.\nIf set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching. Possible values: [\"EXCLUSION_TYPE_EXCLUDE\"]"]
    pub fn set_exclusion_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exclusion_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `likelihood`.\nLikelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria\nspecified by the rule. Default value: \"VERY_LIKELY\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.likelihood = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dictionary`.\n"]
    pub fn set_dictionary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dictionary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dictionary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `info_type`.\n"]
    pub fn set_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stored_type`.\n"]
    pub fn set_stored_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.stored_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.stored_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `surrogate_type`.\n"]
    pub fn set_surrogate_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.surrogate_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.surrogate_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl {
            exclusion_type: core::default::Default::default(),
            likelihood: core::default::Default::default(),
            dictionary: core::default::Default::default(),
            info_type: core::default::Default::default(),
            regex: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            stored_type: core::default::Default::default(),
            surrogate_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef {
        DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `exclusion_type` after provisioning.\nIf set to EXCLUSION_TYPE_EXCLUDE this infoType will not cause a finding to be returned. It still can be used for rules matching. Possible values: [\"EXCLUSION_TYPE_EXCLUDE\"]"]
    pub fn exclusion_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclusion_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `likelihood` after provisioning.\nLikelihood to return for this CustomInfoType. This base value can be altered by a detection rule if the finding meets the criteria\nspecified by the rule. Default value: \"VERY_LIKELY\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.likelihood", self.base))
    }

    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]
    pub fn dictionary(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElDictionaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }

    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]
    pub fn info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElInfoTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRegexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSensitivityScoreElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }

    #[doc = "Get a reference to the value of field `stored_type` after provisioning.\n"]
    pub fn stored_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElStoredTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stored_type", self.base))
    }

    #[doc = "Get a reference to the value of field `surrogate_type` after provisioning.\n"]
    pub fn surrogate_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElSurrogateTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.surrogate_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl { }

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl {
        DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl { score: self.score }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef {
        DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElInfoTypesElDynamic {
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElInfoTypesElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
    #[doc =
        "Set the field `version`.\nVersion of the information type to use. By default, the version is set to stable"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
        DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef {
        DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the information type to use. By default, the version is set to stable"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElInfoTypesElSensitivityScoreElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl { }

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl {
            score: self.score,
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef {
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl,
        >,
    >,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef {
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElSensitivityScoreElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElDynamic {
    info_type: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
    max_findings: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl>,
    >,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
    #[doc = "Set the field `info_type`.\n"]
    pub fn set_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
    type O =
        terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
    #[doc = "Max findings limit for the given infoType."]
    pub max_findings: terrars::PrimField<f64>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl {
            max_findings: self.max_findings,
            info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
        DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_findings` after provisioning.\nMax findings limit for the given infoType."]
    pub fn max_findings(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_findings", self.base))
    }

    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]
    pub fn info_type(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElInfoTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElLimitsElDynamic {
    max_findings_per_info_type: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsEl {
    max_findings_per_item: terrars::PrimField<f64>,
    max_findings_per_request: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_findings_per_info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElLimitsElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElLimitsEl {
    #[doc = "Set the field `max_findings_per_info_type`.\n"]
    pub fn set_max_findings_per_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.max_findings_per_info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.max_findings_per_info_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElLimitsEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElLimitsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElLimitsEl {
    #[doc = "Max number of findings that will be returned for each item scanned. The maximum returned is 2000."]
    pub max_findings_per_item: terrars::PrimField<f64>,
    #[doc = "Max number of findings that will be returned per request/job. The maximum returned is 2000."]
    pub max_findings_per_request: terrars::PrimField<f64>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElLimitsEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElLimitsEl {
        DataLossPreventionInspectTemplateInspectConfigElLimitsEl {
            max_findings_per_item: self.max_findings_per_item,
            max_findings_per_request: self.max_findings_per_request,
            max_findings_per_info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElLimitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElLimitsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElLimitsElRef {
        DataLossPreventionInspectTemplateInspectConfigElLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElLimitsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_findings_per_item` after provisioning.\nMax number of findings that will be returned for each item scanned. The maximum returned is 2000."]
    pub fn max_findings_per_item(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_findings_per_item", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_findings_per_request` after provisioning.\nMax number of findings that will be returned per request/job. The maximum returned is 2000."]
    pub fn max_findings_per_request(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_findings_per_request", self.base))
    }

    #[doc = "Get a reference to the value of field `max_findings_per_info_type` after provisioning.\n"]
    pub fn max_findings_per_info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElLimitsElMaxFindingsPerInfoTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_findings_per_info_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl { }

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl { score: self.score }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElSensitivityScoreElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl { }

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
    #[doc =
        "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl {
            path: self.path,
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef {
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
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
    words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl { }

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
    #[doc =
        "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl {
            words: self.words,
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef {
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
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElDynamic {
    cloud_storage_path: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl,
        >,
    >,
    word_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_path: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    word_list: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl>,
    >,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
    #[doc = "Set the field `cloud_storage_path`.\n"]
    pub fn set_cloud_storage_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathEl,
                        >,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListEl,
                        >,
                    >,
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

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl {
            cloud_storage_path: core::default::Default::default(),
            word_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]
    pub fn cloud_storage_path(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElCloudStoragePathElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_path", self.base))
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElWordListElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
    #[doc =
        "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."]
    pub fn set_group_indexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.group_indexes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
    #[doc =
        "Pattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `group_indexes` after provisioning.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."]
    pub fn group_indexes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_indexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nPattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_before: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
    #[doc = "Set the field `window_after`.\nNumber of characters after the finding to consider."]
    pub fn set_window_after(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.window_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `window_before`.\nNumber of characters before the finding to consider."]
    pub fn set_window_before(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.window_before = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl {
            window_after: core::default::Default::default(),
            window_before: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `window_after` after provisioning.\nNumber of characters after the finding to consider."]
    pub fn window_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `window_before` after provisioning.\nNumber of characters before the finding to consider."]
    pub fn window_before(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_before", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElDynamic {
    hotword_regex: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl,
        >,
    >,
    proximity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_regex: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proximity: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl,
        >,
    >,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
    #[doc = "Set the field `hotword_regex`.\n"]
    pub fn set_hotword_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hotword_regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proximity`.\n"]
    pub fn set_proximity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proximity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proximity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl {
            hotword_regex: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]
    pub fn hotword_regex(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElHotwordRegexElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElProximityElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {

}

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl {
            score: self.score,
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `score` after provisioning.\nThe sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub fn score(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElDynamic {
    sensitivity_score: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl,
        >,
    >,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sensitivity_score`.\n"]
    pub fn set_sensitivity_score(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sensitivity_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sensitivity_score = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nVersion name for this InfoType."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElSensitivityScoreElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElDynamic {
    info_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl,
        >,
    >,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl {
            info_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElInfoTypesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
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

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
    #[doc =
        "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef {
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

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDynamic {
    dictionary: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl>,
    >,
    exclude_by_hotword: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl,
        >,
    >,
    exclude_info_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl,
        >,
    >,
    regex: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
    matching_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_by_hotword: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_info_types: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
    #[doc = "Set the field `dictionary`.\n"]
    pub fn set_dictionary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dictionary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dictionary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude_by_hotword`.\n"]
    pub fn set_exclude_by_hotword(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude_by_hotword = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_by_hotword = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude_info_types`.\n"]
    pub fn set_exclude_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude_info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude_info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
    type O =
        terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
    #[doc =
        "How the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\"]"]
    pub matching_type: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl {
            matching_type: self.matching_type,
            dictionary: core::default::Default::default(),
            exclude_by_hotword: core::default::Default::default(),
            exclude_info_types: core::default::Default::default(),
            regex: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `matching_type` after provisioning.\nHow the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\"]"]
    pub fn matching_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.matching_type", self.base))
    }

    #[doc = "Get a reference to the value of field `dictionary` after provisioning.\n"]
    pub fn dictionary(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElDictionaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_by_hotword` after provisioning.\n"]
    pub fn exclude_by_hotword(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeByHotwordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_by_hotword", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_info_types` after provisioning.\n"]
    pub fn exclude_info_types(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElExcludeInfoTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRegexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
    #[doc =
        "Set the field `group_indexes`.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."]
    pub fn set_group_indexes(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.group_indexes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
    #[doc =
        "Pattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `group_indexes` after provisioning.\nThe index of the submatch to extract as findings. When not specified,\nthe entire match is returned. No more than 3 may be included."]
    pub fn group_indexes(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.group_indexes", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pattern` after provisioning.\nPattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relative_likelihood: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
    #[doc =
        "Set the field `fixed_likelihood`.\nSet the likelihood of a finding to a fixed value. Either this or relative_likelihood can be set. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_fixed_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fixed_likelihood = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `relative_likelihood`.\nIncrease or decrease the likelihood by the specified number of levels. For example,\nif a finding would be POSSIBLE without the detection rule and relativeLikelihood is 1,\nthen it is upgraded to LIKELY, while a value of -1 would downgrade it to UNLIKELY.\nLikelihood may never drop below VERY_UNLIKELY or exceed VERY_LIKELY, so applying an\nadjustment of 1 followed by an adjustment of -1 when base likelihood is VERY_LIKELY\nwill result in a final likelihood of LIKELY. Either this or fixed_likelihood can be set."]
    pub fn set_relative_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.relative_likelihood = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl {
            fixed_likelihood: core::default::Default::default(),
            relative_likelihood: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `fixed_likelihood` after provisioning.\nSet the likelihood of a finding to a fixed value. Either this or relative_likelihood can be set. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn fixed_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fixed_likelihood", self.base))
    }

    #[doc =
        "Get a reference to the value of field `relative_likelihood` after provisioning.\nIncrease or decrease the likelihood by the specified number of levels. For example,\nif a finding would be POSSIBLE without the detection rule and relativeLikelihood is 1,\nthen it is upgraded to LIKELY, while a value of -1 would downgrade it to UNLIKELY.\nLikelihood may never drop below VERY_UNLIKELY or exceed VERY_LIKELY, so applying an\nadjustment of 1 followed by an adjustment of -1 when base likelihood is VERY_LIKELY\nwill result in a final likelihood of LIKELY. Either this or fixed_likelihood can be set."]
    pub fn relative_likelihood(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relative_likelihood", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_before: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
    #[doc =
        "Set the field `window_after`.\nNumber of characters after the finding to consider. Either this or window_before must be specified"]
    pub fn set_window_after(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.window_after = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `window_before`.\nNumber of characters before the finding to consider. Either this or window_after must be specified"]
    pub fn set_window_before(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.window_before = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl {
            window_after: core::default::Default::default(),
            window_before: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `window_after` after provisioning.\nNumber of characters after the finding to consider. Either this or window_before must be specified"]
    pub fn window_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_after", self.base))
    }

    #[doc =
        "Get a reference to the value of field `window_before` after provisioning.\nNumber of characters before the finding to consider. Either this or window_after must be specified"]
    pub fn window_before(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.window_before", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElDynamic {
    hotword_regex: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl>,
    >,
    likelihood_adjustment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl,
        >,
    >,
    proximity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_regex: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood_adjustment: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proximity: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {
    #[doc = "Set the field `hotword_regex`.\n"]
    pub fn set_hotword_regex(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hotword_regex = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_regex = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `likelihood_adjustment`.\n"]
    pub fn set_likelihood_adjustment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.likelihood_adjustment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.likelihood_adjustment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proximity`.\n"]
    pub fn set_proximity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.proximity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.proximity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl {
            hotword_regex: core::default::Default::default(),
            likelihood_adjustment: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]
    pub fn hotword_regex(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElHotwordRegexElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `likelihood_adjustment` after provisioning.\n"]
    pub fn likelihood_adjustment(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElLikelihoodAdjustmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.likelihood_adjustment", self.base))
    }

    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElProximityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElDynamic {
    exclusion_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl>>,
    hotword_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
    #[doc = "Set the field `exclusion_rule`.\n"]
    pub fn set_exclusion_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusion_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusion_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hotword_rule`.\n"]
    pub fn set_hotword_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hotword_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hotword_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl {
            exclusion_rule: core::default::Default::default(),
            hotword_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_rule` after provisioning.\n"]
    pub fn exclusion_rule(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElExclusionRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `hotword_rule` after provisioning.\n"]
    pub fn hotword_rule(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElHotwordRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElDynamic {
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl>>,
    rules: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElRuleSetElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetEl {
            info_types: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef {
        DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElInfoTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateInspectConfigElDynamic {
    custom_info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl>>,
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl>>,
    limits: std::option::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElLimitsEl>>,
    rule_set: std::option::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigElRuleSetEl>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateInspectConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_info_types: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_quote: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_info_types: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElLimitsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_set: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplateInspectConfigElRuleSetEl>>,
    dynamic: DataLossPreventionInspectTemplateInspectConfigElDynamic,
}

impl DataLossPreventionInspectTemplateInspectConfigEl {
    #[doc =
        "Set the field `content_options`.\nList of options defining data content to scan. If empty, text, images, and other content will be included. Possible values: [\"CONTENT_TEXT\", \"CONTENT_IMAGE\"]"]
    pub fn set_content_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.content_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_info_types`.\nWhen true, excludes type information of the findings."]
    pub fn set_exclude_info_types(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_info_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `include_quote`.\nWhen true, a contextual quote from the data that triggered a finding is included in the response."]
    pub fn set_include_quote(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_quote = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_likelihood`.\nOnly returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info Default value: \"POSSIBLE\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn set_min_likelihood(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.min_likelihood = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_info_types`.\n"]
    pub fn set_custom_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElInfoTypesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `limits`.\n"]
    pub fn set_limits(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElLimitsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_set`.\n"]
    pub fn set_rule_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigElRuleSetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_set = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplateInspectConfigEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateInspectConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateInspectConfigEl {}

impl BuildDataLossPreventionInspectTemplateInspectConfigEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateInspectConfigEl {
        DataLossPreventionInspectTemplateInspectConfigEl {
            content_options: core::default::Default::default(),
            exclude_info_types: core::default::Default::default(),
            include_quote: core::default::Default::default(),
            min_likelihood: core::default::Default::default(),
            custom_info_types: core::default::Default::default(),
            info_types: core::default::Default::default(),
            limits: core::default::Default::default(),
            rule_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateInspectConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateInspectConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateInspectConfigElRef {
        DataLossPreventionInspectTemplateInspectConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateInspectConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `content_options` after provisioning.\nList of options defining data content to scan. If empty, text, images, and other content will be included. Possible values: [\"CONTENT_TEXT\", \"CONTENT_IMAGE\"]"]
    pub fn content_options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.content_options", self.base))
    }

    #[doc =
        "Get a reference to the value of field `exclude_info_types` after provisioning.\nWhen true, excludes type information of the findings."]
    pub fn exclude_info_types(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_info_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `include_quote` after provisioning.\nWhen true, a contextual quote from the data that triggered a finding is included in the response."]
    pub fn include_quote(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_quote", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_likelihood` after provisioning.\nOnly returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info Default value: \"POSSIBLE\" Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn min_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_likelihood", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_info_types` after provisioning.\n"]
    pub fn custom_info_types(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElCustomInfoTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(&self) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElInfoTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElLimitsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_set` after provisioning.\n"]
    pub fn rule_set(&self) -> terrars::ListRef<DataLossPreventionInspectTemplateInspectConfigElRuleSetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_set", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionInspectTemplateTimeoutsEl {
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplateTimeoutsEl {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplateTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplateTimeoutsEl {}

impl BuildDataLossPreventionInspectTemplateTimeoutsEl {
    pub fn build(self) -> DataLossPreventionInspectTemplateTimeoutsEl {
        DataLossPreventionInspectTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DataLossPreventionInspectTemplateTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplateTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplateTimeoutsElRef {
        DataLossPreventionInspectTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplateTimeoutsElRef {
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
struct DataLossPreventionInspectTemplateDynamic {
    inspect_config: std::option::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplateInspectConfigEl>>,
}
