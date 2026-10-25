#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ApphubServiceData_ {
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
    application_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    discovered_service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    service_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attributes: std::option::Option<std::vec::Vec<ApphubService_Attributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ApphubService_Timeouts>,
    dynamic: ApphubServiceDynamic,
}

struct ApphubService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ApphubServiceData_>,
}

#[derive(Clone)]
pub struct ApphubService(std::rc::Rc<ApphubService_>);

impl ApphubService {
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

    #[doc = "Set the field `description`.\nUser-defined description of a Service."]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\nUser-defined name for the Service."]
    pub fn set_display_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().display_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `attributes`.\n"]
    pub fn set_attributes(self, v: impl std::convert::Into<terrars::BlockAssignable<ApphubService_Attributes>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ApphubService_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `application_id` after provisioning.\nPart of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-defined description of a Service."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovered_service` after provisioning.\nImmutable. The resource name of the original discovered service."]
    pub fn discovered_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovered_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-defined name for the Service."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of a Service. Format:\n\"projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nThe Service identifier."]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_properties` after provisioning.\nProperties of an underlying cloud resource that can comprise a Service."]
    pub fn service_properties(&self) -> terrars::ListRef<ApphubService_ServicePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_reference` after provisioning.\nReference to an underlying networking resource that can comprise a Service."]
    pub fn service_reference(&self) -> terrars::ListRef<ApphubService_ServiceReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_reference", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. Service state. Possible values: STATE_UNSPECIFIED CREATING ACTIVE DELETING DETACHED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. A universally unique identifier (UUID) for the 'Service' in the UUID4\nformat."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<ApphubService_AttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApphubService_TimeoutsRef {
        ApphubService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ApphubService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ApphubService { }

impl terrars::ToListMappable for ApphubService {
    type O = terrars::ListRef<ApphubServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ApphubService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_apphub_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApphubService {
    pub tf_id: std::string::String,
    #[doc =
        "Part of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub application_id: terrars::PrimField<std::string::String>,
    #[doc = "Immutable. The resource name of the original discovered service."]
    pub discovered_service: terrars::PrimField<std::string::String>,
    #[doc =
        "Part of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "The Service identifier."]
    pub service_id: terrars::PrimField<std::string::String>,
}

impl BuildApphubService {
    pub fn build(self, stack: &mut terrars::Stack) -> ApphubService {
        let out = ApphubService(std::rc::Rc::new(ApphubService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ApphubServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                application_id: self.application_id,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                discovered_service: self.discovered_service,
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                service_id: self.service_id,
                attributes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApphubServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApphubServiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `application_id` after provisioning.\nPart of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Create time."]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nUser-defined description of a Service."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `discovered_service` after provisioning.\nImmutable. The resource name of the original discovered service."]
    pub fn discovered_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovered_service", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nUser-defined name for the Service."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'.  Full resource name of a parent Application. Example: projects/{HOST_PROJECT_ID}/locations/{LOCATION}/applications/{APPLICATION_ID}"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of a Service. Format:\n\"projects/{host-project-id}/locations/{location}/applications/{application-id}/services/{service-id}\""]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\nThe Service identifier."]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_properties` after provisioning.\nProperties of an underlying cloud resource that can comprise a Service."]
    pub fn service_properties(&self) -> terrars::ListRef<ApphubService_ServicePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_properties", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_reference` after provisioning.\nReference to an underlying networking resource that can comprise a Service."]
    pub fn service_reference(&self) -> terrars::ListRef<ApphubService_ServiceReferenceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_reference", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `state` after provisioning.\nOutput only. Service state. Possible values: STATE_UNSPECIFIED CREATING ACTIVE DELETING DETACHED"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `uid` after provisioning.\nOutput only. A universally unique identifier (UUID) for the 'Service' in the UUID4\nformat."]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Update time."]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<ApphubService_AttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ApphubService_TimeoutsRef {
        ApphubService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ApphubServiceDynamic {
    attributes: std::option::Option<terrars::DynamicBlock<ApphubService_Attributes>>,
}

#[derive(Serialize)]
pub struct ApphubService_ServiceProperties_ExtendedMetadata_Value {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_metadata_schema: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_struct: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApphubService_ServiceProperties_ExtendedMetadata_Value {
    #[doc = "Set the field `extended_metadata_schema`.\n"]
    pub fn set_extended_metadata_schema(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.extended_metadata_schema = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata_struct`.\n"]
    pub fn set_metadata_struct(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metadata_struct = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApphubService_ServiceProperties_ExtendedMetadata_Value {
    type O = terrars::BlockAssignable<ApphubService_ServiceProperties_ExtendedMetadata_Value>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_ServiceProperties_ExtendedMetadata_Value {}

impl BuildApphubService_ServiceProperties_ExtendedMetadata_Value {
    pub fn build(self) -> ApphubService_ServiceProperties_ExtendedMetadata_Value {
        ApphubService_ServiceProperties_ExtendedMetadata_Value {
            extended_metadata_schema: core::default::Default::default(),
            metadata_struct: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApphubService_ServiceProperties_ExtendedMetadata_ValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_ServiceProperties_ExtendedMetadata_ValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApphubService_ServiceProperties_ExtendedMetadata_ValueRef {
        ApphubService_ServiceProperties_ExtendedMetadata_ValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_ServiceProperties_ExtendedMetadata_ValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `extended_metadata_schema` after provisioning.\n"]
    pub fn extended_metadata_schema(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extended_metadata_schema", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata_struct` after provisioning.\n"]
    pub fn metadata_struct(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metadata_struct", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_ServiceProperties_ExtendedMetadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::ListField<ApphubService_ServiceProperties_ExtendedMetadata_Value>>,
}

impl ApphubService_ServiceProperties_ExtendedMetadata {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ApphubService_ServiceProperties_ExtendedMetadata_Value>>,
    ) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApphubService_ServiceProperties_ExtendedMetadata {
    type O = terrars::BlockAssignable<ApphubService_ServiceProperties_ExtendedMetadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_ServiceProperties_ExtendedMetadata {}

impl BuildApphubService_ServiceProperties_ExtendedMetadata {
    pub fn build(self) -> ApphubService_ServiceProperties_ExtendedMetadata {
        ApphubService_ServiceProperties_ExtendedMetadata {
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApphubService_ServiceProperties_ExtendedMetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_ServiceProperties_ExtendedMetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApphubService_ServiceProperties_ExtendedMetadataRef {
        ApphubService_ServiceProperties_ExtendedMetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_ServiceProperties_ExtendedMetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::ListRef<ApphubService_ServiceProperties_ExtendedMetadata_ValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_ServiceProperties_FunctionalType {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApphubService_ServiceProperties_FunctionalType {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApphubService_ServiceProperties_FunctionalType {
    type O = terrars::BlockAssignable<ApphubService_ServiceProperties_FunctionalType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_ServiceProperties_FunctionalType {}

impl BuildApphubService_ServiceProperties_FunctionalType {
    pub fn build(self) -> ApphubService_ServiceProperties_FunctionalType {
        ApphubService_ServiceProperties_FunctionalType { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApphubService_ServiceProperties_FunctionalTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_ServiceProperties_FunctionalTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApphubService_ServiceProperties_FunctionalTypeRef {
        ApphubService_ServiceProperties_FunctionalTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_ServiceProperties_FunctionalTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_ServiceProperties_Identity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApphubService_ServiceProperties_Identity {
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.principal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApphubService_ServiceProperties_Identity {
    type O = terrars::BlockAssignable<ApphubService_ServiceProperties_Identity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_ServiceProperties_Identity {}

impl BuildApphubService_ServiceProperties_Identity {
    pub fn build(self) -> ApphubService_ServiceProperties_Identity {
        ApphubService_ServiceProperties_Identity { principal: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApphubService_ServiceProperties_IdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_ServiceProperties_IdentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApphubService_ServiceProperties_IdentityRef {
        ApphubService_ServiceProperties_IdentityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_ServiceProperties_IdentityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_ServiceProperties_RegistrationType {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApphubService_ServiceProperties_RegistrationType {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApphubService_ServiceProperties_RegistrationType {
    type O = terrars::BlockAssignable<ApphubService_ServiceProperties_RegistrationType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_ServiceProperties_RegistrationType {}

impl BuildApphubService_ServiceProperties_RegistrationType {
    pub fn build(self) -> ApphubService_ServiceProperties_RegistrationType {
        ApphubService_ServiceProperties_RegistrationType { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApphubService_ServiceProperties_RegistrationTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_ServiceProperties_RegistrationTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ApphubService_ServiceProperties_RegistrationTypeRef {
        ApphubService_ServiceProperties_RegistrationTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_ServiceProperties_RegistrationTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_ServiceProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extended_metadata: std::option::Option<terrars::ListField<ApphubService_ServiceProperties_ExtendedMetadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    functional_type: std::option::Option<terrars::ListField<ApphubService_ServiceProperties_FunctionalType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcp_project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity: std::option::Option<terrars::ListField<ApphubService_ServiceProperties_Identity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registration_type: std::option::Option<terrars::ListField<ApphubService_ServiceProperties_RegistrationType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApphubService_ServiceProperties {
    #[doc = "Set the field `extended_metadata`.\n"]
    pub fn set_extended_metadata(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ApphubService_ServiceProperties_ExtendedMetadata>>,
    ) -> Self {
        self.extended_metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `functional_type`.\n"]
    pub fn set_functional_type(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ApphubService_ServiceProperties_FunctionalType>>,
    ) -> Self {
        self.functional_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gcp_project`.\n"]
    pub fn set_gcp_project(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcp_project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `identity`.\n"]
    pub fn set_identity(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ApphubService_ServiceProperties_Identity>>,
    ) -> Self {
        self.identity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registration_type`.\n"]
    pub fn set_registration_type(
        mut self,
        v: impl std::convert::Into<terrars::ListField<ApphubService_ServiceProperties_RegistrationType>>,
    ) -> Self {
        self.registration_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApphubService_ServiceProperties {
    type O = terrars::BlockAssignable<ApphubService_ServiceProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_ServiceProperties {}

impl BuildApphubService_ServiceProperties {
    pub fn build(self) -> ApphubService_ServiceProperties {
        ApphubService_ServiceProperties {
            extended_metadata: core::default::Default::default(),
            functional_type: core::default::Default::default(),
            gcp_project: core::default::Default::default(),
            identity: core::default::Default::default(),
            location: core::default::Default::default(),
            registration_type: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApphubService_ServicePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_ServicePropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApphubService_ServicePropertiesRef {
        ApphubService_ServicePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_ServicePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `extended_metadata` after provisioning.\n"]
    pub fn extended_metadata(&self) -> terrars::ListRef<ApphubService_ServiceProperties_ExtendedMetadataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.extended_metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `functional_type` after provisioning.\n"]
    pub fn functional_type(&self) -> terrars::ListRef<ApphubService_ServiceProperties_FunctionalTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.functional_type", self.base))
    }

    #[doc = "Get a reference to the value of field `gcp_project` after provisioning.\n"]
    pub fn gcp_project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcp_project", self.base))
    }

    #[doc = "Get a reference to the value of field `identity` after provisioning.\n"]
    pub fn identity(&self) -> terrars::ListRef<ApphubService_ServiceProperties_IdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `registration_type` after provisioning.\n"]
    pub fn registration_type(&self) -> terrars::ListRef<ApphubService_ServiceProperties_RegistrationTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registration_type", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_ServiceReference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApphubService_ServiceReference {
    #[doc = "Set the field `uri`.\n"]
    pub fn set_uri(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uri = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApphubService_ServiceReference {
    type O = terrars::BlockAssignable<ApphubService_ServiceReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_ServiceReference {}

impl BuildApphubService_ServiceReference {
    pub fn build(self) -> ApphubService_ServiceReference {
        ApphubService_ServiceReference { uri: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ApphubService_ServiceReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_ServiceReferenceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApphubService_ServiceReferenceRef {
        ApphubService_ServiceReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_ServiceReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_Attributes_BusinessOwners {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    email: terrars::PrimField<std::string::String>,
}

impl ApphubService_Attributes_BusinessOwners {
    #[doc = "Set the field `display_name`.\nContact's name."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApphubService_Attributes_BusinessOwners {
    type O = terrars::BlockAssignable<ApphubService_Attributes_BusinessOwners>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_Attributes_BusinessOwners {
    #[doc = "Required. Email address of the contacts."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildApphubService_Attributes_BusinessOwners {
    pub fn build(self) -> ApphubService_Attributes_BusinessOwners {
        ApphubService_Attributes_BusinessOwners {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}

#[allow(dead_code)]
pub struct ApphubService_Attributes_BusinessOwnersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_Attributes_BusinessOwnersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApphubService_Attributes_BusinessOwnersRef {
        ApphubService_Attributes_BusinessOwnersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_Attributes_BusinessOwnersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nContact's name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contacts."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_Attributes_Criticality {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ApphubService_Attributes_Criticality { }

impl terrars::ToListMappable for ApphubService_Attributes_Criticality {
    type O = terrars::BlockAssignable<ApphubService_Attributes_Criticality>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_Attributes_Criticality {
    #[doc = "Criticality type. Possible values: [\"MISSION_CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildApphubService_Attributes_Criticality {
    pub fn build(self) -> ApphubService_Attributes_Criticality {
        ApphubService_Attributes_Criticality { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct ApphubService_Attributes_CriticalityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_Attributes_CriticalityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApphubService_Attributes_CriticalityRef {
        ApphubService_Attributes_CriticalityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_Attributes_CriticalityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nCriticality type. Possible values: [\"MISSION_CRITICAL\", \"HIGH\", \"MEDIUM\", \"LOW\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_Attributes_DeveloperOwners {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    email: terrars::PrimField<std::string::String>,
}

impl ApphubService_Attributes_DeveloperOwners {
    #[doc = "Set the field `display_name`.\nContact's name."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApphubService_Attributes_DeveloperOwners {
    type O = terrars::BlockAssignable<ApphubService_Attributes_DeveloperOwners>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_Attributes_DeveloperOwners {
    #[doc = "Required. Email address of the contacts."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildApphubService_Attributes_DeveloperOwners {
    pub fn build(self) -> ApphubService_Attributes_DeveloperOwners {
        ApphubService_Attributes_DeveloperOwners {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}

#[allow(dead_code)]
pub struct ApphubService_Attributes_DeveloperOwnersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_Attributes_DeveloperOwnersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApphubService_Attributes_DeveloperOwnersRef {
        ApphubService_Attributes_DeveloperOwnersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_Attributes_DeveloperOwnersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nContact's name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contacts."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_Attributes_Environment {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl ApphubService_Attributes_Environment { }

impl terrars::ToListMappable for ApphubService_Attributes_Environment {
    type O = terrars::BlockAssignable<ApphubService_Attributes_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_Attributes_Environment {
    #[doc = "Environment type. Possible values: [\"PRODUCTION\", \"STAGING\", \"TEST\", \"DEVELOPMENT\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildApphubService_Attributes_Environment {
    pub fn build(self) -> ApphubService_Attributes_Environment {
        ApphubService_Attributes_Environment { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct ApphubService_Attributes_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_Attributes_EnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApphubService_Attributes_EnvironmentRef {
        ApphubService_Attributes_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_Attributes_EnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nEnvironment type. Possible values: [\"PRODUCTION\", \"STAGING\", \"TEST\", \"DEVELOPMENT\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_Attributes_OperatorOwners {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    email: terrars::PrimField<std::string::String>,
}

impl ApphubService_Attributes_OperatorOwners {
    #[doc = "Set the field `display_name`.\nContact's name."]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ApphubService_Attributes_OperatorOwners {
    type O = terrars::BlockAssignable<ApphubService_Attributes_OperatorOwners>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_Attributes_OperatorOwners {
    #[doc = "Required. Email address of the contacts."]
    pub email: terrars::PrimField<std::string::String>,
}

impl BuildApphubService_Attributes_OperatorOwners {
    pub fn build(self) -> ApphubService_Attributes_OperatorOwners {
        ApphubService_Attributes_OperatorOwners {
            display_name: core::default::Default::default(),
            email: self.email,
        }
    }
}

#[allow(dead_code)]
pub struct ApphubService_Attributes_OperatorOwnersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_Attributes_OperatorOwnersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApphubService_Attributes_OperatorOwnersRef {
        ApphubService_Attributes_OperatorOwnersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_Attributes_OperatorOwnersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nContact's name."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `email` after provisioning.\nRequired. Email address of the contacts."]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }
}

#[derive(Serialize, Default)]
struct ApphubService_AttributesDynamic {
    business_owners: std::option::Option<terrars::DynamicBlock<ApphubService_Attributes_BusinessOwners>>,
    criticality: std::option::Option<terrars::DynamicBlock<ApphubService_Attributes_Criticality>>,
    developer_owners: std::option::Option<terrars::DynamicBlock<ApphubService_Attributes_DeveloperOwners>>,
    environment: std::option::Option<terrars::DynamicBlock<ApphubService_Attributes_Environment>>,
    operator_owners: std::option::Option<terrars::DynamicBlock<ApphubService_Attributes_OperatorOwners>>,
}

#[derive(Serialize)]
pub struct ApphubService_Attributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    business_owners: std::option::Option<std::vec::Vec<ApphubService_Attributes_BusinessOwners>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    criticality: std::option::Option<std::vec::Vec<ApphubService_Attributes_Criticality>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    developer_owners: std::option::Option<std::vec::Vec<ApphubService_Attributes_DeveloperOwners>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<std::vec::Vec<ApphubService_Attributes_Environment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator_owners: std::option::Option<std::vec::Vec<ApphubService_Attributes_OperatorOwners>>,
    dynamic: ApphubService_AttributesDynamic,
}

impl ApphubService_Attributes {
    #[doc = "Set the field `business_owners`.\n"]
    pub fn set_business_owners(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApphubService_Attributes_BusinessOwners>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.business_owners = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.business_owners = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApphubService_Attributes_Criticality>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.criticality = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.criticality = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `developer_owners`.\n"]
    pub fn set_developer_owners(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApphubService_Attributes_DeveloperOwners>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.developer_owners = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.developer_owners = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApphubService_Attributes_Environment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `operator_owners`.\n"]
    pub fn set_operator_owners(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<ApphubService_Attributes_OperatorOwners>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.operator_owners = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.operator_owners = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ApphubService_Attributes {
    type O = terrars::BlockAssignable<ApphubService_Attributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_Attributes {}

impl BuildApphubService_Attributes {
    pub fn build(self) -> ApphubService_Attributes {
        ApphubService_Attributes {
            business_owners: core::default::Default::default(),
            criticality: core::default::Default::default(),
            developer_owners: core::default::Default::default(),
            environment: core::default::Default::default(),
            operator_owners: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApphubService_AttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_AttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApphubService_AttributesRef {
        ApphubService_AttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_AttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `business_owners` after provisioning.\n"]
    pub fn business_owners(&self) -> terrars::ListRef<ApphubService_Attributes_BusinessOwnersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.business_owners", self.base))
    }

    #[doc = "Get a reference to the value of field `criticality` after provisioning.\n"]
    pub fn criticality(&self) -> terrars::ListRef<ApphubService_Attributes_CriticalityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.criticality", self.base))
    }

    #[doc = "Get a reference to the value of field `developer_owners` after provisioning.\n"]
    pub fn developer_owners(&self) -> terrars::ListRef<ApphubService_Attributes_DeveloperOwnersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.developer_owners", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<ApphubService_Attributes_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `operator_owners` after provisioning.\n"]
    pub fn operator_owners(&self) -> terrars::ListRef<ApphubService_Attributes_OperatorOwnersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.operator_owners", self.base))
    }
}

#[derive(Serialize)]
pub struct ApphubService_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ApphubService_Timeouts {
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

impl terrars::ToListMappable for ApphubService_Timeouts {
    type O = terrars::BlockAssignable<ApphubService_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApphubService_Timeouts {}

impl BuildApphubService_Timeouts {
    pub fn build(self) -> ApphubService_Timeouts {
        ApphubService_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ApphubService_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ApphubService_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ApphubService_TimeoutsRef {
        ApphubService_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ApphubService_TimeoutsRef {
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
