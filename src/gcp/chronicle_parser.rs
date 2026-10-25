#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleParserData_ {
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
    cbn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    logtype: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validated_on_empty_logs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_skipped: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    low_code: std::option::Option<std::vec::Vec<ChronicleParser_LowCode>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleParser_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_info: std::option::Option<std::vec::Vec<ChronicleParser_VersionInfo>>,
    dynamic: ChronicleParserDynamic,
}

struct ChronicleParser_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleParserData_>,
}

#[derive(Clone)]
pub struct ChronicleParser(std::rc::Rc<ChronicleParser_>);

impl ChronicleParser {
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
        "Set the field `cbn`.\nif the parser is built using config\ndocumentation:\nhttps://cloud.google.com/chronicle/docs/preview/parser-extensions/parsing-overview"]
    pub fn set_cbn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cbn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `validated_on_empty_logs`.\nFlag to bypass parser validation when no logs are found.\nIf enabled, the parser won't be be rejected during the validation\nphase when no logs are found."]
    pub fn set_validated_on_empty_logs(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().validated_on_empty_logs = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `validation_skipped`.\nIf true, bypasses parser validation.\nIf enabled, the parser won't be rejected during the validation\nphase and validation will be skipped."]
    pub fn set_validation_skipped(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().validation_skipped = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `low_code`.\n"]
    pub fn set_low_code(self, v: impl std::convert::Into<terrars::BlockAssignable<ChronicleParser_LowCode>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().low_code = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.low_code = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleParser_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version_info`.\n"]
    pub fn set_version_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleParser_VersionInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().version_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.version_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `cbn` after provisioning.\nif the parser is built using config\ndocumentation:\nhttps://cloud.google.com/chronicle/docs/preview/parser-extensions/parsing-overview"]
    pub fn cbn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cbn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `changelogs` after provisioning.\nChangelogs of a parser."]
    pub fn changelogs(&self) -> terrars::ListRef<ChronicleParser_ChangelogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.changelogs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime at which parser was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nInformation about the creator of the parser."]
    pub fn creator(&self) -> terrars::ListRef<ChronicleParser_CreatorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dynamic_parsing_config` after provisioning.\nDynamic parsing config applied over the parser, if any."]
    pub fn dynamic_parsing_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dynamic_parsing_config", self.extract_ref()))
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
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logtype` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn logtype(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logtype", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nname of the parser resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parser` after provisioning.\nOutput only. The server-generated ID of the parser."]
    pub fn parser(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parser", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parser_extension` after provisioning.\nExtension applied over the parser, if any."]
    pub fn parser_extension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parser_extension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `release_stage` after provisioning.\nThe release stage of the parser\nAfter internal validations the prebuilt parser will directly start as\nRelease Candidate. The release_stage of prebuilt parsers are changed\nafter every release cycle:\nThe prebuilt Release Candidate parser is promoted as Release parser.\nThe existing prebuilt Release parser is moved to Rollback state.\nand existing prebuilt rollback parser is moved to Archived.\nIn case of custom parser:\nWhen the customer submits a validation passed custom parser it starts as\nRelease state.\nAnd existing one is moved to Rollback stage.\nAnd the existing rollback is moved to Archived.\nIn case a release or release candidate parser is found faulty,\nthe parser is marked FAULTY,\nif it is release parser then rollback candidate is moved to release.\nPossible values:\nRELEASE\nRELEASE_CANDIDATE\nROLLBACK_CANDIDATE\nARCHIVED\nFAULTY\nARCHIVED_IN_USE"]
    pub fn release_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_stage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the parser\nPossible values:\nACTIVE\nINACTIVE"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the parser\nPossible values:\nCUSTOM\nPREBUILT"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validated_on_empty_logs` after provisioning.\nFlag to bypass parser validation when no logs are found.\nIf enabled, the parser won't be be rejected during the validation\nphase when no logs are found."]
    pub fn validated_on_empty_logs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validated_on_empty_logs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_report` after provisioning.\nThe Validation report generated during parser validation."]
    pub fn validation_report(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_report", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_skipped` after provisioning.\nIf true, bypasses parser validation.\nIf enabled, the parser won't be rejected during the validation\nphase and validation will be skipped."]
    pub fn validation_skipped(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_skipped", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_stage` after provisioning.\nThe validation stage of the parser\nWhen a customer submits a new parser for validation, it starts with a\nnew stage.\nWhen parser is picked for validation, it changes to Validation state.\nIf validation failed it is marked as failed, and\nexisting failed is moved to delete_candidate stage.\nIf passes it is moved to passed stage.\nIf customer opts to submit it, the parser is moved to Release State.\nPossible values:\nNEW\nVALIDATING\nPASSED\nFAILED\nDELETE_CANDIDATE\nINTERNAL_ERROR\nVALIDATION_SKIPPED"]
    pub fn validation_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_stage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `low_code` after provisioning.\n"]
    pub fn low_code(&self) -> terrars::ListRef<ChronicleParser_LowCodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.low_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleParser_TimeoutsRef {
        ChronicleParser_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_info` after provisioning.\n"]
    pub fn version_info(&self) -> terrars::ListRef<ChronicleParser_VersionInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version_info", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleParser {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleParser { }

impl terrars::ToListMappable for ChronicleParser {
    type O = terrars::ListRef<ChronicleParserRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleParser_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_parser".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleParser {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub logtype: terrars::PrimField<std::string::String>,
}

impl BuildChronicleParser {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleParser {
        let out = ChronicleParser(std::rc::Rc::new(ChronicleParser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleParserData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cbn: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                logtype: self.logtype,
                project: core::default::Default::default(),
                validated_on_empty_logs: core::default::Default::default(),
                validation_skipped: core::default::Default::default(),
                low_code: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                version_info: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleParserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleParserRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `cbn` after provisioning.\nif the parser is built using config\ndocumentation:\nhttps://cloud.google.com/chronicle/docs/preview/parser-extensions/parsing-overview"]
    pub fn cbn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cbn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `changelogs` after provisioning.\nChangelogs of a parser."]
    pub fn changelogs(&self) -> terrars::ListRef<ChronicleParser_ChangelogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.changelogs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime at which parser was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `creator` after provisioning.\nInformation about the creator of the parser."]
    pub fn creator(&self) -> terrars::ListRef<ChronicleParser_CreatorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.creator", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dynamic_parsing_config` after provisioning.\nDynamic parsing config applied over the parser, if any."]
    pub fn dynamic_parsing_config(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dynamic_parsing_config", self.extract_ref()))
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
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `logtype` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn logtype(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logtype", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nname of the parser resource."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parser` after provisioning.\nOutput only. The server-generated ID of the parser."]
    pub fn parser(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parser", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parser_extension` after provisioning.\nExtension applied over the parser, if any."]
    pub fn parser_extension(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parser_extension", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `release_stage` after provisioning.\nThe release stage of the parser\nAfter internal validations the prebuilt parser will directly start as\nRelease Candidate. The release_stage of prebuilt parsers are changed\nafter every release cycle:\nThe prebuilt Release Candidate parser is promoted as Release parser.\nThe existing prebuilt Release parser is moved to Rollback state.\nand existing prebuilt rollback parser is moved to Archived.\nIn case of custom parser:\nWhen the customer submits a validation passed custom parser it starts as\nRelease state.\nAnd existing one is moved to Rollback stage.\nAnd the existing rollback is moved to Archived.\nIn case a release or release candidate parser is found faulty,\nthe parser is marked FAULTY,\nif it is release parser then rollback candidate is moved to release.\nPossible values:\nRELEASE\nRELEASE_CANDIDATE\nROLLBACK_CANDIDATE\nARCHIVED\nFAULTY\nARCHIVED_IN_USE"]
    pub fn release_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_stage", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nThe state of the parser\nPossible values:\nACTIVE\nINACTIVE"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe type of the parser\nPossible values:\nCUSTOM\nPREBUILT"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validated_on_empty_logs` after provisioning.\nFlag to bypass parser validation when no logs are found.\nIf enabled, the parser won't be be rejected during the validation\nphase when no logs are found."]
    pub fn validated_on_empty_logs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validated_on_empty_logs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_report` after provisioning.\nThe Validation report generated during parser validation."]
    pub fn validation_report(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_report", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_skipped` after provisioning.\nIf true, bypasses parser validation.\nIf enabled, the parser won't be rejected during the validation\nphase and validation will be skipped."]
    pub fn validation_skipped(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_skipped", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `validation_stage` after provisioning.\nThe validation stage of the parser\nWhen a customer submits a new parser for validation, it starts with a\nnew stage.\nWhen parser is picked for validation, it changes to Validation state.\nIf validation failed it is marked as failed, and\nexisting failed is moved to delete_candidate stage.\nIf passes it is moved to passed stage.\nIf customer opts to submit it, the parser is moved to Release State.\nPossible values:\nNEW\nVALIDATING\nPASSED\nFAILED\nDELETE_CANDIDATE\nINTERNAL_ERROR\nVALIDATION_SKIPPED"]
    pub fn validation_stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_stage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `low_code` after provisioning.\n"]
    pub fn low_code(&self) -> terrars::ListRef<ChronicleParser_LowCodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.low_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleParser_TimeoutsRef {
        ChronicleParser_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version_info` after provisioning.\n"]
    pub fn version_info(&self) -> terrars::ListRef<ChronicleParser_VersionInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version_info", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleParserDynamic {
    low_code: std::option::Option<terrars::DynamicBlock<ChronicleParser_LowCode>>,
    version_info: std::option::Option<terrars::DynamicBlock<ChronicleParser_VersionInfo>>,
}

#[derive(Serialize)]
pub struct ChronicleParser_Changelogs_Entries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    change_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deleted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parser_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleParser_Changelogs_Entries {
    #[doc = "Set the field `change_message`.\n"]
    pub fn set_change_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.change_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_time`.\n"]
    pub fn set_create_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deleted`.\n"]
    pub fn set_deleted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.deleted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parser_version`.\n"]
    pub fn set_parser_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parser_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleParser_Changelogs_Entries {
    type O = terrars::BlockAssignable<ChronicleParser_Changelogs_Entries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParser_Changelogs_Entries {}

impl BuildChronicleParser_Changelogs_Entries {
    pub fn build(self) -> ChronicleParser_Changelogs_Entries {
        ChronicleParser_Changelogs_Entries {
            change_message: core::default::Default::default(),
            create_time: core::default::Default::default(),
            deleted: core::default::Default::default(),
            parser_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParser_Changelogs_EntriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParser_Changelogs_EntriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleParser_Changelogs_EntriesRef {
        ChronicleParser_Changelogs_EntriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParser_Changelogs_EntriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `change_message` after provisioning.\n"]
    pub fn change_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.change_message", self.base))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.base))
    }

    #[doc = "Get a reference to the value of field `deleted` after provisioning.\n"]
    pub fn deleted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deleted", self.base))
    }

    #[doc = "Get a reference to the value of field `parser_version` after provisioning.\n"]
    pub fn parser_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parser_version", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleParser_Changelogs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entries: std::option::Option<terrars::ListField<ChronicleParser_Changelogs_Entries>>,
}

impl ChronicleParser_Changelogs {
    #[doc = "Set the field `entries`.\n"]
    pub fn set_entries(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ChronicleParser_Changelogs_Entries>>,
    ) -> Self {
        self.entries = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleParser_Changelogs {
    type O = terrars::BlockAssignable<ChronicleParser_Changelogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParser_Changelogs {}

impl BuildChronicleParser_Changelogs {
    pub fn build(self) -> ChronicleParser_Changelogs {
        ChronicleParser_Changelogs { entries: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ChronicleParser_ChangelogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParser_ChangelogsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleParser_ChangelogsRef {
        ChronicleParser_ChangelogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParser_ChangelogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entries` after provisioning.\n"]
    pub fn entries(&self) -> terrars::ListRef<ChronicleParser_Changelogs_EntriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entries", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleParser_Creator {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    author: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleParser_Creator {
    #[doc = "Set the field `author`.\n"]
    pub fn set_author(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.author = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer`.\n"]
    pub fn set_customer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.customer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChronicleParser_Creator {
    type O = terrars::BlockAssignable<ChronicleParser_Creator>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParser_Creator {}

impl BuildChronicleParser_Creator {
    pub fn build(self) -> ChronicleParser_Creator {
        ChronicleParser_Creator {
            author: core::default::Default::default(),
            customer: core::default::Default::default(),
            source: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParser_CreatorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParser_CreatorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleParser_CreatorRef {
        ChronicleParser_CreatorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParser_CreatorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `author` after provisioning.\n"]
    pub fn author(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.author", self.base))
    }

    #[doc = "Get a reference to the value of field `customer` after provisioning.\n"]
    pub fn customer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleParser_LowCode_FieldExtractors_Extractors {
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

impl ChronicleParser_LowCode_FieldExtractors_Extractors {
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

impl terrars::ToListMappable for ChronicleParser_LowCode_FieldExtractors_Extractors {
    type O = terrars::BlockAssignable<ChronicleParser_LowCode_FieldExtractors_Extractors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParser_LowCode_FieldExtractors_Extractors {}

impl BuildChronicleParser_LowCode_FieldExtractors_Extractors {
    pub fn build(self) -> ChronicleParser_LowCode_FieldExtractors_Extractors {
        ChronicleParser_LowCode_FieldExtractors_Extractors {
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
pub struct ChronicleParser_LowCode_FieldExtractors_ExtractorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParser_LowCode_FieldExtractors_ExtractorsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleParser_LowCode_FieldExtractors_ExtractorsRef {
        ChronicleParser_LowCode_FieldExtractors_ExtractorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParser_LowCode_FieldExtractors_ExtractorsRef {
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
pub struct ChronicleParser_LowCode_FieldExtractors_PreprocessConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grok_regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleParser_LowCode_FieldExtractors_PreprocessConfig {
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

impl terrars::ToListMappable for ChronicleParser_LowCode_FieldExtractors_PreprocessConfig {
    type O = terrars::BlockAssignable<ChronicleParser_LowCode_FieldExtractors_PreprocessConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParser_LowCode_FieldExtractors_PreprocessConfig {}

impl BuildChronicleParser_LowCode_FieldExtractors_PreprocessConfig {
    pub fn build(self) -> ChronicleParser_LowCode_FieldExtractors_PreprocessConfig {
        ChronicleParser_LowCode_FieldExtractors_PreprocessConfig {
            grok_regex: core::default::Default::default(),
            target: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParser_LowCode_FieldExtractors_PreprocessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParser_LowCode_FieldExtractors_PreprocessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleParser_LowCode_FieldExtractors_PreprocessConfigRef {
        ChronicleParser_LowCode_FieldExtractors_PreprocessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParser_LowCode_FieldExtractors_PreprocessConfigRef {
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
struct ChronicleParser_LowCode_FieldExtractorsDynamic {
    extractors: std::option::Option<terrars::DynamicBlock<ChronicleParser_LowCode_FieldExtractors_Extractors>>,
    preprocess_config: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleParser_LowCode_FieldExtractors_PreprocessConfig>>,
}

#[derive(Serialize)]
pub struct ChronicleParser_LowCode_FieldExtractors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    append_repeated_fields: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extractors: std::option::Option<std::vec::Vec<ChronicleParser_LowCode_FieldExtractors_Extractors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preprocess_config: std::option::Option<std::vec::Vec<ChronicleParser_LowCode_FieldExtractors_PreprocessConfig>>,
    dynamic: ChronicleParser_LowCode_FieldExtractorsDynamic,
}

impl ChronicleParser_LowCode_FieldExtractors {
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
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleParser_LowCode_FieldExtractors_Extractors>>,
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
                    ::Into<terrars::BlockAssignable<ChronicleParser_LowCode_FieldExtractors_PreprocessConfig>>,
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

impl terrars::ToListMappable for ChronicleParser_LowCode_FieldExtractors {
    type O = terrars::BlockAssignable<ChronicleParser_LowCode_FieldExtractors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParser_LowCode_FieldExtractors {}

impl BuildChronicleParser_LowCode_FieldExtractors {
    pub fn build(self) -> ChronicleParser_LowCode_FieldExtractors {
        ChronicleParser_LowCode_FieldExtractors {
            append_repeated_fields: core::default::Default::default(),
            log_format: core::default::Default::default(),
            extractors: core::default::Default::default(),
            preprocess_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParser_LowCode_FieldExtractorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParser_LowCode_FieldExtractorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleParser_LowCode_FieldExtractorsRef {
        ChronicleParser_LowCode_FieldExtractorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParser_LowCode_FieldExtractorsRef {
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
    pub fn extractors(&self) -> terrars::ListRef<ChronicleParser_LowCode_FieldExtractors_ExtractorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extractors", self.base))
    }

    #[doc = "Get a reference to the value of field `preprocess_config` after provisioning.\n"]
    pub fn preprocess_config(&self) -> terrars::ListRef<ChronicleParser_LowCode_FieldExtractors_PreprocessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.preprocess_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct ChronicleParser_LowCodeDynamic {
    field_extractors: std::option::Option<terrars::DynamicBlock<ChronicleParser_LowCode_FieldExtractors>>,
}

#[derive(Serialize)]
pub struct ChronicleParser_LowCode {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_extractors: std::option::Option<std::vec::Vec<ChronicleParser_LowCode_FieldExtractors>>,
    dynamic: ChronicleParser_LowCodeDynamic,
}

impl ChronicleParser_LowCode {
    #[doc = "Set the field `log`.\nThe log used to create this low code parser in the UI."]
    pub fn set_log(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_extractors`.\n"]
    pub fn set_field_extractors(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleParser_LowCode_FieldExtractors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_extractors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_extractors = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ChronicleParser_LowCode {
    type O = terrars::BlockAssignable<ChronicleParser_LowCode>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParser_LowCode {}

impl BuildChronicleParser_LowCode {
    pub fn build(self) -> ChronicleParser_LowCode {
        ChronicleParser_LowCode {
            log: core::default::Default::default(),
            field_extractors: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParser_LowCodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParser_LowCodeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleParser_LowCodeRef {
        ChronicleParser_LowCodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParser_LowCodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `log` after provisioning.\nThe log used to create this low code parser in the UI."]
    pub fn log(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log", self.base))
    }

    #[doc = "Get a reference to the value of field `field_extractors` after provisioning.\n"]
    pub fn field_extractors(&self) -> terrars::ListRef<ChronicleParser_LowCode_FieldExtractorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_extractors", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleParser_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleParser_Timeouts {
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

impl terrars::ToListMappable for ChronicleParser_Timeouts {
    type O = terrars::BlockAssignable<ChronicleParser_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParser_Timeouts {}

impl BuildChronicleParser_Timeouts {
    pub fn build(self) -> ChronicleParser_Timeouts {
        ChronicleParser_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleParser_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParser_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleParser_TimeoutsRef {
        ChronicleParser_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParser_TimeoutsRef {
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
pub struct ChronicleParser_VersionInfo {
    auto_upgrade_disabled: terrars::PrimField<bool>,
}

impl ChronicleParser_VersionInfo { }

impl terrars::ToListMappable for ChronicleParser_VersionInfo {
    type O = terrars::BlockAssignable<ChronicleParser_VersionInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleParser_VersionInfo {
    #[doc =
        "Signifies if the parser is disabled for auto upgrade. If true, the parser\nwill not be upgraded by the auto upgrade process."]
    pub auto_upgrade_disabled: terrars::PrimField<bool>,
}

impl BuildChronicleParser_VersionInfo {
    pub fn build(self) -> ChronicleParser_VersionInfo {
        ChronicleParser_VersionInfo { auto_upgrade_disabled: self.auto_upgrade_disabled }
    }
}

#[allow(dead_code)]
pub struct ChronicleParser_VersionInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleParser_VersionInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleParser_VersionInfoRef {
        ChronicleParser_VersionInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleParser_VersionInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `auto_upgrade_disabled` after provisioning.\nSignifies if the parser is disabled for auto upgrade. If true, the parser\nwill not be upgraded by the auto upgrade process."]
    pub fn auto_upgrade_disabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_upgrade_disabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `latest_parser` after provisioning.\nThe resource name of latest Parser for this logtype.\nFormat:\nprojects/{project}/locations/{region}/instances/{instance}/logTypes/{log_type}/parsers/{parser}/{id}"]
    pub fn latest_parser(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_parser", self.base))
    }

    #[doc =
        "Get a reference to the value of field `latest_parser_version` after provisioning.\nThe version for the latest available stable version of the parser."]
    pub fn latest_parser_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_parser_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `rollback_available` after provisioning.\nSignifies if rollback is available for this parser version."]
    pub fn rollback_available(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollback_available", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\nThe version of the parser."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}
