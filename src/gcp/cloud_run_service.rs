#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct CloudRunServiceData_ {
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
    autogenerate_revision_name: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<CloudRunService_Metadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template: std::option::Option<std::vec::Vec<CloudRunService_Template>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudRunService_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic: std::option::Option<std::vec::Vec<CloudRunService_Traffic>>,
    dynamic: CloudRunServiceDynamic,
}

struct CloudRunService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudRunServiceData_>,
}

#[derive(Clone)]
pub struct CloudRunService(std::rc::Rc<CloudRunService_>);

impl CloudRunService {
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
        "Set the field `autogenerate_revision_name`.\nIf set to 'true', the revision name (template.metadata.name) will be omitted and\nautogenerated by Cloud Run. This cannot be set to 'true' while 'template.metadata.name'\nis also set.\n(For legacy support, if 'template.metadata.name' is unset in state while\nthis field is set to false, the revision name will still autogenerate.)"]
    pub fn set_autogenerate_revision_name(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().autogenerate_revision_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(self, v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Metadata>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `template`.\n"]
    pub fn set_template(self, v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudRunService_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic`.\n"]
    pub fn set_traffic(self, v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Traffic>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `autogenerate_revision_name` after provisioning.\nIf set to 'true', the revision name (template.metadata.name) will be omitted and\nautogenerated by Cloud Run. This cannot be set to 'true' while 'template.metadata.name'\nis also set.\n(For legacy support, if 'template.metadata.name' is unset in state while\nthis field is set to false, the revision name will still autogenerate.)"]
    pub fn autogenerate_revision_name(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autogenerate_revision_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run instance. eg us-central1"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName must be unique within a Google Cloud project and region.\nIs required when creating resources. Name is primarily intended\nfor creation idempotence and configuration definition. Cannot be updated.\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe current status of the Service."]
    pub fn status(&self) -> terrars::ListRef<CloudRunService_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<CloudRunService_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::ListRef<CloudRunService_TemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunService_TimeoutsRef {
        CloudRunService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic` after provisioning.\n"]
    pub fn traffic(&self) -> terrars::ListRef<CloudRunService_TrafficRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }
}

impl terrars::Referable for CloudRunService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudRunService { }

impl terrars::ToListMappable for CloudRunService {
    type O = terrars::ListRef<CloudRunServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudRunService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_cloud_run_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudRunService {
    pub tf_id: std::string::String,
    #[doc = "The location of the cloud run instance. eg us-central1"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc =
        "Name must be unique within a Google Cloud project and region.\nIs required when creating resources. Name is primarily intended\nfor creation idempotence and configuration definition. Cannot be updated.\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunService {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudRunService {
        let out = CloudRunService(std::rc::Rc::new(CloudRunService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudRunServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                autogenerate_revision_name: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                name: self.name,
                project: core::default::Default::default(),
                metadata: core::default::Default::default(),
                template: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                traffic: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudRunServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudRunServiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `autogenerate_revision_name` after provisioning.\nIf set to 'true', the revision name (template.metadata.name) will be omitted and\nautogenerated by Cloud Run. This cannot be set to 'true' while 'template.metadata.name'\nis also set.\n(For legacy support, if 'template.metadata.name' is unset in state while\nthis field is set to false, the revision name will still autogenerate.)"]
    pub fn autogenerate_revision_name(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autogenerate_revision_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location of the cloud run instance. eg us-central1"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName must be unique within a Google Cloud project and region.\nIs required when creating resources. Name is primarily intended\nfor creation idempotence and configuration definition. Cannot be updated.\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nThe current status of the Service."]
    pub fn status(&self) -> terrars::ListRef<CloudRunService_StatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<CloudRunService_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template` after provisioning.\n"]
    pub fn template(&self) -> terrars::ListRef<CloudRunService_TemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudRunService_TimeoutsRef {
        CloudRunService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic` after provisioning.\n"]
    pub fn traffic(&self) -> terrars::ListRef<CloudRunService_TrafficRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudRunServiceDynamic {
    metadata: std::option::Option<terrars::DynamicBlock<CloudRunService_Metadata>>,
    template: std::option::Option<terrars::DynamicBlock<CloudRunService_Template>>,
    traffic: std::option::Option<terrars::DynamicBlock<CloudRunService_Traffic>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Status_Conditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Status_Conditions {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Status_Conditions {
    type O = terrars::BlockAssignable<CloudRunService_Status_Conditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Status_Conditions {}

impl BuildCloudRunService_Status_Conditions {
    pub fn build(self) -> CloudRunService_Status_Conditions {
        CloudRunService_Status_Conditions {
            message: core::default::Default::default(),
            reason: core::default::Default::default(),
            status: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Status_ConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Status_ConditionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_Status_ConditionsRef {
        CloudRunService_Status_ConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Status_ConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Status_Traffic {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_revision: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Status_Traffic {
    #[doc = "Set the field `latest_revision`.\n"]
    pub fn set_latest_revision(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.latest_revision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `percent`.\n"]
    pub fn set_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `revision_name`.\n"]
    pub fn set_revision_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tag`.\n"]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Status_Traffic {
    type O = terrars::BlockAssignable<CloudRunService_Status_Traffic>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Status_Traffic {}

impl BuildCloudRunService_Status_Traffic {
    pub fn build(self) -> CloudRunService_Status_Traffic {
        CloudRunService_Status_Traffic {
            latest_revision: core::default::Default::default(),
            percent: core::default::Default::default(),
            revision_name: core::default::Default::default(),
            tag: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Status_TrafficRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Status_TrafficRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_Status_TrafficRef {
        CloudRunService_Status_TrafficRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Status_TrafficRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `latest_revision` after provisioning.\n"]
    pub fn latest_revision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_revision", self.base))
    }

    #[doc = "Get a reference to the value of field `percent` after provisioning.\n"]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }

    #[doc = "Get a reference to the value of field `revision_name` after provisioning.\n"]
    pub fn revision_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tag` after provisioning.\n"]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conditions: std::option::Option<terrars::ListField<CloudRunService_Status_Conditions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_created_revision_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_ready_revision_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    observed_generation: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic: std::option::Option<terrars::ListField<CloudRunService_Status_Traffic>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Status {
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CloudRunService_Status_Conditions>>,
    ) -> Self {
        self.conditions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `latest_created_revision_name`.\n"]
    pub fn set_latest_created_revision_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.latest_created_revision_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `latest_ready_revision_name`.\n"]
    pub fn set_latest_ready_revision_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.latest_ready_revision_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `observed_generation`.\n"]
    pub fn set_observed_generation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.observed_generation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic`.\n"]
    pub fn set_traffic(
        mut self,
        v: impl std::convert::Into<terrars::ListField<CloudRunService_Status_Traffic>>,
    ) -> Self {
        self.traffic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Status {
    type O = terrars::BlockAssignable<CloudRunService_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Status {}

impl BuildCloudRunService_Status {
    pub fn build(self) -> CloudRunService_Status {
        CloudRunService_Status {
            conditions: core::default::Default::default(),
            latest_created_revision_name: core::default::Default::default(),
            latest_ready_revision_name: core::default::Default::default(),
            observed_generation: core::default::Default::default(),
            traffic: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_StatusRef {
        CloudRunService_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> terrars::ListRef<CloudRunService_Status_ConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }

    #[doc = "Get a reference to the value of field `latest_created_revision_name` after provisioning.\n"]
    pub fn latest_created_revision_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_created_revision_name", self.base))
    }

    #[doc = "Get a reference to the value of field `latest_ready_revision_name` after provisioning.\n"]
    pub fn latest_ready_revision_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_ready_revision_name", self.base))
    }

    #[doc = "Get a reference to the value of field `observed_generation` after provisioning.\n"]
    pub fn observed_generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.observed_generation", self.base))
    }

    #[doc = "Get a reference to the value of field `traffic` after provisioning.\n"]
    pub fn traffic(&self) -> terrars::ListRef<CloudRunService_Status_TrafficRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Metadata {
    #[doc =
        "Set the field `annotations`.\nAnnotations is a key value map stored with a resource that\nmay be set by external tools to store and retrieve arbitrary metadata.\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations\n\n**Note**: The Cloud Run API may add additional annotations that were not provided in your config.\nIf terraform plan shows a diff where a server-side annotation is added, you can add it to your config\nor apply the lifecycle.ignore_changes rule to the metadata.0.annotations field.\n\nAnnotations with 'run.googleapis.com/' and 'autoscaling.knative.dev' are restricted. Use the following annotation\nkeys to configure features on a Service:\n\n- 'run.googleapis.com/binary-authorization-breakglass' sets the [Binary Authorization breakglass](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--breakglass).\n- 'run.googleapis.com/binary-authorization' sets the [Binary Authorization](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--binary-authorization).\n- 'run.googleapis.com/client-name' sets the client name calling the Cloud Run API.\n- 'run.googleapis.com/custom-audiences' sets the [custom audiences](https://cloud.google.com/sdk/gcloud/reference/alpha/run/deploy#--add-custom-audiences)\n  that can be used in the audience field of ID token for authenticated requests.\n- 'run.googleapis.com/description' sets a user defined description for the Service.\n- 'run.googleapis.com/ingress' sets the [ingress settings](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--ingress)\n  for the Service. For example, '\"run.googleapis.com/ingress\" = \"all\"'.\n- 'run.googleapis.com/launch-stage' sets the [launch stage](https://cloud.google.com/run/docs/troubleshooting#launch-stage-validation)\n  when a preview feature is used. For example, '\"run.googleapis.com/launch-stage\": \"BETA\"'\n- 'run.googleapis.com/minScale' sets the [minimum number of container instances](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--min) of the Service.\n- 'run.googleapis.com/scalingMode' sets the type of scaling mode for the service. The supported values for scaling mode are \"manual\" and \"automatic\". If not provided, it defaults to \"automatic\".\n- 'run.googleapis.com/manualInstanceCount' sets the total instance count for the service in manual scaling mode. This number of instances is divided among all revisions with specified traffic based on the percent of traffic they are receiving.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn set_annotations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nMap of string keys and values that can be used to organize and categorize\n(scope and select) objects. May match selectors of replication controllers\nand routes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `namespace`.\nIn Cloud Run the namespace must be equal to either the\nproject ID or project number."]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Metadata {
    type O = terrars::BlockAssignable<CloudRunService_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Metadata {}

impl BuildCloudRunService_Metadata {
    pub fn build(self) -> CloudRunService_Metadata {
        CloudRunService_Metadata {
            annotations: core::default::Default::default(),
            labels: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_MetadataRef {
        CloudRunService_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations is a key value map stored with a resource that\nmay be set by external tools to store and retrieve arbitrary metadata.\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations\n\n**Note**: The Cloud Run API may add additional annotations that were not provided in your config.\nIf terraform plan shows a diff where a server-side annotation is added, you can add it to your config\nor apply the lifecycle.ignore_changes rule to the metadata.0.annotations field.\n\nAnnotations with 'run.googleapis.com/' and 'autoscaling.knative.dev' are restricted. Use the following annotation\nkeys to configure features on a Service:\n\n- 'run.googleapis.com/binary-authorization-breakglass' sets the [Binary Authorization breakglass](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--breakglass).\n- 'run.googleapis.com/binary-authorization' sets the [Binary Authorization](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--binary-authorization).\n- 'run.googleapis.com/client-name' sets the client name calling the Cloud Run API.\n- 'run.googleapis.com/custom-audiences' sets the [custom audiences](https://cloud.google.com/sdk/gcloud/reference/alpha/run/deploy#--add-custom-audiences)\n  that can be used in the audience field of ID token for authenticated requests.\n- 'run.googleapis.com/description' sets a user defined description for the Service.\n- 'run.googleapis.com/ingress' sets the [ingress settings](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--ingress)\n  for the Service. For example, '\"run.googleapis.com/ingress\" = \"all\"'.\n- 'run.googleapis.com/launch-stage' sets the [launch stage](https://cloud.google.com/run/docs/troubleshooting#launch-stage-validation)\n  when a preview feature is used. For example, '\"run.googleapis.com/launch-stage\": \"BETA\"'\n- 'run.googleapis.com/minScale' sets the [minimum number of container instances](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--min) of the Service.\n- 'run.googleapis.com/scalingMode' sets the type of scaling mode for the service. The supported values for scaling mode are \"manual\" and \"automatic\". If not provided, it defaults to \"automatic\".\n- 'run.googleapis.com/manualInstanceCount' sets the total instance count for the service in manual scaling mode. This number of instances is divided among all revisions with specified traffic based on the percent of traffic they are receiving.\n\n**Note**: This field is non-authoritative, and will only manage the annotations present in your configuration.\nPlease refer to the field 'effective_annotations' for all of the annotations present on the resource."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `effective_annotations` after provisioning.\nAll of annotations (key/value pairs) present on the resource in GCP, including the annotations configured through Terraform, other clients and services."]
    pub fn effective_annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_annotations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nA sequence number representing a specific generation of the desired state."]
    pub fn generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nMap of string keys and values that can be used to organize and categorize\n(scope and select) objects. May match selectors of replication controllers\nand routes.\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nIn Cloud Run the namespace must be equal to either the\nproject ID or project number."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_version` after provisioning.\nAn opaque value that represents the internal version of this object that\ncan be used by clients to determine when objects have changed. May be used\nfor optimistic concurrency, change detection, and the watch operation on a\nresource or set of resources. They may only be valid for a\nparticular resource or set of resources."]
    pub fn resource_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nSelfLink is a URL representing this object."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nUID is a unique id generated by the server on successful creation of a resource and is not\nallowed to change on PUT operations."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    annotations: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Template_Metadata {
    #[doc =
        "Set the field `annotations`.\nAnnotations is a key value map stored with a resource that\nmay be set by external tools to store and retrieve arbitrary metadata.\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations\n\n**Note**: The Cloud Run API may add additional annotations that were not provided in your config.\nIf terraform plan shows a diff where a server-side annotation is added, you can add it to your config\nor apply the lifecycle.ignore_changes rule to the metadata.0.annotations field.\n\nAnnotations with 'run.googleapis.com/' and 'autoscaling.knative.dev' are restricted. Use the following annotation\nkeys to configure features on a Revision template:\n\n- 'autoscaling.knative.dev/maxScale' sets the [maximum number of container\n  instances](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--max-instances) of the Revision to run.\n- 'autoscaling.knative.dev/minScale' sets the [minimum number of container\n  instances](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--min-instances) of the Revision to run.\n- 'run.googleapis.com/client-name' sets the client name calling the Cloud Run API.\n- 'run.googleapis.com/cloudsql-instances' sets the [Cloud SQL\n  instances](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--add-cloudsql-instances) the Revision connects to.\n- 'run.googleapis.com/cpu-throttling' sets whether to throttle the CPU when the container is not actively serving\n  requests. See https://cloud.google.com/sdk/gcloud/reference/run/deploy#--[no-]cpu-throttling.\n- 'run.googleapis.com/encryption-key-shutdown-hours' sets the number of hours to wait before an automatic shutdown\n  server after CMEK key revocation is detected.\n- 'run.googleapis.com/encryption-key' sets the [CMEK key](https://cloud.google.com/run/docs/securing/using-cmek)\n  reference to encrypt the container with.\n- 'run.googleapis.com/execution-environment' sets the [execution\n  environment](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--execution-environment)\n  where the application will run.\n- 'run.googleapis.com/post-key-revocation-action-type' sets the\n  [action type](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--post-key-revocation-action-type)\n  after CMEK key revocation.\n- 'run.googleapis.com/secrets' sets a list of key-value pairs to set as\n  [secrets](https://cloud.google.com/run/docs/configuring/secrets#yaml).\n- 'run.googleapis.com/sessionAffinity' sets whether to enable\n  [session affinity](https://cloud.google.com/sdk/gcloud/reference/beta/run/deploy#--[no-]session-affinity)\n  for connections to the Revision.\n- 'run.googleapis.com/startup-cpu-boost' sets whether to allocate extra CPU to containers on startup.\n  See https://cloud.google.com/sdk/gcloud/reference/run/deploy#--[no-]cpu-boost.\n- 'run.googleapis.com/network-interfaces' sets [Direct VPC egress](https://cloud.google.com/run/docs/configuring/vpc-direct-vpc#yaml)\n  for the Revision.\n- 'run.googleapis.com/vpc-access-connector' sets a [VPC connector](https://cloud.google.com/run/docs/configuring/connecting-vpc#terraform_1)\n  for the Revision.\n- 'run.googleapis.com/vpc-access-egress' sets the outbound traffic to send through the VPC connector for this resource.\n  See https://cloud.google.com/sdk/gcloud/reference/run/deploy#--vpc-egress.\n- 'run.googleapis.com/gpu-zonal-redundancy-disabled' sets\n  [GPU zonal redundancy](https://cloud.google.com/run/docs/configuring/services/gpu-zonal-redundancy) for the Revision.\n- 'run.googleapis.com/health-check-disabled' disabled health checking containers during deployment."]
    pub fn set_annotations(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.annotations = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nMap of string keys and values that can be used to organize and categorize\n(scope and select) objects."]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nName must be unique within a Google Cloud project and region.\nIs required when creating resources. Name is primarily intended\nfor creation idempotence and configuration definition. Cannot be updated."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `namespace`.\nIn Cloud Run the namespace must be equal to either the\nproject ID or project number. It will default to the resource's project."]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Metadata {
    type O = terrars::BlockAssignable<CloudRunService_Template_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Metadata {}

impl BuildCloudRunService_Template_Metadata {
    pub fn build(self) -> CloudRunService_Template_Metadata {
        CloudRunService_Template_Metadata {
            annotations: core::default::Default::default(),
            labels: core::default::Default::default(),
            name: core::default::Default::default(),
            namespace: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_MetadataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_Template_MetadataRef {
        CloudRunService_Template_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `annotations` after provisioning.\nAnnotations is a key value map stored with a resource that\nmay be set by external tools to store and retrieve arbitrary metadata.\nMore info: https://kubernetes.io/docs/concepts/overview/working-with-objects/annotations\n\n**Note**: The Cloud Run API may add additional annotations that were not provided in your config.\nIf terraform plan shows a diff where a server-side annotation is added, you can add it to your config\nor apply the lifecycle.ignore_changes rule to the metadata.0.annotations field.\n\nAnnotations with 'run.googleapis.com/' and 'autoscaling.knative.dev' are restricted. Use the following annotation\nkeys to configure features on a Revision template:\n\n- 'autoscaling.knative.dev/maxScale' sets the [maximum number of container\n  instances](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--max-instances) of the Revision to run.\n- 'autoscaling.knative.dev/minScale' sets the [minimum number of container\n  instances](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--min-instances) of the Revision to run.\n- 'run.googleapis.com/client-name' sets the client name calling the Cloud Run API.\n- 'run.googleapis.com/cloudsql-instances' sets the [Cloud SQL\n  instances](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--add-cloudsql-instances) the Revision connects to.\n- 'run.googleapis.com/cpu-throttling' sets whether to throttle the CPU when the container is not actively serving\n  requests. See https://cloud.google.com/sdk/gcloud/reference/run/deploy#--[no-]cpu-throttling.\n- 'run.googleapis.com/encryption-key-shutdown-hours' sets the number of hours to wait before an automatic shutdown\n  server after CMEK key revocation is detected.\n- 'run.googleapis.com/encryption-key' sets the [CMEK key](https://cloud.google.com/run/docs/securing/using-cmek)\n  reference to encrypt the container with.\n- 'run.googleapis.com/execution-environment' sets the [execution\n  environment](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--execution-environment)\n  where the application will run.\n- 'run.googleapis.com/post-key-revocation-action-type' sets the\n  [action type](https://cloud.google.com/sdk/gcloud/reference/run/deploy#--post-key-revocation-action-type)\n  after CMEK key revocation.\n- 'run.googleapis.com/secrets' sets a list of key-value pairs to set as\n  [secrets](https://cloud.google.com/run/docs/configuring/secrets#yaml).\n- 'run.googleapis.com/sessionAffinity' sets whether to enable\n  [session affinity](https://cloud.google.com/sdk/gcloud/reference/beta/run/deploy#--[no-]session-affinity)\n  for connections to the Revision.\n- 'run.googleapis.com/startup-cpu-boost' sets whether to allocate extra CPU to containers on startup.\n  See https://cloud.google.com/sdk/gcloud/reference/run/deploy#--[no-]cpu-boost.\n- 'run.googleapis.com/network-interfaces' sets [Direct VPC egress](https://cloud.google.com/run/docs/configuring/vpc-direct-vpc#yaml)\n  for the Revision.\n- 'run.googleapis.com/vpc-access-connector' sets a [VPC connector](https://cloud.google.com/run/docs/configuring/connecting-vpc#terraform_1)\n  for the Revision.\n- 'run.googleapis.com/vpc-access-egress' sets the outbound traffic to send through the VPC connector for this resource.\n  See https://cloud.google.com/sdk/gcloud/reference/run/deploy#--vpc-egress.\n- 'run.googleapis.com/gpu-zonal-redundancy-disabled' sets\n  [GPU zonal redundancy](https://cloud.google.com/run/docs/configuring/services/gpu-zonal-redundancy) for the Revision.\n- 'run.googleapis.com/health-check-disabled' disabled health checking containers during deployment."]
    pub fn annotations(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.annotations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `generation` after provisioning.\nA sequence number representing a specific generation of the desired state."]
    pub fn generation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nMap of string keys and values that can be used to organize and categorize\n(scope and select) objects."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nName must be unique within a Google Cloud project and region.\nIs required when creating resources. Name is primarily intended\nfor creation idempotence and configuration definition. Cannot be updated."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `namespace` after provisioning.\nIn Cloud Run the namespace must be equal to either the\nproject ID or project number. It will default to the resource's project."]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }

    #[doc =
        "Get a reference to the value of field `resource_version` after provisioning.\nAn opaque value that represents the internal version of this object that\ncan be used by clients to determine when objects have changed. May be used\nfor optimistic concurrency, change detection, and the watch operation on a\nresource or set of resources. They may only be valid for a\nparticular resource or set of resources."]
    pub fn resource_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_version", self.base))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nSelfLink is a URL representing this object."]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.base))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nUID is a unique id generated by the server on successful creation of a resource and is not\nallowed to change on PUT operations."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRef {
    key: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
}

impl CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRef { }

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRef {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRef {
    #[doc =
        "A Cloud Secret Manager secret version. Must be 'latest' for the latest\nversion or an integer for a specific version."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc =
        "The name of the secret in Cloud Secret Manager. By default, the secret is assumed to be in the same project.\nIf the secret is in another project, you must define an alias.\nAn alias definition has the form:\n{alias}:projects/{project-id|project-number}/secrets/{secret-name}.\nIf multiple alias definitions are needed, they must be separated by commas.\nThe alias definitions must be set on the run.googleapis.com/secrets annotation."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRef {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRef {
        CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRef {
            key: self.key,
            name: self.name,
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRefRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRefRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRefRef {
        CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRefRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRefRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nA Cloud Secret Manager secret version. Must be 'latest' for the latest\nversion or an integer for a specific version."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe name of the secret in Cloud Secret Manager. By default, the secret is assumed to be in the same project.\nIf the secret is in another project, you must define an alias.\nAn alias definition has the form:\n{alias}:projects/{project-id|project-number}/secrets/{secret-name}.\nIf multiple alias definitions are needed, they must be separated by commas.\nThe alias definitions must be set on the run.googleapis.com/secrets annotation."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_Containers_Env_ValueFromDynamic {
    secret_key_ref: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRef>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_Env_ValueFrom {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_key_ref: std
    ::option
    ::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRef>>,
    dynamic: CloudRunService_Template_Spec_Containers_Env_ValueFromDynamic,
}

impl CloudRunService_Template_Spec_Containers_Env_ValueFrom {
    #[doc = "Set the field `secret_key_ref`.\n"]
    pub fn set_secret_key_ref(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRef>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_key_ref = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_key_ref = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_Env_ValueFrom {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_Env_ValueFrom>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_Env_ValueFrom {}

impl BuildCloudRunService_Template_Spec_Containers_Env_ValueFrom {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_Env_ValueFrom {
        CloudRunService_Template_Spec_Containers_Env_ValueFrom {
            secret_key_ref: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_Env_ValueFromRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_Env_ValueFromRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_Env_ValueFromRef {
        CloudRunService_Template_Spec_Containers_Env_ValueFromRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_Env_ValueFromRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_key_ref` after provisioning.\n"]
    pub fn secret_key_ref(
        &self,
    ) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_Env_ValueFrom_SecretKeyRefRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_key_ref", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_Containers_EnvDynamic {
    value_from: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_Env_ValueFrom>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_Env {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value_from: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_Env_ValueFrom>>,
    dynamic: CloudRunService_Template_Spec_Containers_EnvDynamic,
}

impl CloudRunService_Template_Spec_Containers_Env {
    #[doc = "Set the field `name`.\nName of the environment variable."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\nDefaults to \"\"."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value_from`.\n"]
    pub fn set_value_from(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_Env_ValueFrom>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value_from = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_Env {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_Env>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_Env {}

impl BuildCloudRunService_Template_Spec_Containers_Env {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_Env {
        CloudRunService_Template_Spec_Containers_Env {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
            value_from: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_EnvRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_EnvRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_EnvRef {
        CloudRunService_Template_Spec_Containers_EnvRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_EnvRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the environment variable."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nDefaults to \"\"."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `value_from` after provisioning.\n"]
    pub fn value_from(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_Env_ValueFromRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value_from", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference {
    name: terrars::PrimField<std::string::String>,
}

impl CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference { }

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference {
    type O =
        terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference {
    #[doc = "Name of the referent."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference {
        CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference { name: self.name }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReferenceRef {
        CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the referent."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRefDynamic {
    local_object_reference: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRef {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_object_reference: std
    ::option
    ::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference>>,
    dynamic: CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRefDynamic,
}

impl CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRef {
    #[doc = "Set the field `optional`.\nSpecify whether the ConfigMap must be defined"]
    pub fn set_optional(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.optional = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_object_reference`.\n"]
    pub fn set_local_object_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.local_object_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.local_object_reference = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRef {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRef {}

impl BuildCloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRef {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRef {
        CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRef {
            optional: core::default::Default::default(),
            local_object_reference: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRefRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRefRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRefRef {
        CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRefRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRefRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `optional` after provisioning.\nSpecify whether the ConfigMap must be defined"]
    pub fn optional(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optional", self.base))
    }

    #[doc = "Get a reference to the value of field `local_object_reference` after provisioning.\n"]
    pub fn local_object_reference(
        &self,
    ) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_object_reference", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_Containers_EnvFrom_SecretRefDynamic {
    local_object_reference: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    optional: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_object_reference: std
    ::option
    ::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference>>,
    dynamic: CloudRunService_Template_Spec_Containers_EnvFrom_SecretRefDynamic,
}

impl CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef {
    #[doc = "Set the field `optional`.\nSpecify whether the Secret must be defined"]
    pub fn set_optional(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.optional = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_object_reference`.\n"]
    pub fn set_local_object_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReference,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.local_object_reference = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.local_object_reference = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_EnvFrom_SecretRef {}

impl BuildCloudRunService_Template_Spec_Containers_EnvFrom_SecretRef {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef {
        CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef {
            optional: core::default::Default::default(),
            local_object_reference: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_EnvFrom_SecretRefRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_EnvFrom_SecretRefRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_EnvFrom_SecretRefRef {
        CloudRunService_Template_Spec_Containers_EnvFrom_SecretRefRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_EnvFrom_SecretRefRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `optional` after provisioning.\nSpecify whether the Secret must be defined"]
    pub fn optional(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.optional", self.base))
    }

    #[doc = "Get a reference to the value of field `local_object_reference` after provisioning.\n"]
    pub fn local_object_reference(
        &self,
    ) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef_LocalObjectReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_object_reference", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_Containers_EnvFromDynamic {
    config_map_ref: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRef>>,
    secret_ref: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_EnvFrom {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    config_map_ref: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRef>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_ref: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef>>,
    dynamic: CloudRunService_Template_Spec_Containers_EnvFromDynamic,
}

impl CloudRunService_Template_Spec_Containers_EnvFrom {
    #[doc = "Set the field `prefix`.\nAn optional identifier to prepend to each key in the ConfigMap."]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `config_map_ref`.\n"]
    pub fn set_config_map_ref(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRef>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.config_map_ref = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.config_map_ref = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret_ref`.\n"]
    pub fn set_secret_ref(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRef>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret_ref = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret_ref = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_EnvFrom {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_EnvFrom>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_EnvFrom {}

impl BuildCloudRunService_Template_Spec_Containers_EnvFrom {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_EnvFrom {
        CloudRunService_Template_Spec_Containers_EnvFrom {
            prefix: core::default::Default::default(),
            config_map_ref: core::default::Default::default(),
            secret_ref: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_EnvFromRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_EnvFromRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_EnvFromRef {
        CloudRunService_Template_Spec_Containers_EnvFromRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_EnvFromRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `prefix` after provisioning.\nAn optional identifier to prepend to each key in the ConfigMap."]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `config_map_ref` after provisioning.\n"]
    pub fn config_map_ref(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_EnvFrom_ConfigMapRefRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.config_map_ref", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_ref` after provisioning.\n"]
    pub fn secret_ref(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_EnvFrom_SecretRefRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret_ref", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_StartupProbe_Grpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Template_Spec_Containers_StartupProbe_Grpc {
    #[doc =
        "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service`.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn set_service(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_StartupProbe_Grpc {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe_Grpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_StartupProbe_Grpc {}

impl BuildCloudRunService_Template_Spec_Containers_StartupProbe_Grpc {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_StartupProbe_Grpc {
        CloudRunService_Template_Spec_Containers_StartupProbe_Grpc {
            port: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_StartupProbe_GrpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_StartupProbe_GrpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_StartupProbe_GrpcRef {
        CloudRunService_Template_Spec_Containers_StartupProbe_GrpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_StartupProbe_GrpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service` after provisioning.\nThe name of the service to place in the gRPC HealthCheckRequest\n(see https://github.com/grpc/grpc/blob/master/doc/health-checking.md).\nIf this is not specified, the default behavior is defined by gRPC."]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeaders {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeaders {
    #[doc = "Set the field `value`.\nThe header field value."]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeaders {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeaders {
    #[doc = "The header field name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeaders {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeaders {
        CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeaders {
            name: self.name,
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeadersRef {
        CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe header field name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe header field value."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_Containers_StartupProbe_HttpGetDynamic {
    http_headers: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeaders>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_headers: std
    ::option
    ::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeaders>>,
    dynamic: CloudRunService_Template_Spec_Containers_StartupProbe_HttpGetDynamic,
}

impl CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet {
    #[doc = "Set the field `path`.\nPath to access on the HTTP server. If set, it should not be empty string."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_headers`.\n"]
    pub fn set_http_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeaders>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_StartupProbe_HttpGet {}

impl BuildCloudRunService_Template_Spec_Containers_StartupProbe_HttpGet {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet {
        CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            http_headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_StartupProbe_HttpGetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_StartupProbe_HttpGetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_StartupProbe_HttpGetRef {
        CloudRunService_Template_Spec_Containers_StartupProbe_HttpGetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_StartupProbe_HttpGetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server. If set, it should not be empty string."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `http_headers` after provisioning.\n"]
    pub fn http_headers(
        &self,
    ) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet_HttpHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_headers", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_Containers_LivenessProbeDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_StartupProbe_Grpc>>,
    http_get: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_LivenessProbe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_StartupProbe_Grpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet>>,
    dynamic: CloudRunService_Template_Spec_Containers_LivenessProbeDynamic,
}

impl CloudRunService_Template_Spec_Containers_LivenessProbe {
    #[doc =
        "Set the field `failure_threshold`.\nMinimum consecutive failures for the probe to be considered failed after\nhaving succeeded. Defaults to 3. Minimum value is 1."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_delay_seconds`.\nNumber of seconds after the container has started before the probe is\ninitiated.\nDefaults to 0 seconds. Minimum value is 0. Maximum value is 3600."]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe.\nDefault to 10 seconds. Minimum value is 1. Maximum value is 3600."]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out.\nDefaults to 1 second. Minimum value is 1. Maximum value is 3600.\nMust be smaller than period_seconds."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe_Grpc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_get = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_LivenessProbe {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_LivenessProbe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_LivenessProbe {}

impl BuildCloudRunService_Template_Spec_Containers_LivenessProbe {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_LivenessProbe {
        CloudRunService_Template_Spec_Containers_LivenessProbe {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_LivenessProbeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_LivenessProbeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_LivenessProbeRef {
        CloudRunService_Template_Spec_Containers_LivenessProbeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_LivenessProbeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nMinimum consecutive failures for the probe to be considered failed after\nhaving succeeded. Defaults to 3. Minimum value is 1."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds after the container has started before the probe is\ninitiated.\nDefaults to 0 seconds. Minimum value is 0. Maximum value is 3600."]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe.\nDefault to 10 seconds. Minimum value is 1. Maximum value is 3600."]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out.\nDefaults to 1 second. Minimum value is 1. Maximum value is 3600.\nMust be smaller than period_seconds."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_StartupProbe_GrpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_Ports {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Template_Spec_Containers_Ports {
    #[doc =
        "Set the field `container_port`.\nPort number the container listens on. This must be a valid port number (between 1 and 65535). Defaults to \"8080\"."]
    pub fn set_container_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_port = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `name`.\nIf specified, used to specify which protocol to use. Allowed values are \"http1\" (HTTP/1) and \"h2c\" (HTTP/2 end-to-end). Defaults to \"http1\"."]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\nProtocol for port. Must be \"TCP\". Defaults to \"TCP\"."]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_Ports {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_Ports>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_Ports {}

impl BuildCloudRunService_Template_Spec_Containers_Ports {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_Ports {
        CloudRunService_Template_Spec_Containers_Ports {
            container_port: core::default::Default::default(),
            name: core::default::Default::default(),
            protocol: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_PortsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_PortsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_PortsRef {
        CloudRunService_Template_Spec_Containers_PortsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_PortsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_port` after provisioning.\nPort number the container listens on. This must be a valid port number (between 1 and 65535). Defaults to \"8080\"."]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIf specified, used to specify which protocol to use. Allowed values are \"http1\" (HTTP/1) and \"h2c\" (HTTP/2 end-to-end). Defaults to \"http1\"."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `protocol` after provisioning.\nProtocol for port. Must be \"TCP\". Defaults to \"TCP\"."]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGet {
    #[doc = "Set the field `path`.\nPath to access on the HTTP server. If set, it should not be empty string."]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGet {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGet {}

impl BuildCloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGet {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGet {
        CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGet {
            path: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGetRef {
        CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nPath to access on the HTTP server. If set, it should not be empty string."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_Containers_ReadinessProbeDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_StartupProbe_Grpc>>,
    http_get: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGet>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_ReadinessProbe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_StartupProbe_Grpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGet>>,
    dynamic: CloudRunService_Template_Spec_Containers_ReadinessProbeDynamic,
}

impl CloudRunService_Template_Spec_Containers_ReadinessProbe {
    #[doc =
        "Set the field `failure_threshold`.\nMinimum consecutive failures for the probe to be considered failed after\nhaving succeeded. Defaults to 3."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe.\nDefault to 10 seconds."]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `success_threshold`.\nMinimum consecutive successes for the probe to be considered successful after having failed.\nDefaults to 2."]
    pub fn set_success_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.success_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out.\nDefaults to 1 second. Must be smaller than period_seconds."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe_Grpc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_get = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_ReadinessProbe {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_ReadinessProbe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_ReadinessProbe {}

impl BuildCloudRunService_Template_Spec_Containers_ReadinessProbe {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_ReadinessProbe {
        CloudRunService_Template_Spec_Containers_ReadinessProbe {
            failure_threshold: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            success_threshold: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_ReadinessProbeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_ReadinessProbeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_ReadinessProbeRef {
        CloudRunService_Template_Spec_Containers_ReadinessProbeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_ReadinessProbeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nMinimum consecutive failures for the probe to be considered failed after\nhaving succeeded. Defaults to 3."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe.\nDefault to 10 seconds."]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `success_threshold` after provisioning.\nMinimum consecutive successes for the probe to be considered successful after having failed.\nDefaults to 2."]
    pub fn success_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out.\nDefaults to 1 second. Must be smaller than period_seconds."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_StartupProbe_GrpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_ReadinessProbe_HttpGetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_Resources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limits: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requests: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl CloudRunService_Template_Spec_Containers_Resources {
    #[doc =
        "Set the field `limits`.\nLimits describes the maximum amount of compute resources allowed.\nCPU Limit details:\n- For fractional CPU values (e.g. '0.5', '0.75', min '0.08') are also supported.\n- CPU allocation must comply with memory limits and concurrency rules described in:\n  https://cloud.google.com/run/docs/configuring/services/cpu\nThe values of the map is string form of the 'quantity' k8s type:\nhttps://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn set_limits(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.limits = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `requests`.\nRequests describes the minimum amount of compute resources required.\nIf Requests is omitted for a container, it defaults to Limits if that is\nexplicitly specified, otherwise to an implementation-defined value.\nThe values of the map is string form of the 'quantity' k8s type:\nhttps://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn set_requests(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.requests = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_Resources {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_Resources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_Resources {}

impl BuildCloudRunService_Template_Spec_Containers_Resources {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_Resources {
        CloudRunService_Template_Spec_Containers_Resources {
            limits: core::default::Default::default(),
            requests: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_ResourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_ResourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_ResourcesRef {
        CloudRunService_Template_Spec_Containers_ResourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_ResourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `limits` after provisioning.\nLimits describes the maximum amount of compute resources allowed.\nCPU Limit details:\n- For fractional CPU values (e.g. '0.5', '0.75', min '0.08') are also supported.\n- CPU allocation must comply with memory limits and concurrency rules described in:\n  https://cloud.google.com/run/docs/configuring/services/cpu\nThe values of the map is string form of the 'quantity' k8s type:\nhttps://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn limits(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.limits", self.base))
    }

    #[doc =
        "Get a reference to the value of field `requests` after provisioning.\nRequests describes the minimum amount of compute resources required.\nIf Requests is omitted for a container, it defaults to Limits if that is\nexplicitly specified, otherwise to an implementation-defined value.\nThe values of the map is string form of the 'quantity' k8s type:\nhttps://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go"]
    pub fn requests(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.requests", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocket {
    #[doc =
        "Set the field `port`.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocket {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_StartupProbe_TcpSocket {}

impl BuildCloudRunService_Template_Spec_Containers_StartupProbe_TcpSocket {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocket {
        CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocket { port: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocketRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocketRef {
        CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `port` after provisioning.\nPort number to access on the container. Number must be in the range 1 to 65535.\nIf not specified, defaults to the same value as container.ports[0].containerPort."]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_Containers_StartupProbeDynamic {
    grpc: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_StartupProbe_Grpc>>,
    http_get: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet>>,
    tcp_socket: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocket>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_StartupProbe {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    failure_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_delay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_StartupProbe_Grpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_get: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_socket: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocket>>,
    dynamic: CloudRunService_Template_Spec_Containers_StartupProbeDynamic,
}

impl CloudRunService_Template_Spec_Containers_StartupProbe {
    #[doc =
        "Set the field `failure_threshold`.\nMinimum consecutive failures for the probe to be considered failed after\nhaving succeeded. Defaults to 3. Minimum value is 1."]
    pub fn set_failure_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.failure_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `initial_delay_seconds`.\nNumber of seconds after the container has started before the probe is\ninitiated.\nDefaults to 0 seconds. Minimum value is 0. Maximum value is 240."]
    pub fn set_initial_delay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.initial_delay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `period_seconds`.\nHow often (in seconds) to perform the probe.\nDefault to 10 seconds. Minimum value is 1. Maximum value is 240."]
    pub fn set_period_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.period_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nNumber of seconds after which the probe times out.\nDefaults to 1 second. Minimum value is 1. Maximum value is 3600.\nMust be smaller than periodSeconds."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `grpc`.\n"]
    pub fn set_grpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe_Grpc>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.grpc = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_get`.\n"]
    pub fn set_http_get(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_get = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_get = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tcp_socket`.\n"]
    pub fn set_tcp_socket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocket>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tcp_socket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tcp_socket = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_StartupProbe {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_StartupProbe {}

impl BuildCloudRunService_Template_Spec_Containers_StartupProbe {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_StartupProbe {
        CloudRunService_Template_Spec_Containers_StartupProbe {
            failure_threshold: core::default::Default::default(),
            initial_delay_seconds: core::default::Default::default(),
            period_seconds: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            grpc: core::default::Default::default(),
            http_get: core::default::Default::default(),
            tcp_socket: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_StartupProbeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_StartupProbeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_StartupProbeRef {
        CloudRunService_Template_Spec_Containers_StartupProbeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_StartupProbeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `failure_threshold` after provisioning.\nMinimum consecutive failures for the probe to be considered failed after\nhaving succeeded. Defaults to 3. Minimum value is 1."]
    pub fn failure_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc =
        "Get a reference to the value of field `initial_delay_seconds` after provisioning.\nNumber of seconds after the container has started before the probe is\ninitiated.\nDefaults to 0 seconds. Minimum value is 0. Maximum value is 240."]
    pub fn initial_delay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_delay_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `period_seconds` after provisioning.\nHow often (in seconds) to perform the probe.\nDefault to 10 seconds. Minimum value is 1. Maximum value is 240."]
    pub fn period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.period_seconds", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nNumber of seconds after which the probe times out.\nDefaults to 1 second. Minimum value is 1. Maximum value is 3600.\nMust be smaller than periodSeconds."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `grpc` after provisioning.\n"]
    pub fn grpc(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_StartupProbe_GrpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc", self.base))
    }

    #[doc = "Get a reference to the value of field `http_get` after provisioning.\n"]
    pub fn http_get(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_StartupProbe_HttpGetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_get", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_socket` after provisioning.\n"]
    pub fn tcp_socket(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_StartupProbe_TcpSocketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_socket", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers_VolumeMounts {
    mount_path: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_path: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Template_Spec_Containers_VolumeMounts {
    #[doc = "Set the field `sub_path`.\nPath within the volume from which the container's volume should be mounted."]
    pub fn set_sub_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sub_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers_VolumeMounts {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_VolumeMounts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers_VolumeMounts {
    #[doc = "Path within the container at which the volume should be mounted.  Must\nnot contain ':'."]
    pub mount_path: terrars::PrimField<std::string::String>,
    #[doc = "This must match the Name of a Volume."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunService_Template_Spec_Containers_VolumeMounts {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers_VolumeMounts {
        CloudRunService_Template_Spec_Containers_VolumeMounts {
            mount_path: self.mount_path,
            name: self.name,
            sub_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Containers_VolumeMountsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Containers_VolumeMountsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Containers_VolumeMountsRef {
        CloudRunService_Template_Spec_Containers_VolumeMountsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Containers_VolumeMountsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mount_path` after provisioning.\nPath within the container at which the volume should be mounted.  Must\nnot contain ':'."]
    pub fn mount_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nThis must match the Name of a Volume."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sub_path` after provisioning.\nPath within the volume from which the container's volume should be mounted."]
    pub fn sub_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sub_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_ContainersDynamic {
    env: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_Env>>,
    env_from: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_EnvFrom>>,
    liveness_probe: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_LivenessProbe>>,
    ports: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_Ports>>,
    readiness_probe: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_ReadinessProbe>>,
    resources: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_Resources>>,
    startup_probe: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_StartupProbe>>,
    volume_mounts: std
    ::option
    ::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers_VolumeMounts>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Containers {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sandbox_launcher: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    working_dir: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_Env>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    env_from: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_EnvFrom>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    liveness_probe: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_LivenessProbe>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_Ports>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    readiness_probe: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_ReadinessProbe>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_Resources>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_probe: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_StartupProbe>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_mounts: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers_VolumeMounts>>,
    dynamic: CloudRunService_Template_Spec_ContainersDynamic,
}

impl CloudRunService_Template_Spec_Containers {
    #[doc =
        "Set the field `args`.\nArguments to the entrypoint.\nThe docker image's CMD is used if this is not provided."]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `command`.\nEntrypoint array. Not executed within a shell.\nThe docker image's ENTRYPOINT is used if this is not provided."]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the container"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `sandbox_launcher`.\nIndicates that this container can act as a sandbox supervisor and launch sandboxes."]
    pub fn set_sandbox_launcher(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.sandbox_launcher = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `working_dir`.\nContainer's working directory.\nIf not specified, the container runtime's default will be used, which\nmight be configured in the container image."]
    pub fn set_working_dir(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.working_dir = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `env`.\n"]
    pub fn set_env(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_Env>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `env_from`.\n"]
    pub fn set_env_from(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_EnvFrom>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.env_from = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.env_from = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `liveness_probe`.\n"]
    pub fn set_liveness_probe(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_LivenessProbe>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.liveness_probe = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.liveness_probe = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_Ports>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ports = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ports = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `readiness_probe`.\n"]
    pub fn set_readiness_probe(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_ReadinessProbe>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.readiness_probe = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.readiness_probe = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_Resources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `startup_probe`.\n"]
    pub fn set_startup_probe(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_StartupProbe>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.startup_probe = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.startup_probe = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volume_mounts`.\n"]
    pub fn set_volume_mounts(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers_VolumeMounts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.volume_mounts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.volume_mounts = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Containers {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Containers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Containers {
    #[doc =
        "Docker image name. This is most often a reference to a container located\nin the container registry, such as gcr.io/cloudrun/hello"]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunService_Template_Spec_Containers {
    pub fn build(self) -> CloudRunService_Template_Spec_Containers {
        CloudRunService_Template_Spec_Containers {
            args: core::default::Default::default(),
            command: core::default::Default::default(),
            image: self.image,
            name: core::default::Default::default(),
            sandbox_launcher: core::default::Default::default(),
            working_dir: core::default::Default::default(),
            env: core::default::Default::default(),
            env_from: core::default::Default::default(),
            liveness_probe: core::default::Default::default(),
            ports: core::default::Default::default(),
            readiness_probe: core::default::Default::default(),
            resources: core::default::Default::default(),
            startup_probe: core::default::Default::default(),
            volume_mounts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_ContainersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_ContainersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_Template_Spec_ContainersRef {
        CloudRunService_Template_Spec_ContainersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_ContainersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `args` after provisioning.\nArguments to the entrypoint.\nThe docker image's CMD is used if this is not provided."]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc =
        "Get a reference to the value of field `command` after provisioning.\nEntrypoint array. Not executed within a shell.\nThe docker image's ENTRYPOINT is used if this is not provided."]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc =
        "Get a reference to the value of field `image` after provisioning.\nDocker image name. This is most often a reference to a container located\nin the container registry, such as gcr.io/cloudrun/hello"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the container"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `sandbox_launcher` after provisioning.\nIndicates that this container can act as a sandbox supervisor and launch sandboxes."]
    pub fn sandbox_launcher(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sandbox_launcher", self.base))
    }

    #[doc =
        "Get a reference to the value of field `working_dir` after provisioning.\nContainer's working directory.\nIf not specified, the container runtime's default will be used, which\nmight be configured in the container image."]
    pub fn working_dir(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.working_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `env_from` after provisioning.\n"]
    pub fn env_from(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_EnvFromRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.env_from", self.base))
    }

    #[doc = "Get a reference to the value of field `liveness_probe` after provisioning.\n"]
    pub fn liveness_probe(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_LivenessProbeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.liveness_probe", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_PortsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }

    #[doc = "Get a reference to the value of field `readiness_probe` after provisioning.\n"]
    pub fn readiness_probe(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_ReadinessProbeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.readiness_probe", self.base))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_ResourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resources", self.base))
    }

    #[doc = "Get a reference to the value of field `startup_probe` after provisioning.\n"]
    pub fn startup_probe(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_StartupProbeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.startup_probe", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_mounts` after provisioning.\n"]
    pub fn volume_mounts(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Containers_VolumeMountsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volume_mounts", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Volumes_Csi {
    driver: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_attributes: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl CloudRunService_Template_Spec_Volumes_Csi {
    #[doc = "Set the field `read_only`.\nIf true, all mounts created from this volume will be read-only."]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `volume_attributes`.\nDriver-specific attributes. The following options are supported for available drivers:\n  * gcsfuse.run.googleapis.com\n    * bucketName: The name of the Cloud Storage Bucket that backs this volume. The Cloud Run Service identity must have access to this bucket."]
    pub fn set_volume_attributes(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.volume_attributes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Volumes_Csi {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes_Csi>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Volumes_Csi {
    #[doc =
        "Unique name representing the type of file system to be created. Cloud Run supports the following values:\n  * gcsfuse.run.googleapis.com: Mount a Google Cloud Storage bucket using GCSFuse. This driver requires the\n    run.googleapis.com/execution-environment annotation to be unset or set to \"gen2\""]
    pub driver: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunService_Template_Spec_Volumes_Csi {
    pub fn build(self) -> CloudRunService_Template_Spec_Volumes_Csi {
        CloudRunService_Template_Spec_Volumes_Csi {
            driver: self.driver,
            read_only: core::default::Default::default(),
            volume_attributes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Volumes_CsiRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Volumes_CsiRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_Template_Spec_Volumes_CsiRef {
        CloudRunService_Template_Spec_Volumes_CsiRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Volumes_CsiRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `driver` after provisioning.\nUnique name representing the type of file system to be created. Cloud Run supports the following values:\n  * gcsfuse.run.googleapis.com: Mount a Google Cloud Storage bucket using GCSFuse. This driver requires the\n    run.googleapis.com/execution-environment annotation to be unset or set to \"gen2\""]
    pub fn driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIf true, all mounts created from this volume will be read-only."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `volume_attributes` after provisioning.\nDriver-specific attributes. The following options are supported for available drivers:\n  * gcsfuse.run.googleapis.com\n    * bucketName: The name of the Cloud Storage Bucket that backs this volume. The Cloud Run Service identity must have access to this bucket."]
    pub fn volume_attributes(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.volume_attributes", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Volumes_EmptyDir {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    medium: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_limit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Template_Spec_Volumes_EmptyDir {
    #[doc =
        "Set the field `medium`.\nThe medium on which the data is stored. The default is \"\" which means to use the node's default medium. Must be an empty string (default) or Memory."]
    pub fn set_medium(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.medium = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `size_limit`.\nLimit on the storage usable by this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. This field's values are of the 'Quantity' k8s type: https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir."]
    pub fn set_size_limit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.size_limit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Volumes_EmptyDir {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes_EmptyDir>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Volumes_EmptyDir {}

impl BuildCloudRunService_Template_Spec_Volumes_EmptyDir {
    pub fn build(self) -> CloudRunService_Template_Spec_Volumes_EmptyDir {
        CloudRunService_Template_Spec_Volumes_EmptyDir {
            medium: core::default::Default::default(),
            size_limit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Volumes_EmptyDirRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Volumes_EmptyDirRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Volumes_EmptyDirRef {
        CloudRunService_Template_Spec_Volumes_EmptyDirRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Volumes_EmptyDirRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `medium` after provisioning.\nThe medium on which the data is stored. The default is \"\" which means to use the node's default medium. Must be an empty string (default) or Memory."]
    pub fn medium(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.medium", self.base))
    }

    #[doc =
        "Get a reference to the value of field `size_limit` after provisioning.\nLimit on the storage usable by this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. This field's values are of the 'Quantity' k8s type: https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/quantity/. The default is nil which means that the limit is undefined. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir."]
    pub fn size_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Volumes_Nfs {
    path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    server: terrars::PrimField<std::string::String>,
}

impl CloudRunService_Template_Spec_Volumes_Nfs {
    #[doc = "Set the field `read_only`.\nIf true, mount the NFS volume as read only in all mounts. Defaults to false."]
    pub fn set_read_only(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.read_only = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Volumes_Nfs {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes_Nfs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Volumes_Nfs {
    #[doc = "Path exported by the NFS server"]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = "IP address or hostname of the NFS server"]
    pub server: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunService_Template_Spec_Volumes_Nfs {
    pub fn build(self) -> CloudRunService_Template_Spec_Volumes_Nfs {
        CloudRunService_Template_Spec_Volumes_Nfs {
            path: self.path,
            read_only: core::default::Default::default(),
            server: self.server,
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Volumes_NfsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Volumes_NfsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_Template_Spec_Volumes_NfsRef {
        CloudRunService_Template_Spec_Volumes_NfsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Volumes_NfsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nPath exported by the NFS server"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc =
        "Get a reference to the value of field `read_only` after provisioning.\nIf true, mount the NFS volume as read only in all mounts. Defaults to false."]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.base))
    }

    #[doc =
        "Get a reference to the value of field `server` after provisioning.\nIP address or hostname of the NFS server"]
    pub fn server(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.server", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Volumes_Secret_Items {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<f64>>,
    path: terrars::PrimField<std::string::String>,
}

impl CloudRunService_Template_Spec_Volumes_Secret_Items {
    #[doc =
        "Set the field `mode`.\nMode bits to use on this file, must be a value between 0000 and 0777. If\nnot specified, the volume defaultMode will be used. This might be in\nconflict with other options that affect the file mode, like fsGroup, and\nthe result can be other mode bits set."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Volumes_Secret_Items {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes_Secret_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Volumes_Secret_Items {
    #[doc =
        "The Cloud Secret Manager secret version.\nCan be 'latest' for the latest value or an integer for a specific version."]
    pub key: terrars::PrimField<std::string::String>,
    #[doc =
        "The relative path of the file to map the key to.\nMay not be an absolute path.\nMay not contain the path element '..'.\nMay not start with the string '..'."]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunService_Template_Spec_Volumes_Secret_Items {
    pub fn build(self) -> CloudRunService_Template_Spec_Volumes_Secret_Items {
        CloudRunService_Template_Spec_Volumes_Secret_Items {
            key: self.key,
            mode: core::default::Default::default(),
            path: self.path,
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Volumes_Secret_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Volumes_Secret_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Volumes_Secret_ItemsRef {
        CloudRunService_Template_Spec_Volumes_Secret_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Volumes_Secret_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `key` after provisioning.\nThe Cloud Secret Manager secret version.\nCan be 'latest' for the latest value or an integer for a specific version."]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nMode bits to use on this file, must be a value between 0000 and 0777. If\nnot specified, the volume defaultMode will be used. This might be in\nconflict with other options that affect the file mode, like fsGroup, and\nthe result can be other mode bits set."]
    pub fn mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `path` after provisioning.\nThe relative path of the file to map the key to.\nMay not be an absolute path.\nMay not contain the path element '..'.\nMay not start with the string '..'."]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_Volumes_SecretDynamic {
    items: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Volumes_Secret_Items>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Volumes_Secret {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_mode: std::option::Option<terrars::PrimField<f64>>,
    secret_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Volumes_Secret_Items>>,
    dynamic: CloudRunService_Template_Spec_Volumes_SecretDynamic,
}

impl CloudRunService_Template_Spec_Volumes_Secret {
    #[doc =
        "Set the field `default_mode`.\nMode bits to use on created files by default. Must be a value between 0000\nand 0777. Defaults to 0644. Directories within the path are not affected by\nthis setting. This might be in conflict with other options that affect the\nfile mode, like fsGroup, and the result can be other mode bits set."]
    pub fn set_default_mode(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.default_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes_Secret_Items>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.items = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.items = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Volumes_Secret {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes_Secret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Volumes_Secret {
    #[doc =
        "The name of the secret in Cloud Secret Manager. By default, the secret\nis assumed to be in the same project.\nIf the secret is in another project, you must define an alias.\nAn alias definition has the form:\n{alias}:projects/{project-id|project-number}/secrets/{secret-name}.\nIf multiple alias definitions are needed, they must be separated by\ncommas.\nThe alias definitions must be set on the run.googleapis.com/secrets\nannotation."]
    pub secret_name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunService_Template_Spec_Volumes_Secret {
    pub fn build(self) -> CloudRunService_Template_Spec_Volumes_Secret {
        CloudRunService_Template_Spec_Volumes_Secret {
            default_mode: core::default::Default::default(),
            secret_name: self.secret_name,
            items: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_Volumes_SecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_Volumes_SecretRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudRunService_Template_Spec_Volumes_SecretRef {
        CloudRunService_Template_Spec_Volumes_SecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_Volumes_SecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `default_mode` after provisioning.\nMode bits to use on created files by default. Must be a value between 0000\nand 0777. Defaults to 0644. Directories within the path are not affected by\nthis setting. This might be in conflict with other options that affect the\nfile mode, like fsGroup, and the result can be other mode bits set."]
    pub fn default_mode(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_mode", self.base))
    }

    #[doc =
        "Get a reference to the value of field `secret_name` after provisioning.\nThe name of the secret in Cloud Secret Manager. By default, the secret\nis assumed to be in the same project.\nIf the secret is in another project, you must define an alias.\nAn alias definition has the form:\n{alias}:projects/{project-id|project-number}/secrets/{secret-name}.\nIf multiple alias definitions are needed, they must be separated by\ncommas.\nThe alias definitions must be set on the run.googleapis.com/secrets\nannotation."]
    pub fn secret_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_name", self.base))
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Volumes_Secret_ItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_Spec_VolumesDynamic {
    csi: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Volumes_Csi>>,
    empty_dir: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Volumes_EmptyDir>>,
    nfs: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Volumes_Nfs>>,
    secret: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Volumes_Secret>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec_Volumes {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    csi: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Volumes_Csi>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    empty_dir: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Volumes_EmptyDir>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Volumes_Nfs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Volumes_Secret>>,
    dynamic: CloudRunService_Template_Spec_VolumesDynamic,
}

impl CloudRunService_Template_Spec_Volumes {
    #[doc = "Set the field `csi`.\n"]
    pub fn set_csi(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes_Csi>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.csi = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.csi = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `empty_dir`.\n"]
    pub fn set_empty_dir(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes_EmptyDir>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.empty_dir = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.empty_dir = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes_Nfs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.nfs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.nfs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes_Secret>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec_Volumes {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec_Volumes {
    #[doc = "Volume's name."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudRunService_Template_Spec_Volumes {
    pub fn build(self) -> CloudRunService_Template_Spec_Volumes {
        CloudRunService_Template_Spec_Volumes {
            name: self.name,
            csi: core::default::Default::default(),
            empty_dir: core::default::Default::default(),
            nfs: core::default::Default::default(),
            secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_Spec_VolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_Spec_VolumesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_Template_Spec_VolumesRef {
        CloudRunService_Template_Spec_VolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_Spec_VolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nVolume's name."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `csi` after provisioning.\n"]
    pub fn csi(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Volumes_CsiRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.csi", self.base))
    }

    #[doc = "Get a reference to the value of field `empty_dir` after provisioning.\n"]
    pub fn empty_dir(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Volumes_EmptyDirRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.empty_dir", self.base))
    }

    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Volumes_NfsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<CloudRunService_Template_Spec_Volumes_SecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_Template_SpecDynamic {
    containers: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Containers>>,
    volumes: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec_Volumes>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_concurrency: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_selector: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    containers: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Containers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec_Volumes>>,
    dynamic: CloudRunService_Template_SpecDynamic,
}

impl CloudRunService_Template_Spec {
    #[doc =
        "Set the field `container_concurrency`.\nContainerConcurrency specifies the maximum allowed in-flight (concurrent)\nrequests per container of the Revision. If not specified or 0, defaults to 80 when\nrequested CPU >= 1 and defaults to 1 when requested CPU < 1."]
    pub fn set_container_concurrency(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_concurrency = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `node_selector`.\nNode Selector describes the hardware requirements of the resources.\nUse the following node selector keys to configure features on a Revision:\n  - 'run.googleapis.com/accelerator' sets the [type of GPU](https://cloud.google.com/run/docs/configuring/services/gpu) required by the Revision to run."]
    pub fn set_node_selector(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.node_selector = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `service_account_name`.\nEmail address of the IAM service account associated with the revision of the\nservice. The service account represents the identity of the running revision,\nand determines what permissions the revision has. If not provided, the revision\nwill use the project's default service account."]
    pub fn set_service_account_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_account_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `timeout_seconds`.\nTimeoutSeconds holds the max duration the instance is allowed for responding to a request."]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `containers`.\n"]
    pub fn set_containers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Containers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.containers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.containers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volumes`.\n"]
    pub fn set_volumes(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec_Volumes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.volumes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.volumes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template_Spec {
    type O = terrars::BlockAssignable<CloudRunService_Template_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template_Spec {}

impl BuildCloudRunService_Template_Spec {
    pub fn build(self) -> CloudRunService_Template_Spec {
        CloudRunService_Template_Spec {
            container_concurrency: core::default::Default::default(),
            node_selector: core::default::Default::default(),
            service_account_name: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            containers: core::default::Default::default(),
            volumes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_Template_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_Template_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_Template_SpecRef {
        CloudRunService_Template_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_Template_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_concurrency` after provisioning.\nContainerConcurrency specifies the maximum allowed in-flight (concurrent)\nrequests per container of the Revision. If not specified or 0, defaults to 80 when\nrequested CPU >= 1 and defaults to 1 when requested CPU < 1."]
    pub fn container_concurrency(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_concurrency", self.base))
    }

    #[doc =
        "Get a reference to the value of field `node_selector` after provisioning.\nNode Selector describes the hardware requirements of the resources.\nUse the following node selector keys to configure features on a Revision:\n  - 'run.googleapis.com/accelerator' sets the [type of GPU](https://cloud.google.com/run/docs/configuring/services/gpu) required by the Revision to run."]
    pub fn node_selector(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.node_selector", self.base))
    }

    #[doc =
        "Get a reference to the value of field `service_account_name` after provisioning.\nEmail address of the IAM service account associated with the revision of the\nservice. The service account represents the identity of the running revision,\nand determines what permissions the revision has. If not provided, the revision\nwill use the project's default service account."]
    pub fn service_account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_account_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `serving_state` after provisioning.\nServingState holds a value describing the state the resources\nare in for this Revision.\nIt is expected\nthat the system will manipulate this based on routability and load."]
    pub fn serving_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serving_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `timeout_seconds` after provisioning.\nTimeoutSeconds holds the max duration the instance is allowed for responding to a request."]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `containers` after provisioning.\n"]
    pub fn containers(&self) -> terrars::ListRef<CloudRunService_Template_Spec_ContainersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.containers", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes` after provisioning.\n"]
    pub fn volumes(&self) -> terrars::ListRef<CloudRunService_Template_Spec_VolumesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.volumes", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudRunService_TemplateDynamic {
    metadata: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Metadata>>,
    spec: std::option::Option<terrars::DynamicBlock<CloudRunService_Template_Spec>>,
}

#[derive(Serialize)]
pub struct CloudRunService_Template {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<std::vec::Vec<CloudRunService_Template_Metadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<CloudRunService_Template_Spec>>,
    dynamic: CloudRunService_TemplateDynamic,
}

impl CloudRunService_Template {
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Metadata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudRunService_Template_Spec>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Template {
    type O = terrars::BlockAssignable<CloudRunService_Template>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Template {}

impl BuildCloudRunService_Template {
    pub fn build(self) -> CloudRunService_Template {
        CloudRunService_Template {
            metadata: core::default::Default::default(),
            spec: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_TemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_TemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_TemplateRef {
        CloudRunService_TemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_TemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::ListRef<CloudRunService_Template_MetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<CloudRunService_Template_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudRunService_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Timeouts {
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

impl terrars::ToListMappable for CloudRunService_Timeouts {
    type O = terrars::BlockAssignable<CloudRunService_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Timeouts {}

impl BuildCloudRunService_Timeouts {
    pub fn build(self) -> CloudRunService_Timeouts {
        CloudRunService_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_TimeoutsRef {
        CloudRunService_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_TimeoutsRef {
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
pub struct CloudRunService_Traffic {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latest_revision: std::option::Option<terrars::PrimField<bool>>,
    percent: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revision_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudRunService_Traffic {
    #[doc =
        "Set the field `latest_revision`.\nLatestRevision may be optionally provided to indicate that the latest ready\nRevision of the Configuration should be used for this traffic target. When\nprovided LatestRevision must be true if RevisionName is empty; it must be\nfalse when RevisionName is non-empty."]
    pub fn set_latest_revision(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.latest_revision = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `revision_name`.\nRevisionName of a specific revision to which to send this portion of traffic."]
    pub fn set_revision_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.revision_name = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `tag`.\nTag is optionally used to expose a dedicated url for referencing this target exclusively."]
    pub fn set_tag(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tag = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudRunService_Traffic {
    type O = terrars::BlockAssignable<CloudRunService_Traffic>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudRunService_Traffic {
    #[doc = "Percent specifies percent of the traffic to this Revision or Configuration."]
    pub percent: terrars::PrimField<f64>,
}

impl BuildCloudRunService_Traffic {
    pub fn build(self) -> CloudRunService_Traffic {
        CloudRunService_Traffic {
            latest_revision: core::default::Default::default(),
            percent: self.percent,
            revision_name: core::default::Default::default(),
            tag: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudRunService_TrafficRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudRunService_TrafficRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudRunService_TrafficRef {
        CloudRunService_TrafficRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudRunService_TrafficRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `latest_revision` after provisioning.\nLatestRevision may be optionally provided to indicate that the latest ready\nRevision of the Configuration should be used for this traffic target. When\nprovided LatestRevision must be true if RevisionName is empty; it must be\nfalse when RevisionName is non-empty."]
    pub fn latest_revision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_revision", self.base))
    }

    #[doc =
        "Get a reference to the value of field `percent` after provisioning.\nPercent specifies percent of the traffic to this Revision or Configuration."]
    pub fn percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percent", self.base))
    }

    #[doc =
        "Get a reference to the value of field `revision_name` after provisioning.\nRevisionName of a specific revision to which to send this portion of traffic."]
    pub fn revision_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `tag` after provisioning.\nTag is optionally used to expose a dedicated url for referencing this target exclusively."]
    pub fn tag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag", self.base))
    }

    #[doc =
        "Get a reference to the value of field `url` after provisioning.\nURL displays the URL for accessing tagged traffic targets. URL is displayed in status,\nand is disallowed on spec. URL must contain a scheme (e.g. http://) and a hostname,\nbut may not contain anything else (e.g. basic auth, url path, etc.)"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}
