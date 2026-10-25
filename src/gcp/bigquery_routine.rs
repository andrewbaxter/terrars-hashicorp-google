#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct BigqueryRoutineData_ {
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
    data_governance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    dataset_id: terrars::PrimField<std::string::String>,
    definition_body: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    determinism_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    imported_libraries: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_table_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    return_type: std::option::Option<terrars::PrimField<std::string::String>>,
    routine_id: terrars::PrimField<std::string::String>,
    routine_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arguments: std::option::Option<std::vec::Vec<BigqueryRoutine_Arguments>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    remote_function_options: std::option::Option<std::vec::Vec<BigqueryRoutine_RemoteFunctionOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark_options: std::option::Option<std::vec::Vec<BigqueryRoutine_SparkOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BigqueryRoutine_Timeouts>,
    dynamic: BigqueryRoutineDynamic,
}

struct BigqueryRoutine_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BigqueryRoutineData_>,
}

#[derive(Clone)]
pub struct BigqueryRoutine(std::rc::Rc<BigqueryRoutine_>);

impl BigqueryRoutine {
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
        "Set the field `data_governance_type`.\nIf set to DATA_MASKING, the function is validated and made available as a masking function. For more information, see https://cloud.google.com/bigquery/docs/user-defined-functions#custom-mask Possible values: [\"DATA_MASKING\"]"]
    pub fn set_data_governance_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data_governance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nThe description of the routine if defined."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `determinism_level`.\nThe determinism level of the JavaScript UDF if defined. Possible values: [\"DETERMINISM_LEVEL_UNSPECIFIED\", \"DETERMINISTIC\", \"NOT_DETERMINISTIC\"]"]
    pub fn set_determinism_level(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().determinism_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `imported_libraries`.\nOptional. If language = \"JAVASCRIPT\", this field stores the path of the\nimported JAVASCRIPT libraries."]
    pub fn set_imported_libraries(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().imported_libraries = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `language`.\nThe language of the routine. Possible values: [\"SQL\", \"JAVASCRIPT\", \"PYTHON\", \"JAVA\", \"SCALA\"]"]
    pub fn set_language(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().language = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `return_table_type`.\nOptional. Can be set only if routineType = \"TABLE_VALUED_FUNCTION\".\n\nIf absent, the return table type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the columns in the evaluated table result will\nbe cast to match the column types specificed in return table type, at query time."]
    pub fn set_return_table_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().return_table_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `return_type`.\nA JSON schema for the return type. Optional if language = \"SQL\"; required otherwise.\nIf absent, the return type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the evaluated result will be cast to\nthe specified returned type at query time. ~>**NOTE**: Because this field expects a JSON\nstring, any changes to the string will create a diff, even if the JSON itself hasn't\nchanged. If the API returns a different value for the same schema, e.g. it switche\nd the order of values or replaced STRUCT field type with RECORD field type, we currently\ncannot suppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn set_return_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().return_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `security_mode`.\nOptional. The security mode of the routine, if defined. If not defined, the security mode is automatically determined from the routine's configuration. Possible values: [\"DEFINER\", \"INVOKER\"]"]
    pub fn set_security_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().security_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `arguments`.\n"]
    pub fn set_arguments(self, v: impl std::convert::Into<terrars::BlockAssignable<BigqueryRoutine_Arguments>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().arguments = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.arguments = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remote_function_options`.\n"]
    pub fn set_remote_function_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryRoutine_RemoteFunctionOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remote_function_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remote_function_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark_options`.\n"]
    pub fn set_spark_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryRoutine_SparkOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spark_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spark_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BigqueryRoutine_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this routine was created, in milliseconds since the\nepoch."]
    pub fn creation_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_governance_type` after provisioning.\nIf set to DATA_MASKING, the function is validated and made available as a masking function. For more information, see https://cloud.google.com/bigquery/docs/user-defined-functions#custom-mask Possible values: [\"DATA_MASKING\"]"]
    pub fn data_governance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_governance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this routine"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `definition_body` after provisioning.\nThe body of the routine. For functions, this is the expression in the AS clause.\nIf language=SQL, it is the substring inside (but excluding) the parentheses."]
    pub fn definition_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.definition_body", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the routine if defined."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `determinism_level` after provisioning.\nThe determinism level of the JavaScript UDF if defined. Possible values: [\"DETERMINISM_LEVEL_UNSPECIFIED\", \"DETERMINISTIC\", \"NOT_DETERMINISTIC\"]"]
    pub fn determinism_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.determinism_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `imported_libraries` after provisioning.\nOptional. If language = \"JAVASCRIPT\", this field stores the path of the\nimported JAVASCRIPT libraries."]
    pub fn imported_libraries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.imported_libraries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `language` after provisioning.\nThe language of the routine. Possible values: [\"SQL\", \"JAVASCRIPT\", \"PYTHON\", \"JAVA\", \"SCALA\"]"]
    pub fn language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this routine was modified, in milliseconds since the\nepoch."]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `return_table_type` after provisioning.\nOptional. Can be set only if routineType = \"TABLE_VALUED_FUNCTION\".\n\nIf absent, the return table type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the columns in the evaluated table result will\nbe cast to match the column types specificed in return table type, at query time."]
    pub fn return_table_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_table_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `return_type` after provisioning.\nA JSON schema for the return type. Optional if language = \"SQL\"; required otherwise.\nIf absent, the return type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the evaluated result will be cast to\nthe specified returned type at query time. ~>**NOTE**: Because this field expects a JSON\nstring, any changes to the string will create a diff, even if the JSON itself hasn't\nchanged. If the API returns a different value for the same schema, e.g. it switche\nd the order of values or replaced STRUCT field type with RECORD field type, we currently\ncannot suppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn return_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `routine_id` after provisioning.\nThe ID of the the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters."]
    pub fn routine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routine_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `routine_type` after provisioning.\nThe type of routine. Possible values: [\"SCALAR_FUNCTION\", \"PROCEDURE\", \"TABLE_VALUED_FUNCTION\"]"]
    pub fn routine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routine_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_mode` after provisioning.\nOptional. The security mode of the routine, if defined. If not defined, the security mode is automatically determined from the routine's configuration. Possible values: [\"DEFINER\", \"INVOKER\"]"]
    pub fn security_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arguments` after provisioning.\n"]
    pub fn arguments(&self) -> terrars::ListRef<BigqueryRoutine_ArgumentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_function_options` after provisioning.\n"]
    pub fn remote_function_options(&self) -> terrars::ListRef<BigqueryRoutine_RemoteFunctionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_function_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_options` after provisioning.\n"]
    pub fn spark_options(&self) -> terrars::ListRef<BigqueryRoutine_SparkOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryRoutine_TimeoutsRef {
        BigqueryRoutine_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BigqueryRoutine {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BigqueryRoutine { }

impl terrars::ToListMappable for BigqueryRoutine {
    type O = terrars::ListRef<BigqueryRoutineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BigqueryRoutine_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_bigquery_routine".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBigqueryRoutine {
    pub tf_id: std::string::String,
    #[doc = "The ID of the dataset containing this routine"]
    pub dataset_id: terrars::PrimField<std::string::String>,
    #[doc =
        "The body of the routine. For functions, this is the expression in the AS clause.\nIf language=SQL, it is the substring inside (but excluding) the parentheses."]
    pub definition_body: terrars::PrimField<std::string::String>,
    #[doc =
        "The ID of the the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters."]
    pub routine_id: terrars::PrimField<std::string::String>,
    #[doc = "The type of routine. Possible values: [\"SCALAR_FUNCTION\", \"PROCEDURE\", \"TABLE_VALUED_FUNCTION\"]"]
    pub routine_type: terrars::PrimField<std::string::String>,
}

impl BuildBigqueryRoutine {
    pub fn build(self, stack: &mut terrars::Stack) -> BigqueryRoutine {
        let out = BigqueryRoutine(std::rc::Rc::new(BigqueryRoutine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BigqueryRoutineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                data_governance_type: core::default::Default::default(),
                dataset_id: self.dataset_id,
                definition_body: self.definition_body,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                determinism_level: core::default::Default::default(),
                id: core::default::Default::default(),
                imported_libraries: core::default::Default::default(),
                language: core::default::Default::default(),
                project: core::default::Default::default(),
                return_table_type: core::default::Default::default(),
                return_type: core::default::Default::default(),
                routine_id: self.routine_id,
                routine_type: self.routine_type,
                security_mode: core::default::Default::default(),
                arguments: core::default::Default::default(),
                remote_function_options: core::default::Default::default(),
                spark_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BigqueryRoutineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryRoutineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BigqueryRoutineRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `creation_time` after provisioning.\nThe time when this routine was created, in milliseconds since the\nepoch."]
    pub fn creation_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `data_governance_type` after provisioning.\nIf set to DATA_MASKING, the function is validated and made available as a masking function. For more information, see https://cloud.google.com/bigquery/docs/user-defined-functions#custom-mask Possible values: [\"DATA_MASKING\"]"]
    pub fn data_governance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_governance_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dataset_id` after provisioning.\nThe ID of the dataset containing this routine"]
    pub fn dataset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `definition_body` after provisioning.\nThe body of the routine. For functions, this is the expression in the AS clause.\nIf language=SQL, it is the substring inside (but excluding) the parentheses."]
    pub fn definition_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.definition_body", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nThe description of the routine if defined."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `determinism_level` after provisioning.\nThe determinism level of the JavaScript UDF if defined. Possible values: [\"DETERMINISM_LEVEL_UNSPECIFIED\", \"DETERMINISTIC\", \"NOT_DETERMINISTIC\"]"]
    pub fn determinism_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.determinism_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `imported_libraries` after provisioning.\nOptional. If language = \"JAVASCRIPT\", this field stores the path of the\nimported JAVASCRIPT libraries."]
    pub fn imported_libraries(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.imported_libraries", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `language` after provisioning.\nThe language of the routine. Possible values: [\"SQL\", \"JAVASCRIPT\", \"PYTHON\", \"JAVA\", \"SCALA\"]"]
    pub fn language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_time` after provisioning.\nThe time when this routine was modified, in milliseconds since the\nepoch."]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `return_table_type` after provisioning.\nOptional. Can be set only if routineType = \"TABLE_VALUED_FUNCTION\".\n\nIf absent, the return table type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the columns in the evaluated table result will\nbe cast to match the column types specificed in return table type, at query time."]
    pub fn return_table_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_table_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `return_type` after provisioning.\nA JSON schema for the return type. Optional if language = \"SQL\"; required otherwise.\nIf absent, the return type is inferred from definitionBody at query time in each query\nthat references this routine. If present, then the evaluated result will be cast to\nthe specified returned type at query time. ~>**NOTE**: Because this field expects a JSON\nstring, any changes to the string will create a diff, even if the JSON itself hasn't\nchanged. If the API returns a different value for the same schema, e.g. it switche\nd the order of values or replaced STRUCT field type with RECORD field type, we currently\ncannot suppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn return_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.return_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `routine_id` after provisioning.\nThe ID of the the routine. The ID must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum length is 256 characters."]
    pub fn routine_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routine_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `routine_type` after provisioning.\nThe type of routine. Possible values: [\"SCALAR_FUNCTION\", \"PROCEDURE\", \"TABLE_VALUED_FUNCTION\"]"]
    pub fn routine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routine_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `security_mode` after provisioning.\nOptional. The security mode of the routine, if defined. If not defined, the security mode is automatically determined from the routine's configuration. Possible values: [\"DEFINER\", \"INVOKER\"]"]
    pub fn security_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arguments` after provisioning.\n"]
    pub fn arguments(&self) -> terrars::ListRef<BigqueryRoutine_ArgumentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.arguments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_function_options` after provisioning.\n"]
    pub fn remote_function_options(&self) -> terrars::ListRef<BigqueryRoutine_RemoteFunctionOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.remote_function_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spark_options` after provisioning.\n"]
    pub fn spark_options(&self) -> terrars::ListRef<BigqueryRoutine_SparkOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BigqueryRoutine_TimeoutsRef {
        BigqueryRoutine_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BigqueryRoutineDynamic {
    arguments: std::option::Option<terrars::DynamicBlock<BigqueryRoutine_Arguments>>,
    remote_function_options: std::option::Option<terrars::DynamicBlock<BigqueryRoutine_RemoteFunctionOptions>>,
    spark_options: std::option::Option<terrars::DynamicBlock<BigqueryRoutine_SparkOptions>>,
}

#[derive(Serialize)]
pub struct BigqueryRoutine_Arguments_TableType_Columns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryRoutine_Arguments_TableType_Columns {
    #[doc = "Set the field `name`.\nThe name of the column."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `type_`.\nA JSON schema for the data type of the column. Required unless argumentKind = ANY_TYPE.\n~>**NOTE**: Because this field expects a JSON string, any changes to the string\nwill create a diff, even if the JSON itself hasn't changed. If the API returns\na different value for the same schema, e.g. it switched the order of values\nor replaced STRUCT field type with RECORD field type, we currently cannot\nsuppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryRoutine_Arguments_TableType_Columns {
    type O = terrars::BlockAssignable<BigqueryRoutine_Arguments_TableType_Columns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryRoutine_Arguments_TableType_Columns {}

impl BuildBigqueryRoutine_Arguments_TableType_Columns {
    pub fn build(self) -> BigqueryRoutine_Arguments_TableType_Columns {
        BigqueryRoutine_Arguments_TableType_Columns {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryRoutine_Arguments_TableType_ColumnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryRoutine_Arguments_TableType_ColumnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BigqueryRoutine_Arguments_TableType_ColumnsRef {
        BigqueryRoutine_Arguments_TableType_ColumnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryRoutine_Arguments_TableType_ColumnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the column."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nA JSON schema for the data type of the column. Required unless argumentKind = ANY_TYPE.\n~>**NOTE**: Because this field expects a JSON string, any changes to the string\nwill create a diff, even if the JSON itself hasn't changed. If the API returns\na different value for the same schema, e.g. it switched the order of values\nor replaced STRUCT field type with RECORD field type, we currently cannot\nsuppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryRoutine_Arguments_TableTypeDynamic {
    columns: std::option::Option<terrars::DynamicBlock<BigqueryRoutine_Arguments_TableType_Columns>>,
}

#[derive(Serialize)]
pub struct BigqueryRoutine_Arguments_TableType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    columns: std::option::Option<std::vec::Vec<BigqueryRoutine_Arguments_TableType_Columns>>,
    dynamic: BigqueryRoutine_Arguments_TableTypeDynamic,
}

impl BigqueryRoutine_Arguments_TableType {
    #[doc = "Set the field `columns`.\n"]
    pub fn set_columns(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryRoutine_Arguments_TableType_Columns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.columns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.columns = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryRoutine_Arguments_TableType {
    type O = terrars::BlockAssignable<BigqueryRoutine_Arguments_TableType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryRoutine_Arguments_TableType {}

impl BuildBigqueryRoutine_Arguments_TableType {
    pub fn build(self) -> BigqueryRoutine_Arguments_TableType {
        BigqueryRoutine_Arguments_TableType {
            columns: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryRoutine_Arguments_TableTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryRoutine_Arguments_TableTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryRoutine_Arguments_TableTypeRef {
        BigqueryRoutine_Arguments_TableTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryRoutine_Arguments_TableTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `columns` after provisioning.\n"]
    pub fn columns(&self) -> terrars::ListRef<BigqueryRoutine_Arguments_TableType_ColumnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.columns", self.base))
    }
}

#[derive(Serialize, Default)]
struct BigqueryRoutine_ArgumentsDynamic {
    table_type: std::option::Option<terrars::DynamicBlock<BigqueryRoutine_Arguments_TableType>>,
}

#[derive(Serialize)]
pub struct BigqueryRoutine_Arguments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    argument_kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_type: std::option::Option<std::vec::Vec<BigqueryRoutine_Arguments_TableType>>,
    dynamic: BigqueryRoutine_ArgumentsDynamic,
}

impl BigqueryRoutine_Arguments {
    #[doc =
        "Set the field `argument_kind`.\nDefaults to FIXED_TYPE. Default value: \"FIXED_TYPE\" Possible values: [\"FIXED_TYPE\", \"ANY_TYPE\", \"FIXED_TABLE\"]"]
    pub fn set_argument_kind(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.argument_kind = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `data_type`.\nA JSON schema for the data type. Required unless argumentKind = ANY_TYPE.\n~>**NOTE**: Because this field expects a JSON string, any changes to the string\nwill create a diff, even if the JSON itself hasn't changed. If the API returns\na different value for the same schema, e.g. it switched the order of values\nor replaced STRUCT field type with RECORD field type, we currently cannot\nsuppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn set_data_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `mode`.\nSpecifies whether the argument is input or output. Can be set for procedures only. Possible values: [\"IN\", \"OUT\", \"INOUT\"]"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nThe name of this argument. Can be absent for function return argument."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_type`.\n"]
    pub fn set_table_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BigqueryRoutine_Arguments_TableType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.table_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.table_type = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BigqueryRoutine_Arguments {
    type O = terrars::BlockAssignable<BigqueryRoutine_Arguments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryRoutine_Arguments {}

impl BuildBigqueryRoutine_Arguments {
    pub fn build(self) -> BigqueryRoutine_Arguments {
        BigqueryRoutine_Arguments {
            argument_kind: core::default::Default::default(),
            data_type: core::default::Default::default(),
            mode: core::default::Default::default(),
            name: core::default::Default::default(),
            table_type: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryRoutine_ArgumentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryRoutine_ArgumentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryRoutine_ArgumentsRef {
        BigqueryRoutine_ArgumentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryRoutine_ArgumentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `argument_kind` after provisioning.\nDefaults to FIXED_TYPE. Default value: \"FIXED_TYPE\" Possible values: [\"FIXED_TYPE\", \"ANY_TYPE\", \"FIXED_TABLE\"]"]
    pub fn argument_kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.argument_kind", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_type` after provisioning.\nA JSON schema for the data type. Required unless argumentKind = ANY_TYPE.\n~>**NOTE**: Because this field expects a JSON string, any changes to the string\nwill create a diff, even if the JSON itself hasn't changed. If the API returns\na different value for the same schema, e.g. it switched the order of values\nor replaced STRUCT field type with RECORD field type, we currently cannot\nsuppress the recurring diff this causes. As a workaround, we recommend using\nthe schema as returned by the API."]
    pub fn data_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nSpecifies whether the argument is input or output. Can be set for procedures only. Possible values: [\"IN\", \"OUT\", \"INOUT\"]"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of this argument. Can be absent for function return argument."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `table_type` after provisioning.\n"]
    pub fn table_type(&self) -> terrars::ListRef<BigqueryRoutine_Arguments_TableTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.table_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryRoutine_RemoteFunctionOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_batching_rows: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_defined_context: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl BigqueryRoutine_RemoteFunctionOptions {
    #[doc =
        "Set the field `connection`.\nFully qualified name of the user-provided connection object which holds\nthe authentication information to send requests to the remote service.\nFormat: \"projects/{projectId}/locations/{locationId}/connections/{connectionId}\""]
    pub fn set_connection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `endpoint`.\nEndpoint of the user-provided remote service, e.g.\n'https://us-east1-my_gcf_project.cloudfunctions.net/remote_add'"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_batching_rows`.\nMax number of rows in each batch sent to the remote service. If absent or if 0,\nBigQuery dynamically decides the number of rows in a batch."]
    pub fn set_max_batching_rows(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_batching_rows = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `user_defined_context`.\nUser-defined context as a set of key/value pairs, which will be sent as function\ninvocation context together with batched arguments in the requests to the remote\nservice. The total number of bytes of keys and values must be less than 8KB.\n\nAn object containing a list of \"key\": value pairs. Example:\n'{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn set_user_defined_context(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_defined_context = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryRoutine_RemoteFunctionOptions {
    type O = terrars::BlockAssignable<BigqueryRoutine_RemoteFunctionOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryRoutine_RemoteFunctionOptions {}

impl BuildBigqueryRoutine_RemoteFunctionOptions {
    pub fn build(self) -> BigqueryRoutine_RemoteFunctionOptions {
        BigqueryRoutine_RemoteFunctionOptions {
            connection: core::default::Default::default(),
            endpoint: core::default::Default::default(),
            max_batching_rows: core::default::Default::default(),
            user_defined_context: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryRoutine_RemoteFunctionOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryRoutine_RemoteFunctionOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryRoutine_RemoteFunctionOptionsRef {
        BigqueryRoutine_RemoteFunctionOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryRoutine_RemoteFunctionOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `connection` after provisioning.\nFully qualified name of the user-provided connection object which holds\nthe authentication information to send requests to the remote service.\nFormat: \"projects/{projectId}/locations/{locationId}/connections/{connectionId}\""]
    pub fn connection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection", self.base))
    }

    #[doc =
        "Get a reference to the value of field `endpoint` after provisioning.\nEndpoint of the user-provided remote service, e.g.\n'https://us-east1-my_gcf_project.cloudfunctions.net/remote_add'"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_batching_rows` after provisioning.\nMax number of rows in each batch sent to the remote service. If absent or if 0,\nBigQuery dynamically decides the number of rows in a batch."]
    pub fn max_batching_rows(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_batching_rows", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_defined_context` after provisioning.\nUser-defined context as a set of key/value pairs, which will be sent as function\ninvocation context together with batched arguments in the requests to the remote\nservice. The total number of bytes of keys and values must be less than 8KB.\n\nAn object containing a list of \"key\": value pairs. Example:\n'{ \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }'."]
    pub fn user_defined_context(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_defined_context", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryRoutine_SparkOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_file_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    py_file_uris: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryRoutine_SparkOptions {
    #[doc =
        "Set the field `archive_uris`.\nArchive files to be extracted into the working directory of each executor. For more information about Apache Spark, see Apache Spark."]
    pub fn set_archive_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.archive_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `connection`.\nFully qualified name of the user-provided Spark connection object.\nFormat: \"projects/{projectId}/locations/{locationId}/connections/{connectionId}\""]
    pub fn set_connection(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_image`.\nCustom container image for the runtime environment."]
    pub fn set_container_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_image = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `file_uris`.\nFiles to be placed in the working directory of each executor. For more information about Apache Spark, see Apache Spark."]
    pub fn set_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `jar_uris`.\nJARs to include on the driver and executor CLASSPATH. For more information about Apache Spark, see Apache Spark."]
    pub fn set_jar_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.jar_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_class`.\nThe fully qualified name of a class in jarUris, for example, com.example.wordcount.\nExactly one of mainClass and main_jar_uri field should be set for Java/Scala language type."]
    pub fn set_main_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_class = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `main_file_uri`.\nThe main file/jar URI of the Spark application.\nExactly one of the definitionBody field and the mainFileUri field must be set for Python.\nExactly one of mainClass and mainFileUri field should be set for Java/Scala language type."]
    pub fn set_main_file_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_file_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `properties`.\nConfiguration properties as a set of key/value pairs, which will be passed on to the Spark application.\nFor more information, see Apache Spark and the procedure option list.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `py_file_uris`.\nPython files to be placed on the PYTHONPATH for PySpark application. Supported file types: .py, .egg, and .zip. For more information about Apache Spark, see Apache Spark."]
    pub fn set_py_file_uris(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.py_file_uris = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `runtime_version`.\nRuntime version. If not specified, the default runtime version is used."]
    pub fn set_runtime_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.runtime_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BigqueryRoutine_SparkOptions {
    type O = terrars::BlockAssignable<BigqueryRoutine_SparkOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryRoutine_SparkOptions {}

impl BuildBigqueryRoutine_SparkOptions {
    pub fn build(self) -> BigqueryRoutine_SparkOptions {
        BigqueryRoutine_SparkOptions {
            archive_uris: core::default::Default::default(),
            connection: core::default::Default::default(),
            container_image: core::default::Default::default(),
            file_uris: core::default::Default::default(),
            jar_uris: core::default::Default::default(),
            main_class: core::default::Default::default(),
            main_file_uri: core::default::Default::default(),
            properties: core::default::Default::default(),
            py_file_uris: core::default::Default::default(),
            runtime_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryRoutine_SparkOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryRoutine_SparkOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryRoutine_SparkOptionsRef {
        BigqueryRoutine_SparkOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryRoutine_SparkOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `archive_uris` after provisioning.\nArchive files to be extracted into the working directory of each executor. For more information about Apache Spark, see Apache Spark."]
    pub fn archive_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `connection` after provisioning.\nFully qualified name of the user-provided Spark connection object.\nFormat: \"projects/{projectId}/locations/{locationId}/connections/{connectionId}\""]
    pub fn connection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection", self.base))
    }

    #[doc =
        "Get a reference to the value of field `container_image` after provisioning.\nCustom container image for the runtime environment."]
    pub fn container_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_image", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_uris` after provisioning.\nFiles to be placed in the working directory of each executor. For more information about Apache Spark, see Apache Spark."]
    pub fn file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `jar_uris` after provisioning.\nJARs to include on the driver and executor CLASSPATH. For more information about Apache Spark, see Apache Spark."]
    pub fn jar_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jar_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_class` after provisioning.\nThe fully qualified name of a class in jarUris, for example, com.example.wordcount.\nExactly one of mainClass and main_jar_uri field should be set for Java/Scala language type."]
    pub fn main_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }

    #[doc =
        "Get a reference to the value of field `main_file_uri` after provisioning.\nThe main file/jar URI of the Spark application.\nExactly one of the definitionBody field and the mainFileUri field must be set for Python.\nExactly one of mainClass and mainFileUri field should be set for Java/Scala language type."]
    pub fn main_file_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_file_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `properties` after provisioning.\nConfiguration properties as a set of key/value pairs, which will be passed on to the Spark application.\nFor more information, see Apache Spark and the procedure option list.\nAn object containing a list of \"key\": value pairs. Example: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }."]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc =
        "Get a reference to the value of field `py_file_uris` after provisioning.\nPython files to be placed on the PYTHONPATH for PySpark application. Supported file types: .py, .egg, and .zip. For more information about Apache Spark, see Apache Spark."]
    pub fn py_file_uris(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.py_file_uris", self.base))
    }

    #[doc =
        "Get a reference to the value of field `runtime_version` after provisioning.\nRuntime version. If not specified, the default runtime version is used."]
    pub fn runtime_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_version", self.base))
    }
}

#[derive(Serialize)]
pub struct BigqueryRoutine_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BigqueryRoutine_Timeouts {
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

impl terrars::ToListMappable for BigqueryRoutine_Timeouts {
    type O = terrars::BlockAssignable<BigqueryRoutine_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBigqueryRoutine_Timeouts {}

impl BuildBigqueryRoutine_Timeouts {
    pub fn build(self) -> BigqueryRoutine_Timeouts {
        BigqueryRoutine_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BigqueryRoutine_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BigqueryRoutine_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BigqueryRoutine_TimeoutsRef {
        BigqueryRoutine_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BigqueryRoutine_TimeoutsRef {
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
