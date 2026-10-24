use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DataCatalogTagTemplateData {
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
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    tag_template_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fields: std::option::Option<std::vec::Vec<DataCatalogTagTemplateFieldsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataCatalogTagTemplateTimeoutsEl>,
    dynamic: DataCatalogTagTemplateDynamic,
}

struct DataCatalogTagTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCatalogTagTemplateData>,
}

#[derive(Clone)]
pub struct DataCatalogTagTemplate(std::rc::Rc<DataCatalogTagTemplate_>);

impl DataCatalogTagTemplate {
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

    #[doc = "Set the field `display_name`.\nThe display name for this template."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `force_delete`.\nThis confirms the deletion of any possible tags using this template. Must be set to true in order to delete the tag template."]
    pub fn set_force_delete(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `region`.\nTemplate location region."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fields`.\n"]
    pub fn set_fields(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCatalogTagTemplateFieldsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataCatalogTagTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for this template."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_delete` after provisioning.\nThis confirms the deletion of any possible tags using this template. Must be set to true in order to delete the tag template."]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the tag template in URL format. Example: projects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nTemplate location region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tag_template_id` after provisioning.\nThe id of the tag template to create."]
    pub fn tag_template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_template_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataCatalogTagTemplateTimeoutsElRef {
        DataCatalogTagTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataCatalogTagTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataCatalogTagTemplate { }

impl terrars::ToListMappable for DataCatalogTagTemplate {
    type O = terrars::ListRef<DataCatalogTagTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataCatalogTagTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_data_catalog_tag_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCatalogTagTemplate {
    pub tf_id: std::string::String,
    #[doc = "The id of the tag template to create."]
    pub tag_template_id: terrars::PrimField<std::string::String>,
}

impl BuildDataCatalogTagTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCatalogTagTemplate {
        let out = DataCatalogTagTemplate(std::rc::Rc::new(DataCatalogTagTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCatalogTagTemplateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                display_name: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                id: core::default::Default::default(),
                project: core::default::Default::default(),
                region: core::default::Default::default(),
                tag_template_id: self.tag_template_id,
                fields: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataCatalogTagTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogTagTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCatalogTagTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for this template."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `force_delete` after provisioning.\nThis confirms the deletion of any possible tags using this template. Must be set to true in order to delete the tag template."]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the tag template in URL format. Example: projects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nTemplate location region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `tag_template_id` after provisioning.\nThe id of the tag template to create."]
    pub fn tag_template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_template_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataCatalogTagTemplateTimeoutsElRef {
        DataCatalogTagTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {
    display_name: terrars::PrimField<std::string::String>,
}

impl DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl { }

impl terrars::ToListMappable for DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {
    type O = terrars::BlockAssignable<DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {
    #[doc = "The display name of the enum value."]
    pub display_name: terrars::PrimField<std::string::String>,
}

impl BuildDataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {
    pub fn build(self) -> DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl {
        DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl { display_name: self.display_name }
    }
}

pub struct DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesElRef {
        DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name of the enum value."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCatalogTagTemplateFieldsElTypeElEnumTypeElDynamic {
    allowed_values: std
    ::option
    ::Option<terrars::DynamicBlock<DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl>>,
}

#[derive(Serialize)]
pub struct DataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_values: std::option::Option<std::vec::Vec<DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl>>,
    dynamic: DataCatalogTagTemplateFieldsElTypeElEnumTypeElDynamic,
}

impl DataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
    #[doc = "Set the field `allowed_values`.\n"]
    pub fn set_allowed_values(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataCatalogTagTemplateFieldsElTypeElEnumTypeElAllowedValuesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_values = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
    type O = terrars::BlockAssignable<DataCatalogTagTemplateFieldsElTypeElEnumTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogTagTemplateFieldsElTypeElEnumTypeEl {}

impl BuildDataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
    pub fn build(self) -> DataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
        DataCatalogTagTemplateFieldsElTypeElEnumTypeEl {
            allowed_values: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef {
        DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct DataCatalogTagTemplateFieldsElTypeElDynamic {
    enum_type: std::option::Option<terrars::DynamicBlock<DataCatalogTagTemplateFieldsElTypeElEnumTypeEl>>,
}

#[derive(Serialize)]
pub struct DataCatalogTagTemplateFieldsElTypeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primitive_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enum_type: std::option::Option<std::vec::Vec<DataCatalogTagTemplateFieldsElTypeElEnumTypeEl>>,
    dynamic: DataCatalogTagTemplateFieldsElTypeElDynamic,
}

impl DataCatalogTagTemplateFieldsElTypeEl {
    #[doc =
        "Set the field `primitive_type`.\nRepresents primitive types - string, bool etc.\n Exactly one of 'primitive_type' or 'enum_type' must be set Possible values: [\"DOUBLE\", \"STRING\", \"BOOL\", \"TIMESTAMP\"]"]
    pub fn set_primitive_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.primitive_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enum_type`.\n"]
    pub fn set_enum_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCatalogTagTemplateFieldsElTypeElEnumTypeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.enum_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.enum_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCatalogTagTemplateFieldsElTypeEl {
    type O = terrars::BlockAssignable<DataCatalogTagTemplateFieldsElTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogTagTemplateFieldsElTypeEl {}

impl BuildDataCatalogTagTemplateFieldsElTypeEl {
    pub fn build(self) -> DataCatalogTagTemplateFieldsElTypeEl {
        DataCatalogTagTemplateFieldsElTypeEl {
            primitive_type: core::default::Default::default(),
            enum_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataCatalogTagTemplateFieldsElTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogTagTemplateFieldsElTypeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCatalogTagTemplateFieldsElTypeElRef {
        DataCatalogTagTemplateFieldsElTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogTagTemplateFieldsElTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `primitive_type` after provisioning.\nRepresents primitive types - string, bool etc.\n Exactly one of 'primitive_type' or 'enum_type' must be set Possible values: [\"DOUBLE\", \"STRING\", \"BOOL\", \"TIMESTAMP\"]"]
    pub fn primitive_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primitive_type", self.base))
    }

    #[doc = "Get a reference to the value of field `enum_type` after provisioning.\n"]
    pub fn enum_type(&self) -> terrars::ListRef<DataCatalogTagTemplateFieldsElTypeElEnumTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enum_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataCatalogTagTemplateFieldsElDynamic {
    type_: std::option::Option<terrars::DynamicBlock<DataCatalogTagTemplateFieldsElTypeEl>>,
}

#[derive(Serialize)]
pub struct DataCatalogTagTemplateFieldsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    field_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_required: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    order: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<std::vec::Vec<DataCatalogTagTemplateFieldsElTypeEl>>,
    dynamic: DataCatalogTagTemplateFieldsElDynamic,
}

impl DataCatalogTagTemplateFieldsEl {
    #[doc = "Set the field `description`.\nA description for this field."]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nThe display name for this field."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_required`.\nWhether this is a required field. Defaults to false."]
    pub fn set_is_required(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_required = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `order`.\nThe order of this field with respect to other fields in this tag template.\nA higher value indicates a more important field. The value can be negative.\nMultiple fields can have the same order, and field orders within a tag do not have to be sequential."]
    pub fn set_order(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataCatalogTagTemplateFieldsElTypeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.type_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.type_ = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataCatalogTagTemplateFieldsEl {
    type O = terrars::BlockAssignable<DataCatalogTagTemplateFieldsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogTagTemplateFieldsEl {
    #[doc = ""]
    pub field_id: terrars::PrimField<std::string::String>,
}

impl BuildDataCatalogTagTemplateFieldsEl {
    pub fn build(self) -> DataCatalogTagTemplateFieldsEl {
        DataCatalogTagTemplateFieldsEl {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            field_id: self.field_id,
            is_required: core::default::Default::default(),
            order: core::default::Default::default(),
            type_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataCatalogTagTemplateFieldsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogTagTemplateFieldsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCatalogTagTemplateFieldsElRef {
        DataCatalogTagTemplateFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogTagTemplateFieldsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description for this field."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe display name for this field."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `field_id` after provisioning.\n"]
    pub fn field_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_required` after provisioning.\nWhether this is a required field. Defaults to false."]
    pub fn is_required(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_required", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name of the tag template field in URL format. Example: projects/{project_id}/locations/{location}/tagTemplates/{tagTemplateId}/fields/{field}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `order` after provisioning.\nThe order of this field with respect to other fields in this tag template.\nA higher value indicates a more important field. The value can be negative.\nMultiple fields can have the same order, and field orders within a tag do not have to be sequential."]
    pub fn order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::ListRef<DataCatalogTagTemplateFieldsElTypeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCatalogTagTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCatalogTagTemplateTimeoutsEl {
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

impl terrars::ToListMappable for DataCatalogTagTemplateTimeoutsEl {
    type O = terrars::BlockAssignable<DataCatalogTagTemplateTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCatalogTagTemplateTimeoutsEl {}

impl BuildDataCatalogTagTemplateTimeoutsEl {
    pub fn build(self) -> DataCatalogTagTemplateTimeoutsEl {
        DataCatalogTagTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DataCatalogTagTemplateTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCatalogTagTemplateTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCatalogTagTemplateTimeoutsElRef {
        DataCatalogTagTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCatalogTagTemplateTimeoutsElRef {
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
struct DataCatalogTagTemplateDynamic {
    fields: std::option::Option<terrars::DynamicBlock<DataCatalogTagTemplateFieldsEl>>,
}
