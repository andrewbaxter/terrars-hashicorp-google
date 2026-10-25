#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ClouddeployCustomTargetTypeData_ {
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
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_actions: std::option::Option<std::vec::Vec<ClouddeployCustomTargetType_CustomActions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tasks: std::option::Option<std::vec::Vec<ClouddeployCustomTargetType_Tasks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ClouddeployCustomTargetType_Timeouts>,
    dynamic: ClouddeployCustomTargetTypeDynamic,
}

struct ClouddeployCustomTargetType_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ClouddeployCustomTargetTypeData_>,
}

#[derive(Clone)]
pub struct ClouddeployCustomTargetType(std::rc::Rc<ClouddeployCustomTargetType_>);

impl ClouddeployCustomTargetType {
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
        "Set the field `annotations`.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\nDescription of the 'CustomTargetType'. Max length is 255 characters."]
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
        "Set the field `labels`.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
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

    #[doc = "Set the field `custom_actions`.\n"]
    pub fn set_custom_actions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployCustomTargetType_CustomActions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tasks`.\n"]
    pub fn set_tasks(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployCustomTargetType_Tasks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tasks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tasks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ClouddeployCustomTargetType_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime at which the 'CustomTargetType' was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_target_type_id` after provisioning.\nResource id of the 'CustomTargetType'."]
    pub fn custom_target_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_target_type_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the 'CustomTargetType'. Max length is 255 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe weak etag of the 'CustomTargetType' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the source."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'CustomTargetType'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the 'CustomTargetType'."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime at which the 'CustomTargetType' was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_actions` after provisioning.\n"]
    pub fn custom_actions(&self) -> terrars::ListRef<ClouddeployCustomTargetType_CustomActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tasks` after provisioning.\n"]
    pub fn tasks(&self) -> terrars::ListRef<ClouddeployCustomTargetType_TasksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tasks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployCustomTargetType_TimeoutsRef {
        ClouddeployCustomTargetType_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ClouddeployCustomTargetType {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ClouddeployCustomTargetType { }

impl terrars::ToListMappable for ClouddeployCustomTargetType {
    type O = terrars::ListRef<ClouddeployCustomTargetTypeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ClouddeployCustomTargetType_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_clouddeploy_custom_target_type".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildClouddeployCustomTargetType {
    pub tf_id: std::string::String,
    #[doc = "The location of the source."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Name of the 'CustomTargetType'."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployCustomTargetType {
    pub fn build(self, stack: &mut terrars::Stack) -> ClouddeployCustomTargetType {
        let out = ClouddeployCustomTargetType(std::rc::Rc::new(ClouddeployCustomTargetType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ClouddeployCustomTargetTypeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                annotations: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                custom_actions: core::default::Default::default(),
                tasks: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ClouddeployCustomTargetTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployCustomTargetTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ClouddeployCustomTargetTypeRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nUser annotations. These attributes can only be set and used by the user, and not by Cloud Deploy. See https://google.aip.dev/128#annotations for more details such as format and size limitations.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `create_time` after provisioning.\nTime at which the 'CustomTargetType' was created."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `custom_target_type_id` after provisioning.\nResource id of the 'CustomTargetType'."]
    pub fn custom_target_type_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_target_type_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nDescription of the 'CustomTargetType'. Max length is 255 characters."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `etag` after provisioning.\nThe weak etag of the 'CustomTargetType' resource. This checksum is computed by the server based on the value of other fields, and may be sent on update and delete requests to ensure the client has an up-to-date value before proceeding."]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nLabels are attributes that can be set and used by both the user and by Cloud Deploy. Labels must meet the following constraints: * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. * All characters must use UTF-8 encoding, and international characters are allowed. * Keys must start with a lowercase letter or international character. * Each resource is limited to a maximum of 64 labels. Both keys and values are additionally constrained to be <= 128 bytes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\nThe location of the source."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the 'CustomTargetType'."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nUnique identifier of the 'CustomTargetType'."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `update_time` after provisioning.\nTime at which the 'CustomTargetType' was updated."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_actions` after provisioning.\n"]
    pub fn custom_actions(&self) -> terrars::ListRef<ClouddeployCustomTargetType_CustomActionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_actions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tasks` after provisioning.\n"]
    pub fn tasks(&self) -> terrars::ListRef<ClouddeployCustomTargetType_TasksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tasks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ClouddeployCustomTargetType_TimeoutsRef {
        ClouddeployCustomTargetType_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ClouddeployCustomTargetTypeDynamic {
    custom_actions: std::option::Option<terrars::DynamicBlock<ClouddeployCustomTargetType_CustomActions>>,
    tasks: std::option::Option<terrars::DynamicBlock<ClouddeployCustomTargetType_Tasks>>,
}

#[derive(Serialize)]
pub struct ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_Git {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
    repo: terrars::PrimField<std::string::String>,
}

impl ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_Git {
    #[doc = "Set the field `path`.\nRelative path from the repository root to the Skaffold file."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ref_`.\nGit ref the package should be cloned from."]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_Git {
    type O = terrars::BlockAssignable<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_Git>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_Git {
    #[doc = "Git repository the package should be cloned from."]
    pub repo: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_Git {
    pub fn build(self) -> ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_Git {
        ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_Git {
            path: core::default::Default::default(),
            ref_: core::default::Default::default(),
            repo: self.repo,
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GitRef {
        ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nRelative path from the repository root to the Skaffold file."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ref_` after provisioning.\nGit ref the package should be cloned from."]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repo` after provisioning.\nGit repository the package should be cloned from."]
    pub fn repo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repo", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "ref", skip_serializing_if = "std::option::Option::is_none")]
    ref_: std::option::Option<terrars::PrimField<std::string::String>>,
    repository: terrars::PrimField<std::string::String>,
}

impl ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepo {
    #[doc = "Set the field `path`.\nRelative path from the repository root to the Skaffold file."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ref_`.\nBranch or tag to use when cloning the repository."]
    pub fn set_ref(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepo {
    type O =
        terrars
        ::BlockAssignable<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepo {
    #[doc =
        "Cloud Build 2nd gen repository in the format of 'projects/<project>/locations/<location>/connections/<connection>/repositories/<repository>'."]
    pub repository: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepo {
    pub fn build(self) -> ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepo {
        ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepo {
            path: core::default::Default::default(),
            ref_: core::default::Default::default(),
            repository: self.repository,
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepoRef {
        ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nRelative path from the repository root to the Skaffold file."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ref_` after provisioning.\nBranch or tag to use when cloning the repository."]
    pub fn ref_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref", self.base))
    }

    #[doc =
        "Get a reference to the value of field `repository` after provisioning.\nCloud Build 2nd gen repository in the format of 'projects/<project>/locations/<location>/connections/<connection>/repositories/<repository>'."]
    pub fn repository(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.repository", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    source: terrars::PrimField<std::string::String>,
}

impl ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorage {
    #[doc = "Set the field `path`.\nRelative path from the source to the Skaffold file."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorage {
    type O =
        terrars
        ::BlockAssignable<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorage {
    #[doc =
        "Cloud Storage source paths to copy recursively. For example, providing 'gs://my-bucket/dir/configs/*' will result in Skaffold copying all files within the 'dir/configs' directory in the bucket 'my-bucket'."]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorage {
    pub fn build(self) -> ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorage {
        ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorage {
            path: core::default::Default::default(),
            source: self.source,
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorageRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorageRef {
        ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nRelative path from the source to the Skaffold file."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `source` after provisioning.\nCloud Storage source paths to copy recursively. For example, providing 'gs://my-bucket/dir/configs/*' will result in Skaffold copying all files within the 'dir/configs' directory in the bucket 'my-bucket'."]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModulesDynamic {
    git: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_Git>>,
    google_cloud_build_repo: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepo>,
    >,
    google_cloud_storage: std
    ::option
    ::Option<
        terrars::DynamicBlock<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorage>,
    >,
}

#[derive(Serialize)]
pub struct ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    git: std::option::Option<std::vec::Vec<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_Git>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_build_repo: std
    ::option
    ::Option<std::vec::Vec<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    google_cloud_storage: std
    ::option
    ::Option<std::vec::Vec<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorage>>,
    dynamic: ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModulesDynamic,
}

impl ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules {
    #[doc = "Set the field `configs`.\nThe Skaffold Config modules to use from the specified source."]
    pub fn set_configs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.configs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `git`.\n"]
    pub fn set_git(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_Git>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.git = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.git = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_cloud_build_repo`.\n"]
    pub fn set_google_cloud_build_repo(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepo,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_cloud_build_repo = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud_build_repo = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `google_cloud_storage`.\n"]
    pub fn set_google_cloud_storage(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorage,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.google_cloud_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.google_cloud_storage = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules {
    type O = terrars::BlockAssignable<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules {}

impl BuildClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules {
    pub fn build(self) -> ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules {
        ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules {
            configs: core::default::Default::default(),
            git: core::default::Default::default(),
            google_cloud_build_repo: core::default::Default::default(),
            google_cloud_storage: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModulesRef {
        ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `configs` after provisioning.\nThe Skaffold Config modules to use from the specified source."]
    pub fn configs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configs", self.base))
    }

    #[doc = "Get a reference to the value of field `git` after provisioning.\n"]
    pub fn git(&self) -> terrars::ListRef<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.git", self.base))
    }

    #[doc = "Get a reference to the value of field `google_cloud_build_repo` after provisioning.\n"]
    pub fn google_cloud_build_repo(
        &self,
    ) -> terrars::ListRef<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudBuildRepoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_cloud_build_repo", self.base))
    }

    #[doc = "Get a reference to the value of field `google_cloud_storage` after provisioning.\n"]
    pub fn google_cloud_storage(
        &self,
    ) -> terrars::ListRef<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules_GoogleCloudStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.google_cloud_storage", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployCustomTargetType_CustomActionsDynamic {
    include_skaffold_modules: std
    ::option
    ::Option<terrars::DynamicBlock<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules>>,
}

#[derive(Serialize)]
pub struct ClouddeployCustomTargetType_CustomActions {
    deploy_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    render_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_skaffold_modules: std
    ::option
    ::Option<std::vec::Vec<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules>>,
    dynamic: ClouddeployCustomTargetType_CustomActionsDynamic,
}

impl ClouddeployCustomTargetType_CustomActions {
    #[doc =
        "Set the field `render_action`.\nThe Skaffold custom action responsible for render operations. If not provided then Cloud Deploy will perform the render operations via 'skaffold render'."]
    pub fn set_render_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.render_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_skaffold_modules`.\n"]
    pub fn set_include_skaffold_modules(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModules>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include_skaffold_modules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include_skaffold_modules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployCustomTargetType_CustomActions {
    type O = terrars::BlockAssignable<ClouddeployCustomTargetType_CustomActions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployCustomTargetType_CustomActions {
    #[doc = "The Skaffold custom action responsible for deploy operations."]
    pub deploy_action: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployCustomTargetType_CustomActions {
    pub fn build(self) -> ClouddeployCustomTargetType_CustomActions {
        ClouddeployCustomTargetType_CustomActions {
            deploy_action: self.deploy_action,
            render_action: core::default::Default::default(),
            include_skaffold_modules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployCustomTargetType_CustomActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployCustomTargetType_CustomActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployCustomTargetType_CustomActionsRef {
        ClouddeployCustomTargetType_CustomActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployCustomTargetType_CustomActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deploy_action` after provisioning.\nThe Skaffold custom action responsible for deploy operations."]
    pub fn deploy_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deploy_action", self.base))
    }

    #[doc =
        "Get a reference to the value of field `render_action` after provisioning.\nThe Skaffold custom action responsible for render operations. If not provided then Cloud Deploy will perform the render operations via 'skaffold render'."]
    pub fn render_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.render_action", self.base))
    }

    #[doc = "Get a reference to the value of field `include_skaffold_modules` after provisioning.\n"]
    pub fn include_skaffold_modules(
        &self,
    ) -> terrars::ListRef<ClouddeployCustomTargetType_CustomActions_IncludeSkaffoldModulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include_skaffold_modules", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployCustomTargetType_Tasks_Deploy_Container {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    image: terrars::PrimField<std::string::String>,
}

impl ClouddeployCustomTargetType_Tasks_Deploy_Container {
    #[doc =
        "Set the field `args`.\nArgs is the container arguments to use. This overrides the default arguments defined in the container image."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `command`.\nCommand is the container entrypoint to use. This overrides the default entrypoint defined in the container image."]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\nEnvironment variables that are set in the container."]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.env = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ClouddeployCustomTargetType_Tasks_Deploy_Container {
    type O = terrars::BlockAssignable<ClouddeployCustomTargetType_Tasks_Deploy_Container>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployCustomTargetType_Tasks_Deploy_Container {
    #[doc = "Image is the container image to use."]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildClouddeployCustomTargetType_Tasks_Deploy_Container {
    pub fn build(self) -> ClouddeployCustomTargetType_Tasks_Deploy_Container {
        ClouddeployCustomTargetType_Tasks_Deploy_Container {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            env: core::default::Default::default(),
            image: self.image,
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployCustomTargetType_Tasks_Deploy_ContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployCustomTargetType_Tasks_Deploy_ContainerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ClouddeployCustomTargetType_Tasks_Deploy_ContainerRef {
        ClouddeployCustomTargetType_Tasks_Deploy_ContainerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployCustomTargetType_Tasks_Deploy_ContainerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nArgs is the container arguments to use. This overrides the default arguments defined in the container image."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nCommand is the container entrypoint to use. This overrides the default entrypoint defined in the container image."]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `env` after provisioning.\nEnvironment variables that are set in the container."]
    pub fn env(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.env", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\nImage is the container image to use."]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployCustomTargetType_Tasks_DeployDynamic {
    container: std::option::Option<terrars::DynamicBlock<ClouddeployCustomTargetType_Tasks_Deploy_Container>>,
}

#[derive(Serialize)]
pub struct ClouddeployCustomTargetType_Tasks_Deploy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<std::vec::Vec<ClouddeployCustomTargetType_Tasks_Deploy_Container>>,
    dynamic: ClouddeployCustomTargetType_Tasks_DeployDynamic,
}

impl ClouddeployCustomTargetType_Tasks_Deploy {
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployCustomTargetType_Tasks_Deploy_Container>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployCustomTargetType_Tasks_Deploy {
    type O = terrars::BlockAssignable<ClouddeployCustomTargetType_Tasks_Deploy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployCustomTargetType_Tasks_Deploy {}

impl BuildClouddeployCustomTargetType_Tasks_Deploy {
    pub fn build(self) -> ClouddeployCustomTargetType_Tasks_Deploy {
        ClouddeployCustomTargetType_Tasks_Deploy {
            container: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployCustomTargetType_Tasks_DeployRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployCustomTargetType_Tasks_DeployRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployCustomTargetType_Tasks_DeployRef {
        ClouddeployCustomTargetType_Tasks_DeployRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployCustomTargetType_Tasks_DeployRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<ClouddeployCustomTargetType_Tasks_Deploy_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployCustomTargetType_Tasks_RenderDynamic {
    container: std::option::Option<terrars::DynamicBlock<ClouddeployCustomTargetType_Tasks_Deploy_Container>>,
}

#[derive(Serialize)]
pub struct ClouddeployCustomTargetType_Tasks_Render {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<std::vec::Vec<ClouddeployCustomTargetType_Tasks_Deploy_Container>>,
    dynamic: ClouddeployCustomTargetType_Tasks_RenderDynamic,
}

impl ClouddeployCustomTargetType_Tasks_Render {
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployCustomTargetType_Tasks_Deploy_Container>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployCustomTargetType_Tasks_Render {
    type O = terrars::BlockAssignable<ClouddeployCustomTargetType_Tasks_Render>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployCustomTargetType_Tasks_Render {}

impl BuildClouddeployCustomTargetType_Tasks_Render {
    pub fn build(self) -> ClouddeployCustomTargetType_Tasks_Render {
        ClouddeployCustomTargetType_Tasks_Render {
            container: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployCustomTargetType_Tasks_RenderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployCustomTargetType_Tasks_RenderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployCustomTargetType_Tasks_RenderRef {
        ClouddeployCustomTargetType_Tasks_RenderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployCustomTargetType_Tasks_RenderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<ClouddeployCustomTargetType_Tasks_Deploy_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }
}

#[derive(Serialize, Default)]
struct ClouddeployCustomTargetType_TasksDynamic {
    deploy: std::option::Option<terrars::DynamicBlock<ClouddeployCustomTargetType_Tasks_Deploy>>,
    render: std::option::Option<terrars::DynamicBlock<ClouddeployCustomTargetType_Tasks_Render>>,
}

#[derive(Serialize)]
pub struct ClouddeployCustomTargetType_Tasks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deploy: std::option::Option<std::vec::Vec<ClouddeployCustomTargetType_Tasks_Deploy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    render: std::option::Option<std::vec::Vec<ClouddeployCustomTargetType_Tasks_Render>>,
    dynamic: ClouddeployCustomTargetType_TasksDynamic,
}

impl ClouddeployCustomTargetType_Tasks {
    #[doc = "Set the field `deploy`.\n"]
    pub fn set_deploy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployCustomTargetType_Tasks_Deploy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deploy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deploy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `render`.\n"]
    pub fn set_render(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ClouddeployCustomTargetType_Tasks_Render>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.render = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.render = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ClouddeployCustomTargetType_Tasks {
    type O = terrars::BlockAssignable<ClouddeployCustomTargetType_Tasks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployCustomTargetType_Tasks {}

impl BuildClouddeployCustomTargetType_Tasks {
    pub fn build(self) -> ClouddeployCustomTargetType_Tasks {
        ClouddeployCustomTargetType_Tasks {
            deploy: core::default::Default::default(),
            render: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployCustomTargetType_TasksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployCustomTargetType_TasksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployCustomTargetType_TasksRef {
        ClouddeployCustomTargetType_TasksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployCustomTargetType_TasksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deploy` after provisioning.\n"]
    pub fn deploy(&self) -> terrars::ListRef<ClouddeployCustomTargetType_Tasks_DeployRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deploy", self.base))
    }

    #[doc = "Get a reference to the value of field `render` after provisioning.\n"]
    pub fn render(&self) -> terrars::ListRef<ClouddeployCustomTargetType_Tasks_RenderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.render", self.base))
    }
}

#[derive(Serialize)]
pub struct ClouddeployCustomTargetType_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ClouddeployCustomTargetType_Timeouts {
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

impl terrars::ToListMappable for ClouddeployCustomTargetType_Timeouts {
    type O = terrars::BlockAssignable<ClouddeployCustomTargetType_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildClouddeployCustomTargetType_Timeouts {}

impl BuildClouddeployCustomTargetType_Timeouts {
    pub fn build(self) -> ClouddeployCustomTargetType_Timeouts {
        ClouddeployCustomTargetType_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ClouddeployCustomTargetType_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ClouddeployCustomTargetType_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ClouddeployCustomTargetType_TimeoutsRef {
        ClouddeployCustomTargetType_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ClouddeployCustomTargetType_TimeoutsRef {
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
