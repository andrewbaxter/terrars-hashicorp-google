#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleEnvironmentData_ {
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
    aliases_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base64_image: std::option::Option<terrars::PrimField<std::string::String>>,
    contact: terrars::PrimField<std::string::String>,
    contact_emails: terrars::PrimField<std::string::String>,
    contact_phone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_access_scopes_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_protection: std::option::Option<terrars::PrimField<bool>>,
    description: terrars::PrimField<std::string::String>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parallel_instance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    retention_duration: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dynamic_parameters: std::option::Option<std::vec::Vec<ChronicleEnvironment_DynamicParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleEnvironment_Timeouts>,
    dynamic: ChronicleEnvironmentDynamic,
}

struct ChronicleEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleEnvironmentData_>,
}

#[derive(Clone)]
pub struct ChronicleEnvironment(std::rc::Rc<ChronicleEnvironment_>);

impl ChronicleEnvironment {
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

    #[doc = "Set the field `aliases_json`.\nEnvironment nicknames."]
    pub fn set_aliases_json(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().aliases_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `base64_image`.\nEnvironment icon."]
    pub fn set_base64_image(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().base64_image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_access_scopes_json`.\ndata access scopes."]
    pub fn set_data_access_scopes_json(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().data_access_scopes_json = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_protection`.\nWhether Terraform will be prevented from destroying the environment. Deleting an environment will remove all its data and all playbooks, environments, integrations instances, reports and agents related to the environment. Once you delete an environment, it cannot be reversed. Deleting environments via terraform destroy or terraform apply will only succeed if this field is false in the Terraform state."]
    pub fn set_deletion_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().deletion_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `instance_uri`.\nURL of the environment. Used to route UI links to the correct SIEM instance\nwhen making cross-SecOps requests from SOAR."]
    pub fn set_instance_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `parallel_instance`.\nThe optional parallel SIEM instance used as a data source. Used to route\nAPI requests to the correct SIEM instance when making cross-SecOps requests\nfrom SOAR. For most customers, this is not required, since the parent\ninstance is used as the data source by default.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}"]
    pub fn set_parallel_instance(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().parallel_instance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `weight`.\nThe weight of the environment, enabling customers to control distribution\nof resources between the separate environments in a single instance of\nChronicle SOAR."]
    pub fn set_weight(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().weight = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dynamic_parameters`.\n"]
    pub fn set_dynamic_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleEnvironment_DynamicParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dynamic_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dynamic_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleEnvironment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `aliases_json` after provisioning.\nEnvironment nicknames."]
    pub fn aliases_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aliases_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base64_image` after provisioning.\nEnvironment icon."]
    pub fn base64_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base64_image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `contact` after provisioning.\nMAX_NAME_LENGTH = 256\nName of the contact for the environment."]
    pub fn contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `contact_emails` after provisioning.\nMAX_NAME_LENGTH = 256\nEmail of the contact for the environment. Multiple emails can be sepereated with the ';' character."]
    pub fn contact_emails(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_emails", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `contact_phone` after provisioning.\nMAX_NAME_LENGTH = 256\nPhone number of the contact for the environment."]
    pub fn contact_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_phone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_access_scopes_json` after provisioning.\ndata access scopes."]
    pub fn data_access_scopes_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_access_scopes_json", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the environment. Deleting an environment will remove all its data and all playbooks, environments, integrations instances, reports and agents related to the environment. Once you delete an environment, it cannot be reversed. Deleting environments via terraform destroy or terraform apply will only succeed if this field is false in the Terraform state."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nMAX_NAME_LENGTH = 256\nDescription of the environment."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nName of the environment\nMAX_NAME_LENGTH = 256"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\nId of the environment record."]
    pub fn environment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_id", self.extract_ref()))
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
        "Get a reference to the value of field `instance_uri` after provisioning.\nURL of the environment. Used to route UI links to the correct SIEM instance\nwhen making cross-SecOps requests from SOAR."]
    pub fn instance_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique name(ID) of the Environment.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/environments/{environment}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parallel_instance` after provisioning.\nThe optional parallel SIEM instance used as a data source. Used to route\nAPI requests to the correct SIEM instance when making cross-SecOps requests\nfrom SOAR. For most customers, this is not required, since the parent\ninstance is used as the data source by default.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}"]
    pub fn parallel_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallel_instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_duration` after provisioning.\nEnvironment data retention in months."]
    pub fn retention_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nThe weight of the environment, enabling customers to control distribution\nof resources between the separate environments in a single instance of\nChronicle SOAR."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamic_parameters` after provisioning.\n"]
    pub fn dynamic_parameters(&self) -> terrars::ListRef<ChronicleEnvironment_DynamicParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamic_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleEnvironment_TimeoutsRef {
        ChronicleEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleEnvironment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleEnvironment { }

impl terrars::ToListMappable for ChronicleEnvironment {
    type O = terrars::ListRef<ChronicleEnvironmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleEnvironment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_environment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleEnvironment {
    pub tf_id: std::string::String,
    #[doc = "MAX_NAME_LENGTH = 256\nName of the contact for the environment."]
    pub contact: terrars::PrimField<std::string::String>,
    #[doc =
        "MAX_NAME_LENGTH = 256\nEmail of the contact for the environment. Multiple emails can be sepereated with the ';' character."]
    pub contact_emails: terrars::PrimField<std::string::String>,
    #[doc = "MAX_NAME_LENGTH = 256\nPhone number of the contact for the environment."]
    pub contact_phone: terrars::PrimField<std::string::String>,
    #[doc = "MAX_NAME_LENGTH = 256\nDescription of the environment."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = "Name of the environment\nMAX_NAME_LENGTH = 256"]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
    #[doc = "Environment data retention in months."]
    pub retention_duration: terrars::PrimField<f64>,
}

impl BuildChronicleEnvironment {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleEnvironment {
        let out = ChronicleEnvironment(std::rc::Rc::new(ChronicleEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleEnvironmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                aliases_json: core::default::Default::default(),
                base64_image: core::default::Default::default(),
                contact: self.contact,
                contact_emails: self.contact_emails,
                contact_phone: self.contact_phone,
                data_access_scopes_json: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                deletion_protection: core::default::Default::default(),
                description: self.description,
                display_name: self.display_name,
                id: core::default::Default::default(),
                instance: self.instance,
                instance_uri: core::default::Default::default(),
                location: self.location,
                parallel_instance: core::default::Default::default(),
                project: core::default::Default::default(),
                retention_duration: self.retention_duration,
                weight: core::default::Default::default(),
                dynamic_parameters: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleEnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleEnvironmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aliases_json` after provisioning.\nEnvironment nicknames."]
    pub fn aliases_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aliases_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `base64_image` after provisioning.\nEnvironment icon."]
    pub fn base64_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base64_image", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `contact` after provisioning.\nMAX_NAME_LENGTH = 256\nName of the contact for the environment."]
    pub fn contact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `contact_emails` after provisioning.\nMAX_NAME_LENGTH = 256\nEmail of the contact for the environment. Multiple emails can be sepereated with the ';' character."]
    pub fn contact_emails(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_emails", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `contact_phone` after provisioning.\nMAX_NAME_LENGTH = 256\nPhone number of the contact for the environment."]
    pub fn contact_phone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contact_phone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_access_scopes_json` after provisioning.\ndata access scopes."]
    pub fn data_access_scopes_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_access_scopes_json", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `deletion_protection` after provisioning.\nWhether Terraform will be prevented from destroying the environment. Deleting an environment will remove all its data and all playbooks, environments, integrations instances, reports and agents related to the environment. Once you delete an environment, it cannot be reversed. Deleting environments via terraform destroy or terraform apply will only succeed if this field is false in the Terraform state."]
    pub fn deletion_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_protection", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nMAX_NAME_LENGTH = 256\nDescription of the environment."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nName of the environment\nMAX_NAME_LENGTH = 256"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\nId of the environment record."]
    pub fn environment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_id", self.extract_ref()))
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
        "Get a reference to the value of field `instance_uri` after provisioning.\nURL of the environment. Used to route UI links to the correct SIEM instance\nwhen making cross-SecOps requests from SOAR."]
    pub fn instance_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_uri", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The unique name(ID) of the Environment.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/environments/{environment}"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `parallel_instance` after provisioning.\nThe optional parallel SIEM instance used as a data source. Used to route\nAPI requests to the correct SIEM instance when making cross-SecOps requests\nfrom SOAR. For most customers, this is not required, since the parent\ninstance is used as the data source by default.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}"]
    pub fn parallel_instance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parallel_instance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_duration` after provisioning.\nEnvironment data retention in months."]
    pub fn retention_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_duration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `weight` after provisioning.\nThe weight of the environment, enabling customers to control distribution\nof resources between the separate environments in a single instance of\nChronicle SOAR."]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dynamic_parameters` after provisioning.\n"]
    pub fn dynamic_parameters(&self) -> terrars::ListRef<ChronicleEnvironment_DynamicParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dynamic_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleEnvironment_TimeoutsRef {
        ChronicleEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleEnvironmentDynamic {
    dynamic_parameters: std::option::Option<terrars::DynamicBlock<ChronicleEnvironment_DynamicParameters>>,
}

#[derive(Serialize)]
pub struct ChronicleEnvironment_DynamicParameters {
    dynamic_parameter_id: terrars::PrimField<f64>,
    value: terrars::PrimField<std::string::String>,
}

impl ChronicleEnvironment_DynamicParameters { }

impl terrars::ToListMappable for ChronicleEnvironment_DynamicParameters {
    type O = terrars::BlockAssignable<ChronicleEnvironment_DynamicParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleEnvironment_DynamicParameters {
    #[doc = "The ID of the dynamic parameter."]
    pub dynamic_parameter_id: terrars::PrimField<f64>,
    #[doc = "The value of the dynamic parameter."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildChronicleEnvironment_DynamicParameters {
    pub fn build(self) -> ChronicleEnvironment_DynamicParameters {
        ChronicleEnvironment_DynamicParameters {
            dynamic_parameter_id: self.dynamic_parameter_id,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleEnvironment_DynamicParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleEnvironment_DynamicParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleEnvironment_DynamicParametersRef {
        ChronicleEnvironment_DynamicParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleEnvironment_DynamicParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dynamic_parameter_id` after provisioning.\nThe ID of the dynamic parameter."]
    pub fn dynamic_parameter_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dynamic_parameter_id", self.base))
    }

    #[doc = "Get a reference to the value of field `environment_id` after provisioning.\nThe ID of the environment."]
    pub fn environment_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nThe value of the dynamic parameter."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleEnvironment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleEnvironment_Timeouts {
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

impl terrars::ToListMappable for ChronicleEnvironment_Timeouts {
    type O = terrars::BlockAssignable<ChronicleEnvironment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleEnvironment_Timeouts {}

impl BuildChronicleEnvironment_Timeouts {
    pub fn build(self) -> ChronicleEnvironment_Timeouts {
        ChronicleEnvironment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleEnvironment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleEnvironment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleEnvironment_TimeoutsRef {
        ChronicleEnvironment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleEnvironment_TimeoutsRef {
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
