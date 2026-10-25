#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct HealthcareFhirStoreData_ {
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
    complex_data_type_reference_parsing: std::option::Option<terrars::PrimField<std::string::String>>,
    dataset: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_search_handling_strict: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deletion_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_referential_integrity: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_resource_versioning: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_history_import: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_update_create: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_config: std::option::Option<std::vec::Vec<HealthcareFhirStore_NotificationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_configs: std::option::Option<std::vec::Vec<HealthcareFhirStore_NotificationConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_configs: std::option::Option<std::vec::Vec<HealthcareFhirStore_StreamConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<HealthcareFhirStore_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_config: std::option::Option<std::vec::Vec<HealthcareFhirStore_ValidationConfig>>,
    dynamic: HealthcareFhirStoreDynamic,
}

struct HealthcareFhirStore_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<HealthcareFhirStoreData_>,
}

#[derive(Clone)]
pub struct HealthcareFhirStore(std::rc::Rc<HealthcareFhirStore_>);

impl HealthcareFhirStore {
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
        "Set the field `complex_data_type_reference_parsing`.\nEnable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED by default after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources. Possible values: [\"COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED\", \"DISABLED\", \"ENABLED\"]"]
    pub fn set_complex_data_type_reference_parsing(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().complex_data_type_reference_parsing = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `default_search_handling_strict`.\nIf true, overrides the default search behavior for this FHIR store to handling=strict which returns an error for unrecognized search parameters.\nIf false, uses the FHIR specification default handling=lenient which ignores unrecognized search parameters.\nThe handling can always be changed from the default on an individual API call by setting the HTTP header Prefer: handling=strict or Prefer: handling=lenient."]
    pub fn set_default_search_handling_strict(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_search_handling_strict = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_referential_integrity`.\nWhether to disable referential integrity in this FHIR store. This field is immutable after FHIR store\ncreation. The default value is false, meaning that the API will enforce referential integrity and fail the\nrequests that will result in inconsistent state in the FHIR store. When this field is set to true, the API\nwill skip referential integrity check. Consequently, operations that rely on references, such as\nPatient.get$everything, will not return all the results if broken references exist.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn set_disable_referential_integrity(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_referential_integrity = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_resource_versioning`.\nWhether to disable resource versioning for this FHIR store. This field can not be changed after the creation\nof FHIR store. If set to false, which is the default behavior, all write operations will cause historical\nversions to be recorded automatically. The historical versions can be fetched through the history APIs, but\ncannot be updated. If set to true, no historical versions will be kept. The server will send back errors for\nattempts to read the historical versions.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn set_disable_resource_versioning(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_resource_versioning = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_history_import`.\nWhether to allow the bulk import API to accept history bundles and directly insert historical resource\nversions into the FHIR store. Importing resource histories creates resource interactions that appear to have\noccurred in the past, which clients may not want to allow. If set to false, history bundles within an import\nwill fail with an error.\n\n** Changing this property may recreate the FHIR store (removing all data) **\n\n** This property can be changed manually in the Google Cloud Healthcare admin console without recreating the FHIR store **"]
    pub fn set_enable_history_import(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_history_import = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enable_update_create`.\nWhether this FHIR store has the updateCreate capability. This determines if the client can use an Update\noperation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through\nthe Create operation and attempts to Update a non-existent resource will return errors. Please treat the audit\nlogs with appropriate levels of care if client-specified resource IDs contain sensitive data such as patient\nidentifiers, those IDs will be part of the FHIR resource path recorded in Cloud audit logs and Cloud Pub/Sub\nnotifications."]
    pub fn set_enable_update_create(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_update_create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `labels`.\nUser-supplied key-value pairs used to organize FHIR stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn set_labels(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_config`.\n"]
    pub fn set_notification_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcareFhirStore_NotificationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification_configs`.\n"]
    pub fn set_notification_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcareFhirStore_NotificationConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stream_configs`.\n"]
    pub fn set_stream_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcareFhirStore_StreamConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stream_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stream_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<HealthcareFhirStore_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validation_config`.\n"]
    pub fn set_validation_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcareFhirStore_ValidationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `complex_data_type_reference_parsing` after provisioning.\nEnable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED by default after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources. Possible values: [\"COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED\", \"DISABLED\", \"ENABLED\"]"]
    pub fn complex_data_type_reference_parsing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.complex_data_type_reference_parsing", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `dataset` after provisioning.\nIdentifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_search_handling_strict` after provisioning.\nIf true, overrides the default search behavior for this FHIR store to handling=strict which returns an error for unrecognized search parameters.\nIf false, uses the FHIR specification default handling=lenient which ignores unrecognized search parameters.\nThe handling can always be changed from the default on an individual API call by setting the HTTP header Prefer: handling=strict or Prefer: handling=lenient."]
    pub fn default_search_handling_strict(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_search_handling_strict", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_referential_integrity` after provisioning.\nWhether to disable referential integrity in this FHIR store. This field is immutable after FHIR store\ncreation. The default value is false, meaning that the API will enforce referential integrity and fail the\nrequests that will result in inconsistent state in the FHIR store. When this field is set to true, the API\nwill skip referential integrity check. Consequently, operations that rely on references, such as\nPatient.get$everything, will not return all the results if broken references exist.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn disable_referential_integrity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_referential_integrity", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `disable_resource_versioning` after provisioning.\nWhether to disable resource versioning for this FHIR store. This field can not be changed after the creation\nof FHIR store. If set to false, which is the default behavior, all write operations will cause historical\nversions to be recorded automatically. The historical versions can be fetched through the history APIs, but\ncannot be updated. If set to true, no historical versions will be kept. The server will send back errors for\nattempts to read the historical versions.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn disable_resource_versioning(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_resource_versioning", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_history_import` after provisioning.\nWhether to allow the bulk import API to accept history bundles and directly insert historical resource\nversions into the FHIR store. Importing resource histories creates resource interactions that appear to have\noccurred in the past, which clients may not want to allow. If set to false, history bundles within an import\nwill fail with an error.\n\n** Changing this property may recreate the FHIR store (removing all data) **\n\n** This property can be changed manually in the Google Cloud Healthcare admin console without recreating the FHIR store **"]
    pub fn enable_history_import(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_history_import", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_update_create` after provisioning.\nWhether this FHIR store has the updateCreate capability. This determines if the client can use an Update\noperation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through\nthe Create operation and attempts to Update a non-existent resource will return errors. Please treat the audit\nlogs with appropriate levels of care if client-specified resource IDs contain sensitive data such as patient\nidentifiers, those IDs will be part of the FHIR resource path recorded in Cloud audit logs and Cloud Pub/Sub\nnotifications."]
    pub fn enable_update_create(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_update_create", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize FHIR stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for the FhirStore.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe FHIR specification version. Possible values: [\"DSTU2\", \"STU3\", \"R4\"]"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<HealthcareFhirStore_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_configs` after provisioning.\n"]
    pub fn notification_configs(&self) -> terrars::ListRef<HealthcareFhirStore_NotificationConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_configs` after provisioning.\n"]
    pub fn stream_configs(&self) -> terrars::ListRef<HealthcareFhirStore_StreamConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stream_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcareFhirStore_TimeoutsRef {
        HealthcareFhirStore_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_config` after provisioning.\n"]
    pub fn validation_config(&self) -> terrars::ListRef<HealthcareFhirStore_ValidationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_config", self.extract_ref()))
    }
}

impl terrars::Referable for HealthcareFhirStore {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for HealthcareFhirStore { }

impl terrars::ToListMappable for HealthcareFhirStore {
    type O = terrars::ListRef<HealthcareFhirStoreRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for HealthcareFhirStore_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_healthcare_fhir_store".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildHealthcareFhirStore {
    pub tf_id: std::string::String,
    #[doc =
        "Identifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub dataset: terrars::PrimField<std::string::String>,
    #[doc =
        "The resource name for the FhirStore.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "The FHIR specification version. Possible values: [\"DSTU2\", \"STU3\", \"R4\"]"]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildHealthcareFhirStore {
    pub fn build(self, stack: &mut terrars::Stack) -> HealthcareFhirStore {
        let out = HealthcareFhirStore(std::rc::Rc::new(HealthcareFhirStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(HealthcareFhirStoreData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                complex_data_type_reference_parsing: core::default::Default::default(),
                dataset: self.dataset,
                default_search_handling_strict: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                disable_referential_integrity: core::default::Default::default(),
                disable_resource_versioning: core::default::Default::default(),
                enable_history_import: core::default::Default::default(),
                enable_update_create: core::default::Default::default(),
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                name: self.name,
                version: self.version,
                notification_config: core::default::Default::default(),
                notification_configs: core::default::Default::default(),
                stream_configs: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                validation_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct HealthcareFhirStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareFhirStoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl HealthcareFhirStoreRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `complex_data_type_reference_parsing` after provisioning.\nEnable parsing of references within complex FHIR data types such as Extensions. If this value is set to ENABLED, then features like referential integrity and Bundle reference rewriting apply to all references. If this flag has not been specified the behavior of the FHIR store will not change, references in complex data types will not be parsed. New stores will have this value set to ENABLED by default after a notification period. Warning: turning on this flag causes processing existing resources to fail if they contain references to non-existent resources. Possible values: [\"COMPLEX_DATA_TYPE_REFERENCE_PARSING_UNSPECIFIED\", \"DISABLED\", \"ENABLED\"]"]
    pub fn complex_data_type_reference_parsing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.complex_data_type_reference_parsing", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `dataset` after provisioning.\nIdentifies the dataset addressed by this request. Must be in the format\n'projects/{project}/locations/{location}/datasets/{dataset}'"]
    pub fn dataset(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `default_search_handling_strict` after provisioning.\nIf true, overrides the default search behavior for this FHIR store to handling=strict which returns an error for unrecognized search parameters.\nIf false, uses the FHIR specification default handling=lenient which ignores unrecognized search parameters.\nThe handling can always be changed from the default on an individual API call by setting the HTTP header Prefer: handling=strict or Prefer: handling=lenient."]
    pub fn default_search_handling_strict(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_search_handling_strict", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deletion_policy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `disable_referential_integrity` after provisioning.\nWhether to disable referential integrity in this FHIR store. This field is immutable after FHIR store\ncreation. The default value is false, meaning that the API will enforce referential integrity and fail the\nrequests that will result in inconsistent state in the FHIR store. When this field is set to true, the API\nwill skip referential integrity check. Consequently, operations that rely on references, such as\nPatient.get$everything, will not return all the results if broken references exist.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn disable_referential_integrity(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.disable_referential_integrity", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `disable_resource_versioning` after provisioning.\nWhether to disable resource versioning for this FHIR store. This field can not be changed after the creation\nof FHIR store. If set to false, which is the default behavior, all write operations will cause historical\nversions to be recorded automatically. The historical versions can be fetched through the history APIs, but\ncannot be updated. If set to true, no historical versions will be kept. The server will send back errors for\nattempts to read the historical versions.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn disable_resource_versioning(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_resource_versioning", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.effective_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_history_import` after provisioning.\nWhether to allow the bulk import API to accept history bundles and directly insert historical resource\nversions into the FHIR store. Importing resource histories creates resource interactions that appear to have\noccurred in the past, which clients may not want to allow. If set to false, history bundles within an import\nwill fail with an error.\n\n** Changing this property may recreate the FHIR store (removing all data) **\n\n** This property can be changed manually in the Google Cloud Healthcare admin console without recreating the FHIR store **"]
    pub fn enable_history_import(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_history_import", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `enable_update_create` after provisioning.\nWhether this FHIR store has the updateCreate capability. This determines if the client can use an Update\noperation to create a new resource with a client-specified ID. If false, all IDs are server-assigned through\nthe Create operation and attempts to Update a non-existent resource will return errors. Please treat the audit\nlogs with appropriate levels of care if client-specified resource IDs contain sensitive data such as patient\nidentifiers, those IDs will be part of the FHIR resource path recorded in Cloud audit logs and Cloud Pub/Sub\nnotifications."]
    pub fn enable_update_create(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_update_create", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `labels` after provisioning.\nUser-supplied key-value pairs used to organize FHIR stores.\n\nLabel keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must\nconform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}][\\p{Ll}\\p{Lo}\\p{N}_-]{0,62}\n\nLabel values are optional, must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128\nbytes, and must conform to the following PCRE regular expression: [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n\nNo more than 64 labels can be associated with a given store.\n\nAn object containing a list of \"key\": value pairs.\nExample: { \"name\": \"wrench\", \"mass\": \"1.3kg\", \"count\": \"3\" }.\n\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field 'effective_labels' for all of the labels present on the resource."]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe resource name for the FhirStore.\n\n** Changing this property may recreate the FHIR store (removing all data) **"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `self_link` after provisioning.\nThe fully qualified name of this dataset"]
    pub fn self_link(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.self_link", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource\n and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.terraform_labels", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `version` after provisioning.\nThe FHIR specification version. Possible values: [\"DSTU2\", \"STU3\", \"R4\"]"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_config` after provisioning.\n"]
    pub fn notification_config(&self) -> terrars::ListRef<HealthcareFhirStore_NotificationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_configs` after provisioning.\n"]
    pub fn notification_configs(&self) -> terrars::ListRef<HealthcareFhirStore_NotificationConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stream_configs` after provisioning.\n"]
    pub fn stream_configs(&self) -> terrars::ListRef<HealthcareFhirStore_StreamConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stream_configs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> HealthcareFhirStore_TimeoutsRef {
        HealthcareFhirStore_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_config` after provisioning.\n"]
    pub fn validation_config(&self) -> terrars::ListRef<HealthcareFhirStore_ValidationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct HealthcareFhirStoreDynamic {
    notification_config: std::option::Option<terrars::DynamicBlock<HealthcareFhirStore_NotificationConfig>>,
    notification_configs: std::option::Option<terrars::DynamicBlock<HealthcareFhirStore_NotificationConfigs>>,
    stream_configs: std::option::Option<terrars::DynamicBlock<HealthcareFhirStore_StreamConfigs>>,
    validation_config: std::option::Option<terrars::DynamicBlock<HealthcareFhirStore_ValidationConfig>>,
}

#[derive(Serialize)]
pub struct HealthcareFhirStore_NotificationConfig {
    pubsub_topic: terrars::PrimField<std::string::String>,
}

impl HealthcareFhirStore_NotificationConfig { }

impl terrars::ToListMappable for HealthcareFhirStore_NotificationConfig {
    type O = terrars::BlockAssignable<HealthcareFhirStore_NotificationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareFhirStore_NotificationConfig {
    #[doc =
        "The Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub pubsub_topic: terrars::PrimField<std::string::String>,
}

impl BuildHealthcareFhirStore_NotificationConfig {
    pub fn build(self) -> HealthcareFhirStore_NotificationConfig {
        HealthcareFhirStore_NotificationConfig { pubsub_topic: self.pubsub_topic }
    }
}

#[allow(dead_code)]
pub struct HealthcareFhirStore_NotificationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareFhirStore_NotificationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcareFhirStore_NotificationConfigRef {
        HealthcareFhirStore_NotificationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareFhirStore_NotificationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub fn pubsub_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }
}

#[derive(Serialize)]
pub struct HealthcareFhirStore_NotificationConfigs {
    pubsub_topic: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_full_resource: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_previous_resource_on_delete: std::option::Option<terrars::PrimField<bool>>,
}

impl HealthcareFhirStore_NotificationConfigs {
    #[doc =
        "Set the field `send_full_resource`.\nWhether to send full FHIR resource to this Pub/Sub topic for Create and Update operation.\nNote that setting this to true does not guarantee that all resources will be sent in the format of\nfull FHIR resource. When a resource change is too large or during heavy traffic, only the resource name will be\nsent. Clients should always check the \"payloadType\" label from a Pub/Sub message to determine whether\nit needs to fetch the full resource as a separate operation."]
    pub fn set_send_full_resource(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.send_full_resource = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `send_previous_resource_on_delete`.\nWhether to send full FHIR resource to this Pub/Sub topic for deleting FHIR resource. Note that setting this to\ntrue does not guarantee that all previous resources will be sent in the format of full FHIR resource. When a\nresource change is too large or during heavy traffic, only the resource name will be sent. Clients should always\ncheck the \"payloadType\" label from a Pub/Sub message to determine whether it needs to fetch the full previous\nresource as a separate operation."]
    pub fn set_send_previous_resource_on_delete(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.send_previous_resource_on_delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HealthcareFhirStore_NotificationConfigs {
    type O = terrars::BlockAssignable<HealthcareFhirStore_NotificationConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareFhirStore_NotificationConfigs {
    #[doc =
        "The Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub pubsub_topic: terrars::PrimField<std::string::String>,
}

impl BuildHealthcareFhirStore_NotificationConfigs {
    pub fn build(self) -> HealthcareFhirStore_NotificationConfigs {
        HealthcareFhirStore_NotificationConfigs {
            pubsub_topic: self.pubsub_topic,
            send_full_resource: core::default::Default::default(),
            send_previous_resource_on_delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HealthcareFhirStore_NotificationConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareFhirStore_NotificationConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcareFhirStore_NotificationConfigsRef {
        HealthcareFhirStore_NotificationConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareFhirStore_NotificationConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `pubsub_topic` after provisioning.\nThe Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.\nPubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.\nIt is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message\nwas published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a\nproject. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given\nCloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail."]
    pub fn pubsub_topic(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pubsub_topic", self.base))
    }

    #[doc =
        "Get a reference to the value of field `send_full_resource` after provisioning.\nWhether to send full FHIR resource to this Pub/Sub topic for Create and Update operation.\nNote that setting this to true does not guarantee that all resources will be sent in the format of\nfull FHIR resource. When a resource change is too large or during heavy traffic, only the resource name will be\nsent. Clients should always check the \"payloadType\" label from a Pub/Sub message to determine whether\nit needs to fetch the full resource as a separate operation."]
    pub fn send_full_resource(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.send_full_resource", self.base))
    }

    #[doc =
        "Get a reference to the value of field `send_previous_resource_on_delete` after provisioning.\nWhether to send full FHIR resource to this Pub/Sub topic for deleting FHIR resource. Note that setting this to\ntrue does not guarantee that all previous resources will be sent in the format of full FHIR resource. When a\nresource change is too large or during heavy traffic, only the resource name will be sent. Clients should always\ncheck the \"payloadType\" label from a Pub/Sub message to determine whether it needs to fetch the full previous\nresource as a separate operation."]
    pub fn send_previous_resource_on_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.send_previous_resource_on_delete", self.base))
    }
}

#[derive(Serialize)]
pub struct HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_ms: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfig {
    #[doc = "Set the field `expiration_ms`.\nNumber of milliseconds for which to keep the storage for a partition."]
    pub fn set_expiration_ms(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expiration_ms = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfig {
    type O =
        terrars
        ::BlockAssignable<
            HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfig {
    #[doc =
        "Type of partitioning. Possible values: [\"PARTITION_TYPE_UNSPECIFIED\", \"HOUR\", \"DAY\", \"MONTH\", \"YEAR\"]"]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildHealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfig {
    pub fn build(
        self,
    ) -> HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfig {
        HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfig {
            expiration_ms: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfigRef {
        HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `expiration_ms` after provisioning.\nNumber of milliseconds for which to keep the storage for a partition."]
    pub fn expiration_ms(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_ms", self.base))
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nType of partitioning. Possible values: [\"PARTITION_TYPE_UNSPECIFIED\", \"HOUR\", \"DAY\", \"MONTH\", \"YEAR\"]"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfigDynamic {
    last_updated_partition_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfig>,
    >,
}

#[derive(Serialize)]
pub struct HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig {
    recursive_structure_depth: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_updated_partition_config: std
    ::option
    ::Option<
        std::vec::Vec<HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfig>,
    >,
    dynamic: HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfigDynamic,
}

impl HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig {
    #[doc =
        "Set the field `schema_type`.\nSpecifies the output schema type.\n * ANALYTICS: Analytics schema defined by the FHIR community.\n  See https://github.com/FHIR/sql-on-fhir/blob/master/sql-on-fhir.md.\n * ANALYTICS_V2: Analytics V2, similar to schema defined by the FHIR community, with added support for extensions with one or more occurrences and contained resources in stringified JSON.\n * LOSSLESS: A data-driven schema generated from the fields present in the FHIR data being exported, with no additional simplification. Default value: \"ANALYTICS\" Possible values: [\"ANALYTICS\", \"ANALYTICS_V2\", \"LOSSLESS\"]"]
    pub fn set_schema_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schema_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_updated_partition_config`.\n"]
    pub fn set_last_updated_partition_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.last_updated_partition_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.last_updated_partition_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig {
    type O = terrars::BlockAssignable<HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig {
    #[doc =
        "The depth for all recursive structures in the output analytics schema. For example, concept in the CodeSystem\nresource is a recursive structure; when the depth is 2, the CodeSystem table will have a column called\nconcept.concept but not concept.concept.concept. If not specified or set to 0, the server will use the default\nvalue 2. The maximum depth allowed is 5."]
    pub recursive_structure_depth: terrars::PrimField<f64>,
}

impl BuildHealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig {
    pub fn build(self) -> HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig {
        HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig {
            recursive_structure_depth: self.recursive_structure_depth,
            schema_type: core::default::Default::default(),
            last_updated_partition_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfigRef {
        HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `recursive_structure_depth` after provisioning.\nThe depth for all recursive structures in the output analytics schema. For example, concept in the CodeSystem\nresource is a recursive structure; when the depth is 2, the CodeSystem table will have a column called\nconcept.concept but not concept.concept.concept. If not specified or set to 0, the server will use the default\nvalue 2. The maximum depth allowed is 5."]
    pub fn recursive_structure_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recursive_structure_depth", self.base))
    }

    #[doc =
        "Get a reference to the value of field `schema_type` after provisioning.\nSpecifies the output schema type.\n * ANALYTICS: Analytics schema defined by the FHIR community.\n  See https://github.com/FHIR/sql-on-fhir/blob/master/sql-on-fhir.md.\n * ANALYTICS_V2: Analytics V2, similar to schema defined by the FHIR community, with added support for extensions with one or more occurrences and contained resources in stringified JSON.\n * LOSSLESS: A data-driven schema generated from the fields present in the FHIR data being exported, with no additional simplification. Default value: \"ANALYTICS\" Possible values: [\"ANALYTICS\", \"ANALYTICS_V2\", \"LOSSLESS\"]"]
    pub fn schema_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schema_type", self.base))
    }

    #[doc = "Get a reference to the value of field `last_updated_partition_config` after provisioning.\n"]
    pub fn last_updated_partition_config(
        &self,
    ) -> terrars
    ::ListRef<HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig_LastUpdatedPartitionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.last_updated_partition_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct HealthcareFhirStore_StreamConfigs_BigqueryDestinationDynamic {
    schema_config: std
    ::option
    ::Option<terrars::DynamicBlock<HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig>>,
}

#[derive(Serialize)]
pub struct HealthcareFhirStore_StreamConfigs_BigqueryDestination {
    dataset_uri: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schema_config: std::option::Option<std::vec::Vec<HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig>>,
    dynamic: HealthcareFhirStore_StreamConfigs_BigqueryDestinationDynamic,
}

impl HealthcareFhirStore_StreamConfigs_BigqueryDestination {
    #[doc = "Set the field `schema_config`.\n"]
    pub fn set_schema_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schema_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schema_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HealthcareFhirStore_StreamConfigs_BigqueryDestination {
    type O = terrars::BlockAssignable<HealthcareFhirStore_StreamConfigs_BigqueryDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareFhirStore_StreamConfigs_BigqueryDestination {
    #[doc = "BigQuery URI to a dataset, up to 2000 characters long, in the format bq://projectId.bqDatasetId"]
    pub dataset_uri: terrars::PrimField<std::string::String>,
}

impl BuildHealthcareFhirStore_StreamConfigs_BigqueryDestination {
    pub fn build(self) -> HealthcareFhirStore_StreamConfigs_BigqueryDestination {
        HealthcareFhirStore_StreamConfigs_BigqueryDestination {
            dataset_uri: self.dataset_uri,
            schema_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HealthcareFhirStore_StreamConfigs_BigqueryDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareFhirStore_StreamConfigs_BigqueryDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> HealthcareFhirStore_StreamConfigs_BigqueryDestinationRef {
        HealthcareFhirStore_StreamConfigs_BigqueryDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareFhirStore_StreamConfigs_BigqueryDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `dataset_uri` after provisioning.\nBigQuery URI to a dataset, up to 2000 characters long, in the format bq://projectId.bqDatasetId"]
    pub fn dataset_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dataset_uri", self.base))
    }

    #[doc = "Get a reference to the value of field `schema_config` after provisioning.\n"]
    pub fn schema_config(
        &self,
    ) -> terrars::ListRef<HealthcareFhirStore_StreamConfigs_BigqueryDestination_SchemaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schema_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct HealthcareFhirStore_StreamConfigsDynamic {
    bigquery_destination: std
    ::option
    ::Option<terrars::DynamicBlock<HealthcareFhirStore_StreamConfigs_BigqueryDestination>>,
}

#[derive(Serialize)]
pub struct HealthcareFhirStore_StreamConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bigquery_destination: std::option::Option<std::vec::Vec<HealthcareFhirStore_StreamConfigs_BigqueryDestination>>,
    dynamic: HealthcareFhirStore_StreamConfigsDynamic,
}

impl HealthcareFhirStore_StreamConfigs {
    #[doc =
        "Set the field `resource_types`.\nSupply a FHIR resource type (such as \"Patient\" or \"Observation\"). See\nhttps://www.hl7.org/fhir/valueset-resource-types.html for a list of all FHIR resource types. The server treats\nan empty list as an intent to stream all the supported resource types in this FHIR store."]
    pub fn set_resource_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bigquery_destination`.\n"]
    pub fn set_bigquery_destination(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<HealthcareFhirStore_StreamConfigs_BigqueryDestination>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bigquery_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bigquery_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for HealthcareFhirStore_StreamConfigs {
    type O = terrars::BlockAssignable<HealthcareFhirStore_StreamConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareFhirStore_StreamConfigs {}

impl BuildHealthcareFhirStore_StreamConfigs {
    pub fn build(self) -> HealthcareFhirStore_StreamConfigs {
        HealthcareFhirStore_StreamConfigs {
            resource_types: core::default::Default::default(),
            bigquery_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HealthcareFhirStore_StreamConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareFhirStore_StreamConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcareFhirStore_StreamConfigsRef {
        HealthcareFhirStore_StreamConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareFhirStore_StreamConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `resource_types` after provisioning.\nSupply a FHIR resource type (such as \"Patient\" or \"Observation\"). See\nhttps://www.hl7.org/fhir/valueset-resource-types.html for a list of all FHIR resource types. The server treats\nan empty list as an intent to stream all the supported resource types in this FHIR store."]
    pub fn resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }

    #[doc = "Get a reference to the value of field `bigquery_destination` after provisioning.\n"]
    pub fn bigquery_destination(&self) -> terrars::ListRef<HealthcareFhirStore_StreamConfigs_BigqueryDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bigquery_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct HealthcareFhirStore_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl HealthcareFhirStore_Timeouts {
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

impl terrars::ToListMappable for HealthcareFhirStore_Timeouts {
    type O = terrars::BlockAssignable<HealthcareFhirStore_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareFhirStore_Timeouts {}

impl BuildHealthcareFhirStore_Timeouts {
    pub fn build(self) -> HealthcareFhirStore_Timeouts {
        HealthcareFhirStore_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HealthcareFhirStore_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareFhirStore_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcareFhirStore_TimeoutsRef {
        HealthcareFhirStore_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareFhirStore_TimeoutsRef {
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
pub struct HealthcareFhirStore_ValidationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_fhirpath_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_profile_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_reference_type_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_required_field_validation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_implementation_guides: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl HealthcareFhirStore_ValidationConfig {
    #[doc =
        "Set the field `disable_fhirpath_validation`.\nWhether to disable FHIRPath validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against FHIRPath requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn set_disable_fhirpath_validation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_fhirpath_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_profile_validation`.\nWhether to disable profile validation for this FHIR store. The default value is false. Set this to true to disable checking incoming resources for conformance against structure definitions in this FHIR store."]
    pub fn set_disable_profile_validation(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.disable_profile_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_reference_type_validation`.\nWhether to disable reference type validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against reference type requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn set_disable_reference_type_validation(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.disable_reference_type_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `disable_required_field_validation`.\nWhether to disable required fields validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against required fields requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn set_disable_required_field_validation(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.disable_required_field_validation = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `enabled_implementation_guides`.\nA list of implementation guide URLs in this FHIR store that are used to configure the profiles to use for validation.\nWhen a URL cannot be resolved (for example, in a type assertion), the server does not return an error.\nFor example, to use the US Core profiles for validation, set enabledImplementationGuides to [\"http://hl7.org/fhir/us/core/ImplementationGuide/ig\"]. If enabledImplementationGuides is empty or omitted, then incoming resources are only required to conform to the base FHIR profiles. Otherwise, a resource must conform to at least one profile listed in the global property of one of the enabled ImplementationGuides.\nThe Cloud Healthcare API does not currently enforce all of the rules in a StructureDefinition. The following rules are supported:\n- min/max\n- minValue/maxValue\n- maxLength\n- type\n- fixed[x]\n- pattern[x] on simple types\n- slicing, when using \"value\" as the discriminator type"]
    pub fn set_enabled_implementation_guides(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.enabled_implementation_guides = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for HealthcareFhirStore_ValidationConfig {
    type O = terrars::BlockAssignable<HealthcareFhirStore_ValidationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildHealthcareFhirStore_ValidationConfig {}

impl BuildHealthcareFhirStore_ValidationConfig {
    pub fn build(self) -> HealthcareFhirStore_ValidationConfig {
        HealthcareFhirStore_ValidationConfig {
            disable_fhirpath_validation: core::default::Default::default(),
            disable_profile_validation: core::default::Default::default(),
            disable_reference_type_validation: core::default::Default::default(),
            disable_required_field_validation: core::default::Default::default(),
            enabled_implementation_guides: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct HealthcareFhirStore_ValidationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for HealthcareFhirStore_ValidationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> HealthcareFhirStore_ValidationConfigRef {
        HealthcareFhirStore_ValidationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl HealthcareFhirStore_ValidationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `disable_fhirpath_validation` after provisioning.\nWhether to disable FHIRPath validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against FHIRPath requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn disable_fhirpath_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_fhirpath_validation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_profile_validation` after provisioning.\nWhether to disable profile validation for this FHIR store. The default value is false. Set this to true to disable checking incoming resources for conformance against structure definitions in this FHIR store."]
    pub fn disable_profile_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_profile_validation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_reference_type_validation` after provisioning.\nWhether to disable reference type validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against reference type requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn disable_reference_type_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_reference_type_validation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `disable_required_field_validation` after provisioning.\nWhether to disable required fields validation for incoming resources. The default value is false. Set this to true to disable checking incoming resources for conformance against required fields requirement defined in the FHIR specification. This property only affects resource types that do not have profiles configured for them, any rules in enabled implementation guides will still be enforced."]
    pub fn disable_required_field_validation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_required_field_validation", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled_implementation_guides` after provisioning.\nA list of implementation guide URLs in this FHIR store that are used to configure the profiles to use for validation.\nWhen a URL cannot be resolved (for example, in a type assertion), the server does not return an error.\nFor example, to use the US Core profiles for validation, set enabledImplementationGuides to [\"http://hl7.org/fhir/us/core/ImplementationGuide/ig\"]. If enabledImplementationGuides is empty or omitted, then incoming resources are only required to conform to the base FHIR profiles. Otherwise, a resource must conform to at least one profile listed in the global property of one of the enabled ImplementationGuides.\nThe Cloud Healthcare API does not currently enforce all of the rules in a StructureDefinition. The following rules are supported:\n- min/max\n- minValue/maxValue\n- maxLength\n- type\n- fixed[x]\n- pattern[x] on simple types\n- slicing, when using \"value\" as the discriminator type"]
    pub fn enabled_implementation_guides(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enabled_implementation_guides", self.base))
    }
}
