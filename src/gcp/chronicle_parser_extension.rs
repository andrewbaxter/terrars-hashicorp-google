#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleParserExtensionData_ {
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
    cbn_snippet: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log: std::option::Option<terrars::PrimField<std::string::String>>,
    log_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_skipped: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamic_parsing: std::option::Option<std::vec::Vec<ChronicleParserExtension_DynamicParsing>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_extractors: std::option::Option<std::vec::Vec<ChronicleParserExtension_FieldExtractors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleParserExtension_Timeouts>,
    dynamic: ChronicleParserExtensionDynamic,
}

struct ChronicleParserExtension_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleParserExtensionData_>,
}

#[derive(Clone)]
pub struct ChronicleParserExtension(std::rc::Rc<ChronicleParserExtension_>);

impl ChronicleParserExtension {
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

    #[doc = "Set the field `cbn_snippet`.\nParser config could be a cbn snippet."]
    pub fn set_cbn_snippet(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cbn_snippet = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log`.\nRaw log used to assist the user in creation of augmentation."]
    pub fn set_log(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `validation_skipped`.\nFlag to bypass parser extension validation.\nIf enabled, the parser extension won't be rejected during the validation\nphase and validation will be skipped."]
    pub fn set_validation_skipped(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().validation_skipped = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dynamic_parsing`.\n"]
    pub fn set_dynamic_parsing(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleParserExtension_DynamicParsing>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamic_parsing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamic_parsing = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `field_extractors`.\n"]
    pub fn set_field_extractors(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleParserExtension_FieldExtractors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().field_extractors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.field_extractors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleParserExtension_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `cbn_snippet` after provisioning.\nParser config could be a cbn snippet."]
    pub fn cbn_snippet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cbn_snippet", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the parser extension was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `extension_validation_report` after provisioning.\nThe latest extension\nvalidation report for this extension."]
    pub fn extension_validation_report(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extension_validation_report", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_live_time` after provisioning.\nThe time the config was last serving live traffic."]
    pub fn last_live_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_live_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log` after provisioning.\nRaw log used to assist the user in creation of augmentation."]
    pub fn log(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_type` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/logTypes/{logtype}/parserExtensions/{parserExtension}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parserextension` after provisioning.\nOutput only. The server-generated ID of the parser extension."]
    pub fn parserextension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parserextension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the parser extension\nPossible values:\nNEW\nVALIDATING\nLIVE\nREJECTED\nINTERNAL_ERROR\nVALIDATED\nARCHIVED\nVALIDATION_SKIPPED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_last_changed_time` after provisioning.\nThe time the config state was last changed."]
    pub fn state_last_changed_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_last_changed_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_report` after provisioning.\nThe validation report generated during extension validation."]
    pub fn validation_report(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_report", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_skipped` after provisioning.\nFlag to bypass parser extension validation.\nIf enabled, the parser extension won't be rejected during the validation\nphase and validation will be skipped."]
    pub fn validation_skipped(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_skipped", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamic_parsing` after provisioning.\n"]
    pub fn dynamic_parsing(&self) -> terrars::ListRef<ChronicleParserExtension_DynamicParsingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamic_parsing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `field_extractors` after provisioning.\n"]
    pub fn field_extractors(&self) -> terrars::ListRef<ChronicleParserExtension_FieldExtractorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_extractors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleParserExtension_TimeoutsRef {
        ChronicleParserExtension_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleParserExtension {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleParserExtension { }

impl terrars::ToListMappable for ChronicleParserExtension {
    type O = terrars::ListRef<ChronicleParserExtensionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleParserExtension_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_parser_extension".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleParserExtension {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub log_type: terrars::PrimField<std::string::String>,
}

impl BuildChronicleParserExtension {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleParserExtension {
        let out = ChronicleParserExtension(std::rc::Rc::new(ChronicleParserExtension_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleParserExtensionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cbn_snippet: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                log: core::default::Default::default(),
                log_type: self.log_type,
                project: core::default::Default::default(),
                validation_skipped: core::default::Default::default(),
                dynamic_parsing: core::default::Default::default(),
                field_extractors: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleParserExtensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParserExtensionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleParserExtensionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cbn_snippet` after provisioning.\nParser config could be a cbn snippet."]
    pub fn cbn_snippet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cbn_snippet", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nThe time the parser extension was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `extension_validation_report` after provisioning.\nThe latest extension\nvalidation report for this extension."]
    pub fn extension_validation_report(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extension_validation_report", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `instance` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_live_time` after provisioning.\nThe time the config was last serving live traffic."]
    pub fn last_live_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_live_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log` after provisioning.\nRaw log used to assist the user in creation of augmentation."]
    pub fn log(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_type` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn log_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/logTypes/{logtype}/parserExtensions/{parserExtension}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parserextension` after provisioning.\nOutput only. The server-generated ID of the parser extension."]
    pub fn parserextension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parserextension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the parser extension\nPossible values:\nNEW\nVALIDATING\nLIVE\nREJECTED\nINTERNAL_ERROR\nVALIDATED\nARCHIVED\nVALIDATION_SKIPPED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state_last_changed_time` after provisioning.\nThe time the config state was last changed."]
    pub fn state_last_changed_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_last_changed_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_report` after provisioning.\nThe validation report generated during extension validation."]
    pub fn validation_report(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_report", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_skipped` after provisioning.\nFlag to bypass parser extension validation.\nIf enabled, the parser extension won't be rejected during the validation\nphase and validation will be skipped."]
    pub fn validation_skipped(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_skipped", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamic_parsing` after provisioning.\n"]
    pub fn dynamic_parsing(&self) -> terrars::ListRef<ChronicleParserExtension_DynamicParsingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamic_parsing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `field_extractors` after provisioning.\n"]
    pub fn field_extractors(&self) -> terrars::ListRef<ChronicleParserExtension_FieldExtractorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_extractors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleParserExtension_TimeoutsRef {
        ChronicleParserExtension_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleParserExtensionDynamic {
    dynamic_parsing: std::option::Option<terrars::DynamicBlock<ChronicleParserExtension_DynamicParsing>>,
    field_extractors: std::option::Option<terrars::DynamicBlock<ChronicleParserExtension_FieldExtractors>>,
}

#[derive(Serialize)]
pub struct ChronicleParserExtension_DynamicParsing_OptedFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sample_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleParserExtension_DynamicParsing_OptedFields {
    #[doc = "Set the field `path`.\nPath of the log field."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sample_value`.\nSample value of the log field."]
    pub fn set_sample_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sample_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleParserExtension_DynamicParsing_OptedFields {
    type O = terrars::BlockAssignable<ChronicleParserExtension_DynamicParsing_OptedFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParserExtension_DynamicParsing_OptedFields {}

impl BuildChronicleParserExtension_DynamicParsing_OptedFields {
    pub fn build(self) -> ChronicleParserExtension_DynamicParsing_OptedFields {
        ChronicleParserExtension_DynamicParsing_OptedFields {
            path: core::default::Default::default(),
            sample_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParserExtension_DynamicParsing_OptedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParserExtension_DynamicParsing_OptedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleParserExtension_DynamicParsing_OptedFieldsRef {
        ChronicleParserExtension_DynamicParsing_OptedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParserExtension_DynamicParsing_OptedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath of the log field."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `sample_value` after provisioning.\nSample value of the log field."]
    pub fn sample_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sample_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleParserExtension_DynamicParsingDynamic {
    opted_fields: std::option::Option<terrars::DynamicBlock<ChronicleParserExtension_DynamicParsing_OptedFields>>,
}

#[derive(Serialize)]
pub struct ChronicleParserExtension_DynamicParsing {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opted_fields: std::option::Option<std::vec::Vec<ChronicleParserExtension_DynamicParsing_OptedFields>>,
    dynamic: ChronicleParserExtension_DynamicParsingDynamic,
}

impl ChronicleParserExtension_DynamicParsing {
    #[doc = "Set the field `opted_fields`.\n"]
    pub fn set_opted_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleParserExtension_DynamicParsing_OptedFields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.opted_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.opted_fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleParserExtension_DynamicParsing {
    type O = terrars::BlockAssignable<ChronicleParserExtension_DynamicParsing>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParserExtension_DynamicParsing {}

impl BuildChronicleParserExtension_DynamicParsing {
    pub fn build(self) -> ChronicleParserExtension_DynamicParsing {
        ChronicleParserExtension_DynamicParsing {
            opted_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParserExtension_DynamicParsingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParserExtension_DynamicParsingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleParserExtension_DynamicParsingRef {
        ChronicleParserExtension_DynamicParsingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParserExtension_DynamicParsingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `opted_fields` after provisioning.\n"]
    pub fn opted_fields(&self) -> terrars::ListRef<ChronicleParserExtension_DynamicParsing_OptedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.opted_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleParserExtension_FieldExtractors_Extractors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    precondition_op: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    precondition_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    precondition_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleParserExtension_FieldExtractors_Extractors {
    #[doc =
        "Set the field `destination_path`.\nPath in generated event which is to be populated. This is required if the\nFieldExtractor is used to specify the parser extension."]
    pub fn set_destination_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `field_path`.\nField path could be a json path, xml path or csv column name\ndepending on log format. It refers to a section or substring in raw log.\nThis is required if the FieldExtractor is used to specify the parser\nextension."]
    pub fn set_field_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `precondition_op`.\nOperator used for precondition.\nPossible values:\nEQUALS\nNOT_EQUALS"]
    pub fn set_precondition_op(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.precondition_op = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `precondition_path`.\nPrecondition path could be a json path, xml path or csv column name\ndepending on log format. It refers to a section or substring in raw log."]
    pub fn set_precondition_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.precondition_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `precondition_value`.\nPrecondition value."]
    pub fn set_precondition_value(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.precondition_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nValue to be mapped to the destination path directly."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleParserExtension_FieldExtractors_Extractors {
    type O = terrars::BlockAssignable<ChronicleParserExtension_FieldExtractors_Extractors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParserExtension_FieldExtractors_Extractors {}

impl BuildChronicleParserExtension_FieldExtractors_Extractors {
    pub fn build(self) -> ChronicleParserExtension_FieldExtractors_Extractors {
        ChronicleParserExtension_FieldExtractors_Extractors {
            destination_path: core::default::Default::default(),
            field_path: core::default::Default::default(),
            precondition_op: core::default::Default::default(),
            precondition_path: core::default::Default::default(),
            precondition_value: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParserExtension_FieldExtractors_ExtractorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParserExtension_FieldExtractors_ExtractorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleParserExtension_FieldExtractors_ExtractorsRef {
        ChronicleParserExtension_FieldExtractors_ExtractorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParserExtension_FieldExtractors_ExtractorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `destination_path` after provisioning.\nPath in generated event which is to be populated. This is required if the\nFieldExtractor is used to specify the parser extension."]
    pub fn destination_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `field_path` after provisioning.\nField path could be a json path, xml path or csv column name\ndepending on log format. It refers to a section or substring in raw log.\nThis is required if the FieldExtractor is used to specify the parser\nextension."]
    pub fn field_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `precondition_op` after provisioning.\nOperator used for precondition.\nPossible values:\nEQUALS\nNOT_EQUALS"]
    pub fn precondition_op(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precondition_op", self.base))
    }

    #[doc =
        "Get a reference to the value of field `precondition_path` after provisioning.\nPrecondition path could be a json path, xml path or csv column name\ndepending on log format. It refers to a section or substring in raw log."]
    pub fn precondition_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precondition_path", self.base))
    }

    #[doc = "Get a reference to the value of field `precondition_value` after provisioning.\nPrecondition value."]
    pub fn precondition_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.precondition_value", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nValue to be mapped to the destination path directly."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleParserExtension_FieldExtractors_PreprocessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grok_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleParserExtension_FieldExtractors_PreprocessConfig {
    #[doc =
        "Set the field `grok_regex`.\nGROK Regex to extract the structured part of the log.\nsyntax documentation:\nwww.elastic.co/guide/en/logstash/current/plugins-filters-grok.html"]
    pub fn set_grok_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.grok_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `target`.\nTarget field name for the structured part of the log.\nThis should match a SEMANTIC identifier from the grok expression."]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleParserExtension_FieldExtractors_PreprocessConfig {
    type O = terrars::BlockAssignable<ChronicleParserExtension_FieldExtractors_PreprocessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParserExtension_FieldExtractors_PreprocessConfig {}

impl BuildChronicleParserExtension_FieldExtractors_PreprocessConfig {
    pub fn build(self) -> ChronicleParserExtension_FieldExtractors_PreprocessConfig {
        ChronicleParserExtension_FieldExtractors_PreprocessConfig {
            grok_regex: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParserExtension_FieldExtractors_PreprocessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParserExtension_FieldExtractors_PreprocessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleParserExtension_FieldExtractors_PreprocessConfigRef {
        ChronicleParserExtension_FieldExtractors_PreprocessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParserExtension_FieldExtractors_PreprocessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `grok_regex` after provisioning.\nGROK Regex to extract the structured part of the log.\nsyntax documentation:\nwww.elastic.co/guide/en/logstash/current/plugins-filters-grok.html"]
    pub fn grok_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grok_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target` after provisioning.\nTarget field name for the structured part of the log.\nThis should match a SEMANTIC identifier from the grok expression."]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleParserExtension_FieldExtractorsDynamic {
    extractors: std::option::Option<terrars::DynamicBlock<ChronicleParserExtension_FieldExtractors_Extractors>>,
    preprocess_config: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleParserExtension_FieldExtractors_PreprocessConfig>>,
}

#[derive(Serialize)]
pub struct ChronicleParserExtension_FieldExtractors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    append_repeated_fields: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extractors: std::option::Option<std::vec::Vec<ChronicleParserExtension_FieldExtractors_Extractors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preprocess_config: std::option::Option<std::vec::Vec<ChronicleParserExtension_FieldExtractors_PreprocessConfig>>,
    dynamic: ChronicleParserExtension_FieldExtractorsDynamic,
}

impl ChronicleParserExtension_FieldExtractors {
    #[doc =
        "Set the field `append_repeated_fields`.\nWhether to append repeated fields or not.\nWhen false, repeated fields will be replaced."]
    pub fn set_append_repeated_fields(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.append_repeated_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_format`.\nPossible values:\nJSON\nCSV\nXML"]
    pub fn set_log_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extractors`.\n"]
    pub fn set_extractors(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleParserExtension_FieldExtractors_Extractors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.extractors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.extractors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `preprocess_config`.\n"]
    pub fn set_preprocess_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ChronicleParserExtension_FieldExtractors_PreprocessConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.preprocess_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.preprocess_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleParserExtension_FieldExtractors {
    type O = terrars::BlockAssignable<ChronicleParserExtension_FieldExtractors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParserExtension_FieldExtractors {}

impl BuildChronicleParserExtension_FieldExtractors {
    pub fn build(self) -> ChronicleParserExtension_FieldExtractors {
        ChronicleParserExtension_FieldExtractors {
            append_repeated_fields: core::default::Default::default(),
            log_format: core::default::Default::default(),
            extractors: core::default::Default::default(),
            preprocess_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParserExtension_FieldExtractorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParserExtension_FieldExtractorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleParserExtension_FieldExtractorsRef {
        ChronicleParserExtension_FieldExtractorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParserExtension_FieldExtractorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `append_repeated_fields` after provisioning.\nWhether to append repeated fields or not.\nWhen false, repeated fields will be replaced."]
    pub fn append_repeated_fields(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.append_repeated_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `log_format` after provisioning.\nPossible values:\nJSON\nCSV\nXML"]
    pub fn log_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `transformed_cbn_snippet` after provisioning.\nCBN snippet generated from field extractors."]
    pub fn transformed_cbn_snippet(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transformed_cbn_snippet", self.base))
    }

    #[doc = "Get a reference to the value of field `extractors` after provisioning.\n"]
    pub fn extractors(&self) -> terrars::ListRef<ChronicleParserExtension_FieldExtractors_ExtractorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extractors", self.base))
    }

    #[doc = "Get a reference to the value of field `preprocess_config` after provisioning.\n"]
    pub fn preprocess_config(&self) -> terrars::ListRef<ChronicleParserExtension_FieldExtractors_PreprocessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preprocess_config", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleParserExtension_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleParserExtension_Timeouts {
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
}

impl terrars::ToListMappable for ChronicleParserExtension_Timeouts {
    type O = terrars::BlockAssignable<ChronicleParserExtension_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParserExtension_Timeouts {}

impl BuildChronicleParserExtension_Timeouts {
    pub fn build(self) -> ChronicleParserExtension_Timeouts {
        ChronicleParserExtension_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParserExtension_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParserExtension_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleParserExtension_TimeoutsRef {
        ChronicleParserExtension_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParserExtension_TimeoutsRef {
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
}
