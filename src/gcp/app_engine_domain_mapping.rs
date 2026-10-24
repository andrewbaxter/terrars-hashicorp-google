use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct AppEngineDomainMappingData {
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
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_settings: std::option::Option<std::vec::Vec<AppEngineDomainMappingSslSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AppEngineDomainMappingTimeoutsEl>,
    dynamic: AppEngineDomainMappingDynamic,
}

struct AppEngineDomainMapping_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppEngineDomainMappingData>,
}

#[derive(Clone)]
pub struct AppEngineDomainMapping(std::rc::Rc<AppEngineDomainMapping_>);

impl AppEngineDomainMapping {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `override_strategy`.\nWhether the domain creation should override any existing mappings for this domain.\nBy default, overrides are rejected. Default value: \"STRICT\" Possible values: [\"STRICT\", \"OVERRIDE\"]"]
    pub fn set_override_strategy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().override_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_settings`.\n"]
    pub fn set_ssl_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppEngineDomainMappingSslSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ssl_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ssl_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AppEngineDomainMappingTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `domain_name` after provisioning.\nRelative name of the domain serving the application. Example: example.com."]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `override_strategy` after provisioning.\nWhether the domain creation should override any existing mappings for this domain.\nBy default, overrides are rejected. Default value: \"STRICT\" Possible values: [\"STRICT\", \"OVERRIDE\"]"]
    pub fn override_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_records` after provisioning.\nThe resource records required to configure this domain mapping. These records must be added to the domain's DNS\nconfiguration in order to serve the application via this domain mapping."]
    pub fn resource_records(&self) -> terrars::ListRef<AppEngineDomainMappingResourceRecordsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_records", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_settings` after provisioning.\n"]
    pub fn ssl_settings(&self) -> terrars::ListRef<AppEngineDomainMappingSslSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineDomainMappingTimeoutsElRef {
        AppEngineDomainMappingTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for AppEngineDomainMapping {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppEngineDomainMapping { }

impl terrars::ToListMappable for AppEngineDomainMapping {
    type O = terrars::ListRef<AppEngineDomainMappingRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppEngineDomainMapping_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_app_engine_domain_mapping".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppEngineDomainMapping {
    pub tf_id: std::string::String,
    #[doc = "Relative name of the domain serving the application. Example: example.com."]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineDomainMapping {
    pub fn build(self, stack: &mut terrars::Stack) -> AppEngineDomainMapping {
        let out = AppEngineDomainMapping(std::rc::Rc::new(AppEngineDomainMapping_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppEngineDomainMappingData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                override_strategy: core::default::Default::default(),
                project: core::default::Default::default(),
                ssl_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppEngineDomainMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineDomainMappingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppEngineDomainMappingRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `domain_name` after provisioning.\nRelative name of the domain serving the application. Example: example.com."]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nFull path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `override_strategy` after provisioning.\nWhether the domain creation should override any existing mappings for this domain.\nBy default, overrides are rejected. Default value: \"STRICT\" Possible values: [\"STRICT\", \"OVERRIDE\"]"]
    pub fn override_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.override_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `resource_records` after provisioning.\nThe resource records required to configure this domain mapping. These records must be added to the domain's DNS\nconfiguration in order to serve the application via this domain mapping."]
    pub fn resource_records(&self) -> terrars::ListRef<AppEngineDomainMappingResourceRecordsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_records", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_settings` after provisioning.\n"]
    pub fn ssl_settings(&self) -> terrars::ListRef<AppEngineDomainMappingSslSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineDomainMappingTimeoutsElRef {
        AppEngineDomainMappingTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppEngineDomainMappingResourceRecordsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rrdata: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineDomainMappingResourceRecordsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rrdata`.\n"]
    pub fn set_rrdata(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rrdata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineDomainMappingResourceRecordsEl {
    type O = terrars::BlockAssignable<AppEngineDomainMappingResourceRecordsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineDomainMappingResourceRecordsEl {}

impl BuildAppEngineDomainMappingResourceRecordsEl {
    pub fn build(self) -> AppEngineDomainMappingResourceRecordsEl {
        AppEngineDomainMappingResourceRecordsEl {
            name: core::default::Default::default(),
            rrdata: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct AppEngineDomainMappingResourceRecordsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineDomainMappingResourceRecordsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineDomainMappingResourceRecordsElRef {
        AppEngineDomainMappingResourceRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineDomainMappingResourceRecordsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `rrdata` after provisioning.\n"]
    pub fn rrdata(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rrdata", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineDomainMappingSslSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
    ssl_management_type: terrars::PrimField<std::string::String>,
}

impl AppEngineDomainMappingSslSettingsEl {
    #[doc =
        "Set the field `certificate_id`.\nID of the AuthorizedCertificate resource configuring SSL for the application. Clearing this field will\nremove SSL support.\nBy default, a managed certificate is automatically created for every domain mapping. To omit SSL support\nor to configure SSL manually, specify 'SslManagementType.MANUAL' on a 'CREATE' or 'UPDATE' request. You must be\nauthorized to administer the 'AuthorizedCertificate' resource to manually map it to a DomainMapping resource.\nExample: 12345."]
    pub fn set_certificate_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppEngineDomainMappingSslSettingsEl {
    type O = terrars::BlockAssignable<AppEngineDomainMappingSslSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineDomainMappingSslSettingsEl {
    #[doc =
        "SSL management type for this domain. If 'AUTOMATIC', a managed certificate is automatically provisioned.\nIf 'MANUAL', 'certificateId' must be manually specified in order to configure SSL for this domain. Possible values: [\"AUTOMATIC\", \"MANUAL\"]"]
    pub ssl_management_type: terrars::PrimField<std::string::String>,
}

impl BuildAppEngineDomainMappingSslSettingsEl {
    pub fn build(self) -> AppEngineDomainMappingSslSettingsEl {
        AppEngineDomainMappingSslSettingsEl {
            certificate_id: core::default::Default::default(),
            ssl_management_type: self.ssl_management_type,
        }
    }
}

pub struct AppEngineDomainMappingSslSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineDomainMappingSslSettingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineDomainMappingSslSettingsElRef {
        AppEngineDomainMappingSslSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineDomainMappingSslSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `certificate_id` after provisioning.\nID of the AuthorizedCertificate resource configuring SSL for the application. Clearing this field will\nremove SSL support.\nBy default, a managed certificate is automatically created for every domain mapping. To omit SSL support\nor to configure SSL manually, specify 'SslManagementType.MANUAL' on a 'CREATE' or 'UPDATE' request. You must be\nauthorized to administer the 'AuthorizedCertificate' resource to manually map it to a DomainMapping resource.\nExample: 12345."]
    pub fn certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `pending_managed_certificate_id` after provisioning.\nID of the managed 'AuthorizedCertificate' resource currently being provisioned, if applicable. Until the new\nmanaged certificate has been successfully provisioned, the previous SSL state will be preserved. Once the\nprovisioning process completes, the 'certificateId' field will reflect the new managed certificate and this\nfield will be left empty. To remove SSL support while there is still a pending managed certificate, clear the\n'certificateId' field with an update request."]
    pub fn pending_managed_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_managed_certificate_id", self.base))
    }

    #[doc =
        "Get a reference to the value of field `ssl_management_type` after provisioning.\nSSL management type for this domain. If 'AUTOMATIC', a managed certificate is automatically provisioned.\nIf 'MANUAL', 'certificateId' must be manually specified in order to configure SSL for this domain. Possible values: [\"AUTOMATIC\", \"MANUAL\"]"]
    pub fn ssl_management_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_management_type", self.base))
    }
}

#[derive(Serialize)]
pub struct AppEngineDomainMappingTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppEngineDomainMappingTimeoutsEl {
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

impl terrars::ToListMappable for AppEngineDomainMappingTimeoutsEl {
    type O = terrars::BlockAssignable<AppEngineDomainMappingTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppEngineDomainMappingTimeoutsEl {}

impl BuildAppEngineDomainMappingTimeoutsEl {
    pub fn build(self) -> AppEngineDomainMappingTimeoutsEl {
        AppEngineDomainMappingTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct AppEngineDomainMappingTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppEngineDomainMappingTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppEngineDomainMappingTimeoutsElRef {
        AppEngineDomainMappingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppEngineDomainMappingTimeoutsElRef {
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
struct AppEngineDomainMappingDynamic {
    ssl_settings: std::option::Option<terrars::DynamicBlock<AppEngineDomainMappingSslSettingsEl>>,
}
