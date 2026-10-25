#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataLossPreventionInspectTemplateData_ {
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
    allow_limited_availability_info_types: std::option::Option<terrars::PrimField<bool>>,
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
    template_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_config: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataLossPreventionInspectTemplate_Timeouts>,
    dynamic: DataLossPreventionInspectTemplateDynamic,
}

struct DataLossPreventionInspectTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLossPreventionInspectTemplateData_>,
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

    #[doc =
        "Set the field `allow_limited_availability_info_types`.\nEnables the use of [limited-availability built-in infoTypes](https://docs.cloud.google.com/sensitive-data-protection/docs/infotypes-reference#limited-availability-infotypes)\nin inspect_config. These infoTypes are supported only in specific regions and can cause scanning errors if used elsewhere."]
    pub fn set_allow_limited_availability_info_types(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().allow_limited_availability_info_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataLossPreventionInspectTemplate_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_limited_availability_info_types` after provisioning.\nEnables the use of [limited-availability built-in infoTypes](https://docs.cloud.google.com/sensitive-data-protection/docs/infotypes-reference#limited-availability-infotypes)\nin inspect_config. These infoTypes are supported only in specific regions and can cause scanning errors if used elsewhere."]
    pub fn allow_limited_availability_info_types(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_limited_availability_info_types", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn inspect_config(&self) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionInspectTemplate_TimeoutsRef {
        DataLossPreventionInspectTemplate_TimeoutsRef::new(
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
            data: std::cell::RefCell::new(DataLossPreventionInspectTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_limited_availability_info_types: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
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
        "Get a reference to the value of field `allow_limited_availability_info_types` after provisioning.\nEnables the use of [limited-availability built-in infoTypes](https://docs.cloud.google.com/sensitive-data-protection/docs/infotypes-reference#limited-availability-infotypes)\nin inspect_config. These infoTypes are supported only in specific regions and can cause scanning errors if used elsewhere."]
    pub fn allow_limited_availability_info_types(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_limited_availability_info_types", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
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
    pub fn inspect_config(&self) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inspect_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLossPreventionInspectTemplate_TimeoutsRef {
        DataLossPreventionInspectTemplate_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplateDynamic {
    inspect_config: std::option::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    path: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath { }

impl terrars
::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    #[doc =
        "A url representing a file or path (no wildcards) in Cloud Storage. Example: 'gs://[BUCKET_NAME]/dictionary.txt'"]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath {
            path: self.path,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef {
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
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordList { }

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    #[doc =
        "Words or phrases defining the dictionary. The dictionary must contain at least one\nphrase and every phrase must contain at least 2 characters that are letters or digits."]
    pub words: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordList {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordList {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordList { words: self.words }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordListRef {
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
struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_DictionaryDynamic {
    cloud_storage_path: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath>,
    >,
    word_list: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordList>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_storage_path: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    word_list: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordList>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_DictionaryDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary {
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
                            DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePath,
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
                            DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordList,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary {
            cloud_storage_path: core::default::Default::default(),
            word_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_DictionaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_DictionaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_DictionaryRef {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_DictionaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_DictionaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_storage_path` after provisioning.\n"]
    pub fn cloud_storage_path(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_CloudStoragePathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_storage_path", self.base))
    }

    #[doc = "Get a reference to the value of field `word_list` after provisioning.\n"]
    pub fn word_list(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary_WordListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.word_list", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore {
    score: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore { }

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore {
    #[doc =
        "The sensitivity score applied to the resource. Possible values: [\"SENSITIVITY_LOW\", \"SENSITIVITY_MODERATE\", \"SENSITIVITY_HIGH\"]"]
    pub score: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore {
        DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore { score: self.score }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScoreRef {
        DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScoreRef {
    #[allow(dead_code)]
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
struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoTypeDynamic {
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoType {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoTypeDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoType {
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
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoType {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoType {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names\nlisted at https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoType {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoType {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoType {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoTypeRef {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoTypeRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex {
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex {
    #[doc =
        "Pattern defining the regular expression.\nIts syntax (https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_RegexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_RegexRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_RegexRef {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_RegexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_RegexRef {
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
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredType {
    name: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredType { }

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredType {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredType {
    #[doc =
        "Resource name of the requested StoredInfoType, for example 'organizations/433245324/storedInfoTypes/432452342'\nor 'projects/project-id/storedInfoTypes/432452342'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredType {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredType {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredType { name: self.name }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredTypeRef {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredTypeRef {
    #[allow(dead_code)]
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
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateType {}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateType { }

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateType {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateType {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateType {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateType {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateType {}
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypesDynamic {
    dictionary: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary>>,
    info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoType>>,
    regex: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex>>,
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>>,
    stored_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredType>>,
    surrogate_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateType>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stored_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surrogate_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateType>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypesDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes {
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
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary>,
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
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoType>,
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
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex>,
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
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>,
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
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredType>,
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
                            DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateType,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes {
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

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypesRef {
        DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypesRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_DictionaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }

    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]
    pub fn info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_InfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_RegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `sensitivity_score` after provisioning.\n"]
    pub fn sensitivity_score(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }

    #[doc = "Get a reference to the value of field `stored_type` after provisioning.\n"]
    pub fn stored_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_StoredTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stored_type", self.base))
    }

    #[doc = "Get a reference to the value of field `surrogate_type` after provisioning.\n"]
    pub fn surrogate_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_SurrogateTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.surrogate_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfig_InfoTypesDynamic {
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_InfoTypes {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_InfoTypesDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_InfoTypes {
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
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_InfoTypes {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_InfoTypes {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_InfoTypes {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_InfoTypes {
        DataLossPreventionInspectTemplate_InspectConfig_InfoTypes {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_InfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_InfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_InfoTypesRef {
        DataLossPreventionInspectTemplate_InspectConfig_InfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_InfoTypesRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeDynamic {
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
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
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>,
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
::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
        DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef {
        DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoTypeDynamic {
    info_type: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType {
    max_findings: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoTypeDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType {
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
                            DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoType,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType {
    #[doc = "Max findings limit for the given infoType."]
    pub max_findings: terrars::PrimField<f64>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType {
        DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType {
            max_findings: self.max_findings,
            info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
        DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoTypeRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType_InfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfig_LimitsDynamic {
    max_findings_per_info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_Limits {
    max_findings_per_item: terrars::PrimField<f64>,
    max_findings_per_request: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_findings_per_info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_LimitsDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_Limits {
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
                            DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoType,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_Limits {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_Limits>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_Limits {
    #[doc = "Max number of findings that will be returned for each item scanned. The maximum returned is 2000."]
    pub max_findings_per_item: terrars::PrimField<f64>,
    #[doc = "Max number of findings that will be returned per request/job. The maximum returned is 2000."]
    pub max_findings_per_request: terrars::PrimField<f64>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_Limits {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_Limits {
        DataLossPreventionInspectTemplate_InspectConfig_Limits {
            max_findings_per_item: self.max_findings_per_item,
            max_findings_per_request: self.max_findings_per_request,
            max_findings_per_info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_LimitsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_LimitsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_LimitsRef {
        DataLossPreventionInspectTemplate_InspectConfig_LimitsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_LimitsRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_Limits_MaxFindingsPerInfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.max_findings_per_info_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoType {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoType {
    #[doc = "Set the field `version`.\nVersion name for this InfoType."]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoType {
    type O =
        terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoType {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoType {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoType {
        DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoType {
            name: self.name,
            version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef {
        DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef {
    #[allow(dead_code)]
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
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoTypeDynamic {
    info_type: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoType>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType {
    min_likelihood: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoType>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoTypeDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType {
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
                            DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoType,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType {
    #[doc =
        "Only returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub min_likelihood: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType {
        DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType {
            min_likelihood: self.min_likelihood,
            info_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoTypeRef {
        DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `min_likelihood` after provisioning.\nOnly returns findings equal or above this threshold. See https://cloud.google.com/dlp/docs/likelihood for more info. Possible values: [\"VERY_UNLIKELY\", \"UNLIKELY\", \"POSSIBLE\", \"LIKELY\", \"VERY_LIKELY\"]"]
    pub fn min_likelihood(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_likelihood", self.base))
    }

    #[doc = "Get a reference to the value of field `info_type` after provisioning.\n"]
    pub fn info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType_InfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypesDynamic {
    sensitivity_score: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_score: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypesDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes {
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
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScore>,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes {
    #[doc =
        "Name of the information type. Either a name of your choosing when creating a CustomInfoType, or one of the names listed\nat https://cloud.google.com/dlp/docs/infotypes-reference when specifying a built-in type."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes {
            name: self.name,
            version: core::default::Default::default(),
            sensitivity_score: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypesRef {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypesRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes_SensitivityScoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sensitivity_score", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_indexes: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    pattern: terrars::PrimField<std::string::String>,
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
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
::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    #[doc =
        "Pattern defining the regular expression. Its syntax\n(https://github.com/google/re2/wiki/Syntax) can be found under the google/re2 repository on GitHub."]
    pub pattern: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex {
            group_indexes: core::default::Default::default(),
            pattern: self.pattern,
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef {
    #[allow(dead_code)]
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
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_Proximity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_before: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_Proximity {
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
::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_Proximity {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_Proximity,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_Proximity {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_Proximity {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_Proximity {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_Proximity {
            window_after: core::default::Default::default(),
            window_before: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_ProximityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_ProximityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_ProximityRef {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_ProximityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_ProximityRef {
    #[allow(dead_code)]
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
struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordDynamic {
    hotword_regex: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>,
    >,
    proximity: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_Proximity,
        >,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_regex: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proximity: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_Proximity>,
    >,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
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
                            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex,
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
                            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_Proximity,
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
::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword {
            hotword_regex: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]
    pub fn hotword_regex(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> terrars
    ::ListRef<
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword_ProximityRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesDynamic {
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes>,
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
::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes {
            info_types: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(&self) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRuleDynamic {
    dictionary: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary>>,
    exclude_by_hotword: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword>,
    >,
    exclude_info_types: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes>,
    >,
    regex: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule {
    matching_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dictionary: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_by_hotword: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_info_types: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRuleDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule {
    #[doc = "Set the field `dictionary`.\n"]
    pub fn set_dictionary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Dictionary>,
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
                            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotword,
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
                            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypes,
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
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_Regex>,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule {
    #[doc =
        "How the rule is applied. See the documentation for more information: https://cloud.google.com/dlp/docs/reference/rest/v2/InspectConfig#MatchingType Possible values: [\"MATCHING_TYPE_FULL_MATCH\", \"MATCHING_TYPE_PARTIAL_MATCH\", \"MATCHING_TYPE_INVERSE_MATCH\"]"]
    pub matching_type: terrars::PrimField<std::string::String>,
}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule {
            matching_type: self.matching_type,
            dictionary: core::default::Default::default(),
            exclude_by_hotword: core::default::Default::default(),
            exclude_info_types: core::default::Default::default(),
            regex: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRuleRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_DictionaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dictionary", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_by_hotword` after provisioning.\n"]
    pub fn exclude_by_hotword(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeByHotwordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_by_hotword", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_info_types` after provisioning.\n"]
    pub fn exclude_info_types(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule_ExcludeInfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes_RegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_likelihood: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relative_likelihood: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
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
::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
    type O =
        terrars
        ::BlockAssignable<
            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
    pub fn build(
        self,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment {
            fixed_likelihood: core::default::Default::default(),
            relative_likelihood: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef {
    #[allow(dead_code)]
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
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    window_before: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
    type O =
        terrars
        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_Proximity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_Proximity {
            window_after: core::default::Default::default(),
            window_before: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef {
    #[allow(dead_code)]
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
struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRuleDynamic {
    hotword_regex: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>,
    >,
    likelihood_adjustment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment>,
    >,
    proximity: std
    ::option
    ::Option<
        terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_Proximity>,
    >,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_regex: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    likelihood_adjustment: std
    ::option
    ::Option<
        std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proximity: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_Proximity>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRuleDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule {
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
                            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegex,
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
                            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustment,
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
                            DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_Proximity,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule {
            hotword_regex: core::default::Default::default(),
            likelihood_adjustment: core::default::Default::default(),
            proximity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRuleRef {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hotword_regex` after provisioning.\n"]
    pub fn hotword_regex(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_HotwordRegexRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `likelihood_adjustment` after provisioning.\n"]
    pub fn likelihood_adjustment(
        &self,
    ) -> terrars
    ::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_LikelihoodAdjustmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.likelihood_adjustment", self.base))
    }

    #[doc = "Get a reference to the value of field `proximity` after provisioning.\n"]
    pub fn proximity(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule_ProximityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proximity", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_RulesDynamic {
    exclusion_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule>>,
    hotword_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusion_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hotword_rule: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_RuleSet_RulesDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules {
    #[doc = "Set the field `exclusion_rule`.\n"]
    pub fn set_exclusion_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRule>,
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
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRule>,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules {
            exclusion_rule: core::default::Default::default(),
            hotword_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_RuleSet_RulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet_RulesRef {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclusion_rule` after provisioning.\n"]
    pub fn exclusion_rule(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_ExclusionRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusion_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `hotword_rule` after provisioning.\n"]
    pub fn hotword_rule(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules_HotwordRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hotword_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfig_RuleSetDynamic {
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes>>,
    rules: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfig_RuleSetDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSet {
    #[doc = "Set the field `info_types`.\n"]
    pub fn set_info_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypes>,
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
                    ::Into<terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_Rules>>,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig_RuleSet {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig_RuleSet {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSet {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSet {
            info_types: core::default::Default::default(),
            rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfig_RuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfig_RuleSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfig_RuleSetRef {
        DataLossPreventionInspectTemplate_InspectConfig_RuleSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfig_RuleSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(&self) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_InfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `rules` after provisioning.\n"]
    pub fn rules(&self) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSet_RulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rules", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLossPreventionInspectTemplate_InspectConfigDynamic {
    custom_info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes>>,
    info_types: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes>>,
    limits: std::option::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_Limits>>,
    min_likelihood_per_info_type: std
    ::option
    ::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType>>,
    rule_set: std::option::Option<terrars::DynamicBlock<DataLossPreventionInspectTemplate_InspectConfig_RuleSet>>,
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_InspectConfig {
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
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info_types: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_Limits>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_likelihood_per_info_type: std
    ::option
    ::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_set: std::option::Option<std::vec::Vec<DataLossPreventionInspectTemplate_InspectConfig_RuleSet>>,
    dynamic: DataLossPreventionInspectTemplate_InspectConfigDynamic,
}

impl DataLossPreventionInspectTemplate_InspectConfig {
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
                    ::Into<terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypes>>,
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
                    ::Into<terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_InfoTypes>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_Limits>>,
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

    #[doc = "Set the field `min_likelihood_per_info_type`.\n"]
    pub fn set_min_likelihood_per_info_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoType>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.min_likelihood_per_info_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.min_likelihood_per_info_type = std::option::Option::Some(d);
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
                    ::Into<terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig_RuleSet>>,
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_InspectConfig {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_InspectConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_InspectConfig {}

impl BuildDataLossPreventionInspectTemplate_InspectConfig {
    pub fn build(self) -> DataLossPreventionInspectTemplate_InspectConfig {
        DataLossPreventionInspectTemplate_InspectConfig {
            content_options: core::default::Default::default(),
            exclude_info_types: core::default::Default::default(),
            include_quote: core::default::Default::default(),
            min_likelihood: core::default::Default::default(),
            custom_info_types: core::default::Default::default(),
            info_types: core::default::Default::default(),
            limits: core::default::Default::default(),
            min_likelihood_per_info_type: core::default::Default::default(),
            rule_set: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_InspectConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_InspectConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLossPreventionInspectTemplate_InspectConfigRef {
        DataLossPreventionInspectTemplate_InspectConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_InspectConfigRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_CustomInfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `info_types` after provisioning.\n"]
    pub fn info_types(&self) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_InfoTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info_types", self.base))
    }

    #[doc = "Get a reference to the value of field `limits` after provisioning.\n"]
    pub fn limits(&self) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_LimitsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc = "Get a reference to the value of field `min_likelihood_per_info_type` after provisioning.\n"]
    pub fn min_likelihood_per_info_type(
        &self,
    ) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_MinLikelihoodPerInfoTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.min_likelihood_per_info_type", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_set` after provisioning.\n"]
    pub fn rule_set(&self) -> terrars::ListRef<DataLossPreventionInspectTemplate_InspectConfig_RuleSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_set", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLossPreventionInspectTemplate_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLossPreventionInspectTemplate_Timeouts {
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

impl terrars::ToListMappable for DataLossPreventionInspectTemplate_Timeouts {
    type O = terrars::BlockAssignable<DataLossPreventionInspectTemplate_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLossPreventionInspectTemplate_Timeouts {}

impl BuildDataLossPreventionInspectTemplate_Timeouts {
    pub fn build(self) -> DataLossPreventionInspectTemplate_Timeouts {
        DataLossPreventionInspectTemplate_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataLossPreventionInspectTemplate_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLossPreventionInspectTemplate_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLossPreventionInspectTemplate_TimeoutsRef {
        DataLossPreventionInspectTemplate_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLossPreventionInspectTemplate_TimeoutsRef {
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
