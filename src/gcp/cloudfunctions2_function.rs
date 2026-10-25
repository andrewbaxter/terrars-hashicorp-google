#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct Cloudfunctions2FunctionData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_config: std::option::Option<std::vec::Vec<Cloudfunctions2Function_BuildConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_trigger: std::option::Option<std::vec::Vec<Cloudfunctions2Function_EventTrigger>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_config: std::option::Option<std::vec::Vec<Cloudfunctions2Function_ServiceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Cloudfunctions2Function_Timeouts>,
    dynamic: Cloudfunctions2FunctionDynamic,
}

struct Cloudfunctions2Function_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Cloudfunctions2FunctionData_>,
}

#[derive(Clone)]
pub struct Cloudfunctions2Function(std::rc::Rc<Cloudfunctions2Function_>);

impl Cloudfunctions2Function {
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

    #[doc = "Set the field `description`.\nUser-provided description of a function."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `kms_key_name`.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources.\nIt must match the pattern projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}."]
    pub fn set_kms_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nA set of key/value label pairs associated with this Cloud Function.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_config`.\n"]
    pub fn set_build_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().build_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.build_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_trigger`.\n"]
    pub fn set_event_trigger(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudfunctions2Function_EventTrigger>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event_trigger = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event_trigger = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_config`.\n"]
    pub fn set_service_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudfunctions2Function_ServiceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Cloudfunctions2Function_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description of a function."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nThe environment the function is hosted on."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources.\nIt must match the pattern projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs associated with this Cloud Function.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of this cloud function."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the function. Function names must\nbe unique globally and match pattern 'projects/*/locations/*/functions/*'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nDescribes the current state of the function."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a Cloud Function."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nOutput only. The deployed url for the function."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_config` after provisioning.\n"]
    pub fn build_config(&self) -> terrars::ListRef<Cloudfunctions2Function_BuildConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_trigger` after provisioning.\n"]
    pub fn event_trigger(&self) -> terrars::ListRef<Cloudfunctions2Function_EventTriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_trigger", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_config` after provisioning.\n"]
    pub fn service_config(&self) -> terrars::ListRef<Cloudfunctions2Function_ServiceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Cloudfunctions2Function_TimeoutsRef {
        Cloudfunctions2Function_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Cloudfunctions2Function {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Cloudfunctions2Function { }

impl terrars::ToListMappable for Cloudfunctions2Function {
    type O = terrars::ListRef<Cloudfunctions2FunctionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Cloudfunctions2Function_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloudfunctions2_function".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfunctions2Function {
    pub tf_id: std::string::String,
    #[doc = "The location of this cloud function."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "A user-defined name of the function. Function names must\nbe unique globally and match pattern 'projects/*/locations/*/functions/*'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctions2Function {
    pub fn build(self, stack: &mut terrars::Stack) -> Cloudfunctions2Function {
        let out = Cloudfunctions2Function(std::rc::Rc::new(Cloudfunctions2Function_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Cloudfunctions2FunctionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_name: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                build_config: core::default::Default::default(),
                event_trigger: core::default::Default::default(),
                service_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Cloudfunctions2FunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2FunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Cloudfunctions2FunctionRef {
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

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-provided description of a function."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `environment` after provisioning.\nThe environment the function is hosted on."]
    pub fn environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_name` after provisioning.\nResource name of a KMS crypto key (managed by the user) used to encrypt/decrypt function resources.\nIt must match the pattern projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}."]
    pub fn kms_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nA set of key/value label pairs associated with this Cloud Function.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of this cloud function."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA user-defined name of the function. Function names must\nbe unique globally and match pattern 'projects/*/locations/*/functions/*'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nDescribes the current state of the function."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nThe last update timestamp of a Cloud Function."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nOutput only. The deployed url for the function."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_config` after provisioning.\n"]
    pub fn build_config(&self) -> terrars::ListRef<Cloudfunctions2Function_BuildConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.build_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event_trigger` after provisioning.\n"]
    pub fn event_trigger(&self) -> terrars::ListRef<Cloudfunctions2Function_EventTriggerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_trigger", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_config` after provisioning.\n"]
    pub fn service_config(&self) -> terrars::ListRef<Cloudfunctions2Function_ServiceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Cloudfunctions2Function_TimeoutsRef {
        Cloudfunctions2Function_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Cloudfunctions2FunctionDynamic {
    build_config: std::option::Option<terrars::DynamicBlock<Cloudfunctions2Function_BuildConfig>>,
    event_trigger: std::option::Option<terrars::DynamicBlock<Cloudfunctions2Function_EventTrigger>>,
    service_config: std::option::Option<terrars::DynamicBlock<Cloudfunctions2Function_ServiceConfig>>,
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {}

impl Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy { }

impl terrars::ToListMappable for Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {}

impl BuildCloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {
    pub fn build(self) -> Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {
        Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy {}
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef {
        Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {}

impl Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy { }

impl terrars::ToListMappable for Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {}

impl BuildCloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {
    pub fn build(self) -> Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {
        Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy {}
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef {
        Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `runtime_version` after provisioning.\nThe runtime version which was used during latest function deployment."]
    pub fn runtime_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_version", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_BuildConfig_Source_RepoSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    branch_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commit_sha: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert_regex: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Cloudfunctions2Function_BuildConfig_Source_RepoSource {
    #[doc = "Set the field `branch_name`.\nRegex matching branches to build."]
    pub fn set_branch_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.branch_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `commit_sha`.\nRegex matching tags to build."]
    pub fn set_commit_sha(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.commit_sha = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dir`.\nDirectory, relative to the source root, in which to run the build."]
    pub fn set_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dir = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `invert_regex`.\nOnly trigger a build if the revision regex does\nNOT match the revision regex."]
    pub fn set_invert_regex(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert_regex = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `project_id`.\nID of the project that owns the Cloud Source Repository. If omitted, the\nproject ID requesting the build is assumed."]
    pub fn set_project_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.project_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `repo_name`.\nName of the Cloud Source Repository."]
    pub fn set_repo_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.repo_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag_name`.\nRegex matching tags to build."]
    pub fn set_tag_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Cloudfunctions2Function_BuildConfig_Source_RepoSource {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig_Source_RepoSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_BuildConfig_Source_RepoSource {}

impl BuildCloudfunctions2Function_BuildConfig_Source_RepoSource {
    pub fn build(self) -> Cloudfunctions2Function_BuildConfig_Source_RepoSource {
        Cloudfunctions2Function_BuildConfig_Source_RepoSource {
            branch_name: core::default::Default::default(),
            commit_sha: core::default::Default::default(),
            dir: core::default::Default::default(),
            invert_regex: core::default::Default::default(),
            project_id: core::default::Default::default(),
            repo_name: core::default::Default::default(),
            tag_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_BuildConfig_Source_RepoSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_BuildConfig_Source_RepoSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudfunctions2Function_BuildConfig_Source_RepoSourceRef {
        Cloudfunctions2Function_BuildConfig_Source_RepoSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_BuildConfig_Source_RepoSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `branch_name` after provisioning.\nRegex matching branches to build."]
    pub fn branch_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc = "Get a reference to the value of field `commit_sha` after provisioning.\nRegex matching tags to build."]
    pub fn commit_sha(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commit_sha", self.base))
    }

    #[doc =
        "Get a reference to the value of field `dir` after provisioning.\nDirectory, relative to the source root, in which to run the build."]
    pub fn dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dir", self.base))
    }

    #[doc =
        "Get a reference to the value of field `invert_regex` after provisioning.\nOnly trigger a build if the revision regex does\nNOT match the revision regex."]
    pub fn invert_regex(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert_regex", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nID of the project that owns the Cloud Source Repository. If omitted, the\nproject ID requesting the build is assumed."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo_name` after provisioning.\nName of the Cloud Source Repository."]
    pub fn repo_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_name` after provisioning.\nRegex matching tags to build."]
    pub fn tag_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_name", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_BuildConfig_Source_StorageSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Cloudfunctions2Function_BuildConfig_Source_StorageSource {
    #[doc = "Set the field `bucket`.\nGoogle Cloud Storage bucket containing the source"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `generation`.\nGoogle Cloud Storage generation for the object. If the generation\nis omitted, the latest generation will be used."]
    pub fn set_generation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.generation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object`.\nGoogle Cloud Storage object containing the source."]
    pub fn set_object(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Cloudfunctions2Function_BuildConfig_Source_StorageSource {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig_Source_StorageSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_BuildConfig_Source_StorageSource {}

impl BuildCloudfunctions2Function_BuildConfig_Source_StorageSource {
    pub fn build(self) -> Cloudfunctions2Function_BuildConfig_Source_StorageSource {
        Cloudfunctions2Function_BuildConfig_Source_StorageSource {
            bucket: core::default::Default::default(),
            generation: core::default::Default::default(),
            object: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_BuildConfig_Source_StorageSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_BuildConfig_Source_StorageSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudfunctions2Function_BuildConfig_Source_StorageSourceRef {
        Cloudfunctions2Function_BuildConfig_Source_StorageSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_BuildConfig_Source_StorageSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket` after provisioning.\nGoogle Cloud Storage bucket containing the source"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nGoogle Cloud Storage generation for the object. If the generation\nis omitted, the latest generation will be used."]
    pub fn generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `object` after provisioning.\nGoogle Cloud Storage object containing the source."]
    pub fn object(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudfunctions2Function_BuildConfig_SourceDynamic {
    repo_source: std::option::Option<terrars::DynamicBlock<Cloudfunctions2Function_BuildConfig_Source_RepoSource>>,
    storage_source: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudfunctions2Function_BuildConfig_Source_StorageSource>>,
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_BuildConfig_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repo_source: std::option::Option<std::vec::Vec<Cloudfunctions2Function_BuildConfig_Source_RepoSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_source: std::option::Option<std::vec::Vec<Cloudfunctions2Function_BuildConfig_Source_StorageSource>>,
    dynamic: Cloudfunctions2Function_BuildConfig_SourceDynamic,
}

impl Cloudfunctions2Function_BuildConfig_Source {
    #[doc = "Set the field `repo_source`.\n"]
    pub fn set_repo_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig_Source_RepoSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.repo_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.repo_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_source`.\n"]
    pub fn set_storage_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig_Source_StorageSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Cloudfunctions2Function_BuildConfig_Source {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_BuildConfig_Source {}

impl BuildCloudfunctions2Function_BuildConfig_Source {
    pub fn build(self) -> Cloudfunctions2Function_BuildConfig_Source {
        Cloudfunctions2Function_BuildConfig_Source {
            repo_source: core::default::Default::default(),
            storage_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_BuildConfig_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_BuildConfig_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudfunctions2Function_BuildConfig_SourceRef {
        Cloudfunctions2Function_BuildConfig_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_BuildConfig_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `repo_source` after provisioning.\n"]
    pub fn repo_source(&self) -> terrars::ListRef<Cloudfunctions2Function_BuildConfig_Source_RepoSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repo_source", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_source` after provisioning.\n"]
    pub fn storage_source(&self) -> terrars::ListRef<Cloudfunctions2Function_BuildConfig_Source_StorageSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_source", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudfunctions2Function_BuildConfigDynamic {
    automatic_update_policy: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy>>,
    on_deploy_update_policy: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy>>,
    source: std::option::Option<terrars::DynamicBlock<Cloudfunctions2Function_BuildConfig_Source>>,
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_BuildConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_repository: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entry_point: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    automatic_update_policy: std
    ::option
    ::Option<std::vec::Vec<Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_deploy_update_policy: std::option::Option<std::vec::Vec<Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<Cloudfunctions2Function_BuildConfig_Source>>,
    dynamic: Cloudfunctions2Function_BuildConfigDynamic,
}

impl Cloudfunctions2Function_BuildConfig {
    #[doc =
        "Set the field `docker_repository`.\nUser managed repository created in Artifact Registry optionally with a customer managed encryption key."]
    pub fn set_docker_repository(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.docker_repository = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `entry_point`.\nThe name of the function (as defined in source code) that will be executed.\nDefaults to the resource name suffix, if not specified. For backward\ncompatibility, if function with given name is not found, then the system\nwill try to use function named \"function\". For Node.js this is name of a\nfunction exported by the module specified in source_location."]
    pub fn set_entry_point(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.entry_point = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment_variables`.\nUser-provided build-time environment variables for the function."]
    pub fn set_environment_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `runtime`.\nThe runtime in which to run the function. Required when deploying a new\nfunction, optional when updating an existing function."]
    pub fn set_runtime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.runtime = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account`.\nThe fully-qualified name of the service account to be used for building the container."]
    pub fn set_service_account(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_account = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `worker_pool`.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn set_worker_pool(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.worker_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `automatic_update_policy`.\n"]
    pub fn set_automatic_update_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.automatic_update_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.automatic_update_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `on_deploy_update_policy`.\n"]
    pub fn set_on_deploy_update_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_deploy_update_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_deploy_update_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig_Source>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Cloudfunctions2Function_BuildConfig {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_BuildConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_BuildConfig {}

impl BuildCloudfunctions2Function_BuildConfig {
    pub fn build(self) -> Cloudfunctions2Function_BuildConfig {
        Cloudfunctions2Function_BuildConfig {
            docker_repository: core::default::Default::default(),
            entry_point: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            runtime: core::default::Default::default(),
            service_account: core::default::Default::default(),
            worker_pool: core::default::Default::default(),
            automatic_update_policy: core::default::Default::default(),
            on_deploy_update_policy: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_BuildConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_BuildConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudfunctions2Function_BuildConfigRef {
        Cloudfunctions2Function_BuildConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_BuildConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `build` after provisioning.\nThe Cloud Build name of the latest successful\ndeployment of the function."]
    pub fn build(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build", self.base))
    }

    #[doc =
        "Get a reference to the value of field `docker_repository` after provisioning.\nUser managed repository created in Artifact Registry optionally with a customer managed encryption key."]
    pub fn docker_repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.docker_repository", self.base))
    }

    #[doc =
        "Get a reference to the value of field `entry_point` after provisioning.\nThe name of the function (as defined in source code) that will be executed.\nDefaults to the resource name suffix, if not specified. For backward\ncompatibility, if function with given name is not found, then the system\nwill try to use function named \"function\". For Node.js this is name of a\nfunction exported by the module specified in source_location."]
    pub fn entry_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entry_point", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_variables` after provisioning.\nUser-provided build-time environment variables for the function."]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.base))
    }

    #[doc =
        "Get a reference to the value of field `runtime` after provisioning.\nThe runtime in which to run the function. Required when deploying a new\nfunction, optional when updating an existing function."]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account` after provisioning.\nThe fully-qualified name of the service account to be used for building the container."]
    pub fn service_account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account", self.base))
    }

    #[doc =
        "Get a reference to the value of field `worker_pool` after provisioning.\nName of the Cloud Build Custom Worker Pool that should be used to build the function."]
    pub fn worker_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `automatic_update_policy` after provisioning.\n"]
    pub fn automatic_update_policy(
        &self,
    ) -> terrars::ListRef<Cloudfunctions2Function_BuildConfig_AutomaticUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.automatic_update_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `on_deploy_update_policy` after provisioning.\n"]
    pub fn on_deploy_update_policy(
        &self,
    ) -> terrars::ListRef<Cloudfunctions2Function_BuildConfig_OnDeployUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_deploy_update_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<Cloudfunctions2Function_BuildConfig_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_EventTrigger_EventFilters {
    attribute: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
    value: terrars::PrimField<std::string::String>,
}

impl Cloudfunctions2Function_EventTrigger_EventFilters {
    #[doc =
        "Set the field `operator`.\nOptional. The operator used for matching the events with the value of\nthe filter. If not specified, only events that have an exact key-value\npair specified in the filter are matched.\nThe only allowed value is 'match-path-pattern'.\n[See documentation on path patterns here](https://cloud.google.com/eventarc/docs/path-patterns)'"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Cloudfunctions2Function_EventTrigger_EventFilters {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_EventTrigger_EventFilters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_EventTrigger_EventFilters {
    #[doc =
        "'Required. The name of a CloudEvents attribute.\nCurrently, only a subset of attributes are supported for filtering. Use the 'gcloud eventarc providers describe' command to learn more about events and their attributes.\nDo not filter for the 'type' attribute here, as this is already achieved by the resource's 'event_type' attribute."]
    pub attribute: terrars::PrimField<std::string::String>,
    #[doc =
        "Required. The value for the attribute.\nIf the operator field is set as 'match-path-pattern', this value can be a path pattern instead of an exact value."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctions2Function_EventTrigger_EventFilters {
    pub fn build(self) -> Cloudfunctions2Function_EventTrigger_EventFilters {
        Cloudfunctions2Function_EventTrigger_EventFilters {
            attribute: self.attribute,
            operator: core::default::Default::default(),
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_EventTrigger_EventFiltersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_EventTrigger_EventFiltersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudfunctions2Function_EventTrigger_EventFiltersRef {
        Cloudfunctions2Function_EventTrigger_EventFiltersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_EventTrigger_EventFiltersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `attribute` after provisioning.\n'Required. The name of a CloudEvents attribute.\nCurrently, only a subset of attributes are supported for filtering. Use the 'gcloud eventarc providers describe' command to learn more about events and their attributes.\nDo not filter for the 'type' attribute here, as this is already achieved by the resource's 'event_type' attribute."]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc =
        "Get a reference to the value of field `operator` after provisioning.\nOptional. The operator used for matching the events with the value of\nthe filter. If not specified, only events that have an exact key-value\npair specified in the filter are matched.\nThe only allowed value is 'match-path-pattern'.\n[See documentation on path patterns here](https://cloud.google.com/eventarc/docs/path-patterns)'"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc =
        "Get a reference to the value of field `value` after provisioning.\nRequired. The value for the attribute.\nIf the operator field is set as 'match-path-pattern', this value can be a path pattern instead of an exact value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudfunctions2Function_EventTriggerDynamic {
    event_filters: std::option::Option<terrars::DynamicBlock<Cloudfunctions2Function_EventTrigger_EventFilters>>,
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_EventTrigger {
    event_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pubsub_topic: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_filters: std::option::Option<std::vec::Vec<Cloudfunctions2Function_EventTrigger_EventFilters>>,
    dynamic: Cloudfunctions2Function_EventTriggerDynamic,
}

impl Cloudfunctions2Function_EventTrigger {
    #[doc =
        "Set the field `pubsub_topic`.\nThe name of a Pub/Sub topic in the same project that will be used\nas the transport topic for the event delivery."]
    pub fn set_pubsub_topic(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pubsub_topic = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retry_policy`.\nDescribes the retry policy in case of function's execution failure.\nRetried execution is charged as any other execution. Possible values: [\"RETRY_POLICY_UNSPECIFIED\", \"RETRY_POLICY_DO_NOT_RETRY\", \"RETRY_POLICY_RETRY\"]"]
    pub fn set_retry_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.retry_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_email`.\nOptional. The email of the trigger's service account. The service account\nmust have permission to invoke Cloud Run services. If empty, defaults to the\nCompute Engine default service account: {project_number}-compute@developer.gserviceaccount.com."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `trigger_region`.\nThe region that the trigger will be in. The trigger will only receive\nevents originating in this region. It can be the same\nregion as the function, a different region or multi-region, or the global\nregion. If not provided, defaults to the same region as the function."]
    pub fn set_trigger_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.trigger_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event_filters`.\n"]
    pub fn set_event_filters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudfunctions2Function_EventTrigger_EventFilters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.event_filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.event_filters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Cloudfunctions2Function_EventTrigger {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_EventTrigger>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_EventTrigger {
    #[doc = "Required. The type of event to observe."]
    pub event_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctions2Function_EventTrigger {
    pub fn build(self) -> Cloudfunctions2Function_EventTrigger {
        Cloudfunctions2Function_EventTrigger {
            event_type: self.event_type,
            pubsub_topic: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            trigger_region: core::default::Default::default(),
            event_filters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_EventTriggerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_EventTriggerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudfunctions2Function_EventTriggerRef {
        Cloudfunctions2Function_EventTriggerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_EventTriggerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `event_type` after provisioning.\nRequired. The type of event to observe."]
    pub fn event_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe name of a Pub/Sub topic in the same project that will be used\nas the transport topic for the event delivery."]
    pub fn pubsub_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retry_policy` after provisioning.\nDescribes the retry policy in case of function's execution failure.\nRetried execution is charged as any other execution. Possible values: [\"RETRY_POLICY_UNSPECIFIED\", \"RETRY_POLICY_DO_NOT_RETRY\", \"RETRY_POLICY_RETRY\"]"]
    pub fn retry_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nOptional. The email of the trigger's service account. The service account\nmust have permission to invoke Cloud Run services. If empty, defaults to the\nCompute Engine default service account: {project_number}-compute@developer.gserviceaccount.com."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trigger` after provisioning.\nOutput only. The resource name of the Eventarc trigger."]
    pub fn trigger(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger", self.base))
    }

    #[doc =
        "Get a reference to the value of field `trigger_region` after provisioning.\nThe region that the trigger will be in. The trigger will only receive\nevents originating in this region. It can be the same\nregion as the function, a different region or multi-region, or the global\nregion. If not provided, defaults to the same region as the function."]
    pub fn trigger_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_region", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnetwork: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
    #[doc =
        "Set the field `network`.\nThe name of the VPC network to which the function will be connected. Specify either a VPC network or a subnet, or both. If you specify only a network, the subnet uses the same name as the network."]
    pub fn set_network(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.network = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `subnetwork`.\nThe name of the VPC subnetwork that the Cloud Function resource will get IPs from. Specify either a VPC network or a subnet, or both. If both network and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If subnetwork is not specified, the subnetwork with the same name with the network will be used."]
    pub fn set_subnetwork(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnetwork = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\nNetwork tags applied to this Cloud Function resource."]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {}

impl BuildCloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
    pub fn build(self) -> Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
        Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface {
            network: core::default::Default::default(),
            subnetwork: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef {
        Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `network` after provisioning.\nThe name of the VPC network to which the function will be connected. Specify either a VPC network or a subnet, or both. If you specify only a network, the subnet uses the same name as the network."]
    pub fn network(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }

    #[doc =
        "Get a reference to the value of field `subnetwork` after provisioning.\nThe name of the VPC subnetwork that the Cloud Function resource will get IPs from. Specify either a VPC network or a subnet, or both. If both network and subnetwork are specified, the given VPC subnetwork must belong to the given VPC network. If subnetwork is not specified, the subnetwork with the same name with the network will be used."]
    pub fn subnetwork(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnetwork", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tags` after provisioning.\nNetwork tags applied to this Cloud Function resource."]
    pub fn tags(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
    key: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    secret: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables { }

impl terrars::ToListMappable for Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
    #[doc = "Name of the environment variable."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc =
        "Project identifier (preferably project number but can also be the project ID) of the project that contains the secret. If not set, it will be populated with the function's project assuming that the secret exists in the same project as of the function."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the secret in secret manager (not the full resource name)."]
    pub secret: terrars::PrimField<std::string::String>,
    #[doc =
        "Version of the secret (version number or the string 'latest'). It is recommended to use a numeric version for secret environment variables as any updates to the secret value is not reflected until new instances start."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
    pub fn build(self) -> Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
        Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables {
            key: self.key,
            project_id: self.project_id,
            secret: self.secret,
            version: self.version,
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef {
        Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\nName of the environment variable."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nProject identifier (preferably project number but can also be the project ID) of the project that contains the secret. If not set, it will be populated with the function's project assuming that the secret exists in the same project as of the function."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nName of the secret in secret manager (not the full resource name)."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the secret (version number or the string 'latest'). It is recommended to use a numeric version for secret environment variables as any updates to the secret value is not reflected until new instances start."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
    path: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
}

impl Cloudfunctions2Function_ServiceConfig_SecretVolumes_Versions { }

impl terrars::ToListMappable for Cloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_ServiceConfig_SecretVolumes_Versions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
    #[doc =
        "Relative path of the file under the mount path where the secret value for this version will be fetched and made available. For example, setting the mountPath as '/etc/secrets' and path as secret_foo would mount the secret value file at /etc/secrets/secret_foo."]
    pub path: terrars::PrimField<std::string::String>,
    #[doc =
        "Version of the secret (version number or the string 'latest'). It is preferable to use latest version with secret volumes as secret value changes are reflected immediately."]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
    pub fn build(self) -> Cloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
        Cloudfunctions2Function_ServiceConfig_SecretVolumes_Versions {
            path: self.path,
            version: self.version,
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef {
        Cloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nRelative path of the file under the mount path where the secret value for this version will be fetched and made available. For example, setting the mountPath as '/etc/secrets' and path as secret_foo would mount the secret value file at /etc/secrets/secret_foo."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nVersion of the secret (version number or the string 'latest'). It is preferable to use latest version with secret volumes as secret value changes are reflected immediately."]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudfunctions2Function_ServiceConfig_SecretVolumesDynamic {
    versions: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudfunctions2Function_ServiceConfig_SecretVolumes_Versions>>,
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_ServiceConfig_SecretVolumes {
    mount_path: terrars::PrimField<std::string::String>,
    project_id: terrars::PrimField<std::string::String>,
    secret: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    versions: std::option::Option<std::vec::Vec<Cloudfunctions2Function_ServiceConfig_SecretVolumes_Versions>>,
    dynamic: Cloudfunctions2Function_ServiceConfig_SecretVolumesDynamic,
}

impl Cloudfunctions2Function_ServiceConfig_SecretVolumes {
    #[doc = "Set the field `versions`.\n"]
    pub fn set_versions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Cloudfunctions2Function_ServiceConfig_SecretVolumes_Versions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.versions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.versions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Cloudfunctions2Function_ServiceConfig_SecretVolumes {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_ServiceConfig_SecretVolumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_ServiceConfig_SecretVolumes {
    #[doc =
        "The path within the container to mount the secret volume. For example, setting the mountPath as /etc/secrets would mount the secret value files under the /etc/secrets directory. This directory will also be completely shadowed and unavailable to mount any other secrets. Recommended mount path: /etc/secrets"]
    pub mount_path: terrars::PrimField<std::string::String>,
    #[doc =
        "Project identifier (preferably project number but can also be the project ID) of the project that contains the secret. If not set, it will be populated with the function's project assuming that the secret exists in the same project as of the function."]
    pub project_id: terrars::PrimField<std::string::String>,
    #[doc = "Name of the secret in secret manager (not the full resource name)."]
    pub secret: terrars::PrimField<std::string::String>,
}

impl BuildCloudfunctions2Function_ServiceConfig_SecretVolumes {
    pub fn build(self) -> Cloudfunctions2Function_ServiceConfig_SecretVolumes {
        Cloudfunctions2Function_ServiceConfig_SecretVolumes {
            mount_path: self.mount_path,
            project_id: self.project_id,
            secret: self.secret,
            versions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_ServiceConfig_SecretVolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_ServiceConfig_SecretVolumesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Cloudfunctions2Function_ServiceConfig_SecretVolumesRef {
        Cloudfunctions2Function_ServiceConfig_SecretVolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_ServiceConfig_SecretVolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mount_path` after provisioning.\nThe path within the container to mount the secret volume. For example, setting the mountPath as /etc/secrets would mount the secret value files under the /etc/secrets directory. This directory will also be completely shadowed and unavailable to mount any other secrets. Recommended mount path: /etc/secrets"]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `project_id` after provisioning.\nProject identifier (preferably project number but can also be the project ID) of the project that contains the secret. If not set, it will be populated with the function's project assuming that the secret exists in the same project as of the function."]
    pub fn project_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret` after provisioning.\nName of the secret in secret manager (not the full resource name)."]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.base))
    }

    #[doc = "Get a reference to the value of field `versions` after provisioning.\n"]
    pub fn versions(&self) -> terrars::ListRef<Cloudfunctions2Function_ServiceConfig_SecretVolumes_VersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.versions", self.base))
    }
}

#[derive(Serialize, Default)]
struct Cloudfunctions2Function_ServiceConfigDynamic {
    direct_vpc_network_interface: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface>>,
    secret_environment_variables: std
    ::option
    ::Option<terrars::DynamicBlock<Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables>>,
    secret_volumes: std::option::Option<terrars::DynamicBlock<Cloudfunctions2Function_ServiceConfig_SecretVolumes>>,
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_ServiceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_traffic_on_latest_revision: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_memory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    binary_authorization_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_vpc_egress: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment_variables: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_settings: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_instance_request_concurrency: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_connector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_connector_egress_settings: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direct_vpc_network_interface: std
    ::option
    ::Option<std::vec::Vec<Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_environment_variables: std
    ::option
    ::Option<std::vec::Vec<Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_volumes: std::option::Option<std::vec::Vec<Cloudfunctions2Function_ServiceConfig_SecretVolumes>>,
    dynamic: Cloudfunctions2Function_ServiceConfigDynamic,
}

impl Cloudfunctions2Function_ServiceConfig {
    #[doc =
        "Set the field `all_traffic_on_latest_revision`.\nWhether 100% of traffic is routed to the latest revision. Defaults to true. When false, GCF honors the existing traffic configuration of the underlying Cloud Run service. If that configuration is set to route to LATEST (the default), the new deployment will become LATEST and intercept the traffic. To prevent traffic from shifting, you must manually pin the existing service to a specific revision name in Cloud Run before deploying."]
    pub fn set_all_traffic_on_latest_revision(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_traffic_on_latest_revision = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `available_cpu`.\nThe number of CPUs used in a single container instance. Default value is calculated from available memory."]
    pub fn set_available_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.available_cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `available_memory`.\nThe amount of memory available for a function.\nDefaults to 256M. Supported units are k, M, G, Mi, Gi. If no unit is\nsupplied the value is interpreted as bytes."]
    pub fn set_available_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.available_memory = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `binary_authorization_policy`.\nThe binary authorization policy to be checked when deploying the Cloud Run service."]
    pub fn set_binary_authorization_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.binary_authorization_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `direct_vpc_egress`.\nEgress settings for direct VPC. If not provided, it defaults to VPC_EGRESS_PRIVATE_RANGES_ONLY. Possible values: [\"VPC_EGRESS_ALL_TRAFFIC\", \"VPC_EGRESS_PRIVATE_RANGES_ONLY\"]"]
    pub fn set_direct_vpc_egress(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direct_vpc_egress = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `environment_variables`.\nEnvironment variables that shall be available during function execution."]
    pub fn set_environment_variables(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `ingress_settings`.\nAvailable ingress settings. Defaults to \"ALLOW_ALL\" if unspecified. Default value: \"ALLOW_ALL\" Possible values: [\"ALLOW_ALL\", \"ALLOW_INTERNAL_ONLY\", \"ALLOW_INTERNAL_AND_GCLB\"]"]
    pub fn set_ingress_settings(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ingress_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_instance_count`.\nThe limit on the maximum number of function instances that may coexist at a\ngiven time."]
    pub fn set_max_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `max_instance_request_concurrency`.\nSets the maximum number of concurrent requests that each instance can receive. Defaults to 1."]
    pub fn set_max_instance_request_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_instance_request_concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_instance_count`.\nThe limit on the minimum number of function instances that may coexist at a\ngiven time."]
    pub fn set_min_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_account_email`.\nThe email of the service account for this function."]
    pub fn set_service_account_email(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_email = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nThe function execution timeout. Execution is considered failed and\ncan be terminated if the function is not completed at the end of the\ntimeout period. Defaults to 60 seconds."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vpc_connector`.\nThe Serverless VPC Access connector that this cloud function can connect to."]
    pub fn set_vpc_connector(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_connector = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `vpc_connector_egress_settings`.\nAvailable egress settings. Possible values: [\"VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED\", \"PRIVATE_RANGES_ONLY\", \"ALL_TRAFFIC\"]"]
    pub fn set_vpc_connector_egress_settings(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vpc_connector_egress_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direct_vpc_network_interface`.\n"]
    pub fn set_direct_vpc_network_interface(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterface>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.direct_vpc_network_interface = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.direct_vpc_network_interface = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_environment_variables`.\n"]
    pub fn set_secret_environment_variables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariables>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_environment_variables = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_environment_variables = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_volumes`.\n"]
    pub fn set_secret_volumes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Cloudfunctions2Function_ServiceConfig_SecretVolumes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_volumes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_volumes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Cloudfunctions2Function_ServiceConfig {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_ServiceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_ServiceConfig {}

impl BuildCloudfunctions2Function_ServiceConfig {
    pub fn build(self) -> Cloudfunctions2Function_ServiceConfig {
        Cloudfunctions2Function_ServiceConfig {
            all_traffic_on_latest_revision: core::default::Default::default(),
            available_cpu: core::default::Default::default(),
            available_memory: core::default::Default::default(),
            binary_authorization_policy: core::default::Default::default(),
            direct_vpc_egress: core::default::Default::default(),
            environment_variables: core::default::Default::default(),
            ingress_settings: core::default::Default::default(),
            max_instance_count: core::default::Default::default(),
            max_instance_request_concurrency: core::default::Default::default(),
            min_instance_count: core::default::Default::default(),
            service_account_email: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            vpc_connector: core::default::Default::default(),
            vpc_connector_egress_settings: core::default::Default::default(),
            direct_vpc_network_interface: core::default::Default::default(),
            secret_environment_variables: core::default::Default::default(),
            secret_volumes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_ServiceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_ServiceConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudfunctions2Function_ServiceConfigRef {
        Cloudfunctions2Function_ServiceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_ServiceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `all_traffic_on_latest_revision` after provisioning.\nWhether 100% of traffic is routed to the latest revision. Defaults to true. When false, GCF honors the existing traffic configuration of the underlying Cloud Run service. If that configuration is set to route to LATEST (the default), the new deployment will become LATEST and intercept the traffic. To prevent traffic from shifting, you must manually pin the existing service to a specific revision name in Cloud Run before deploying."]
    pub fn all_traffic_on_latest_revision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_traffic_on_latest_revision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `available_cpu` after provisioning.\nThe number of CPUs used in a single container instance. Default value is calculated from available memory."]
    pub fn available_cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_cpu", self.base))
    }

    #[doc =
        "Get a reference to the value of field `available_memory` after provisioning.\nThe amount of memory available for a function.\nDefaults to 256M. Supported units are k, M, G, Mi, Gi. If no unit is\nsupplied the value is interpreted as bytes."]
    pub fn available_memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_memory", self.base))
    }

    #[doc =
        "Get a reference to the value of field `binary_authorization_policy` after provisioning.\nThe binary authorization policy to be checked when deploying the Cloud Run service."]
    pub fn binary_authorization_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.binary_authorization_policy", self.base))
    }

    #[doc =
        "Get a reference to the value of field `direct_vpc_egress` after provisioning.\nEgress settings for direct VPC. If not provided, it defaults to VPC_EGRESS_PRIVATE_RANGES_ONLY. Possible values: [\"VPC_EGRESS_ALL_TRAFFIC\", \"VPC_EGRESS_PRIVATE_RANGES_ONLY\"]"]
    pub fn direct_vpc_egress(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direct_vpc_egress", self.base))
    }

    #[doc =
        "Get a reference to the value of field `environment_variables` after provisioning.\nEnvironment variables that shall be available during function execution."]
    pub fn environment_variables(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `gcf_uri` after provisioning.\nURIs of the Service deployed"]
    pub fn gcf_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcf_uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ingress_settings` after provisioning.\nAvailable ingress settings. Defaults to \"ALLOW_ALL\" if unspecified. Default value: \"ALLOW_ALL\" Possible values: [\"ALLOW_ALL\", \"ALLOW_INTERNAL_ONLY\", \"ALLOW_INTERNAL_AND_GCLB\"]"]
    pub fn ingress_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ingress_settings", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_instance_count` after provisioning.\nThe limit on the maximum number of function instances that may coexist at a\ngiven time."]
    pub fn max_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_instance_request_concurrency` after provisioning.\nSets the maximum number of concurrent requests that each instance can receive. Defaults to 1."]
    pub fn max_instance_request_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_instance_request_concurrency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_instance_count` after provisioning.\nThe limit on the minimum number of function instances that may coexist at a\ngiven time."]
    pub fn min_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_instance_count", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nName of the service associated with a Function."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_email` after provisioning.\nThe email of the service account for this function."]
    pub fn service_account_email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_email", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nThe function execution timeout. Execution is considered failed and\ncan be terminated if the function is not completed at the end of the\ntimeout period. Defaults to 60 seconds."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\nURI of the Service deployed."]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vpc_connector` after provisioning.\nThe Serverless VPC Access connector that this cloud function can connect to."]
    pub fn vpc_connector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_connector", self.base))
    }

    #[doc =
        "Get a reference to the value of field `vpc_connector_egress_settings` after provisioning.\nAvailable egress settings. Possible values: [\"VPC_CONNECTOR_EGRESS_SETTINGS_UNSPECIFIED\", \"PRIVATE_RANGES_ONLY\", \"ALL_TRAFFIC\"]"]
    pub fn vpc_connector_egress_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_connector_egress_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `direct_vpc_network_interface` after provisioning.\n"]
    pub fn direct_vpc_network_interface(
        &self,
    ) -> terrars::ListRef<Cloudfunctions2Function_ServiceConfig_DirectVpcNetworkInterfaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.direct_vpc_network_interface", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_environment_variables` after provisioning.\n"]
    pub fn secret_environment_variables(
        &self,
    ) -> terrars::ListRef<Cloudfunctions2Function_ServiceConfig_SecretEnvironmentVariablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_environment_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_volumes` after provisioning.\n"]
    pub fn secret_volumes(&self) -> terrars::ListRef<Cloudfunctions2Function_ServiceConfig_SecretVolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_volumes", self.base))
    }
}

#[derive(Serialize)]
pub struct Cloudfunctions2Function_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Cloudfunctions2Function_Timeouts {
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

impl terrars::ToListMappable for Cloudfunctions2Function_Timeouts {
    type O = terrars::BlockAssignable<Cloudfunctions2Function_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfunctions2Function_Timeouts {}

impl BuildCloudfunctions2Function_Timeouts {
    pub fn build(self) -> Cloudfunctions2Function_Timeouts {
        Cloudfunctions2Function_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Cloudfunctions2Function_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Cloudfunctions2Function_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Cloudfunctions2Function_TimeoutsRef {
        Cloudfunctions2Function_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Cloudfunctions2Function_TimeoutsRef {
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
