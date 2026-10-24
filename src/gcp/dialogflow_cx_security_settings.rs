use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct DialogflowCxSecuritySettingsData {
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
    deidentify_template: std::option::Option<terrars::PrimField<std::string::String>>,
    display_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspect_template: std::option::Option<terrars::PrimField<std::string::String>>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    purge_data_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redaction_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redaction_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_window_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_export_settings: std::option::Option<std::vec::Vec<DialogflowCxSecuritySettingsAudioExportSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insights_export_settings: std::option::Option<std::vec::Vec<DialogflowCxSecuritySettingsInsightsExportSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DialogflowCxSecuritySettingsTimeoutsEl>,
    dynamic: DialogflowCxSecuritySettingsDynamic,
}

struct DialogflowCxSecuritySettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DialogflowCxSecuritySettingsData>,
}

#[derive(Clone)]
pub struct DialogflowCxSecuritySettings(std::rc::Rc<DialogflowCxSecuritySettings_>);

impl DialogflowCxSecuritySettings {
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
        "Set the field `deidentify_template`.\n[DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. If empty, Dialogflow replaces sensitive info with [redacted] text.\nNote: deidentifyTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/deidentifyTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/deidentifyTemplates/<Template ID>"]
    pub fn set_deidentify_template(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().deidentify_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `inspect_template`.\n[DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. If empty, we use the default DLP inspect config.\nNote: inspectTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/inspectTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/inspectTemplates/<Template ID>"]
    pub fn set_inspect_template(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().inspect_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().project = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `purge_data_types`.\nList of types of data to remove when retention settings triggers purge. Possible values: [\"DIALOGFLOW_HISTORY\"]"]
    pub fn set_purge_data_types(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().purge_data_types = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redaction_scope`.\nDefines what types of data to redact. If not set, defaults to not redacting any kind of data.\n* REDACT_DISK_STORAGE: On data to be written to disk or similar devices that are capable of holding data even if power is disconnected. This includes data that are temporarily saved on disk. Possible values: [\"REDACT_DISK_STORAGE\"]"]
    pub fn set_redaction_scope(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().redaction_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `redaction_strategy`.\nDefines how we redact data. If not set, defaults to not redacting.\n* REDACT_WITH_SERVICE: Call redaction service to clean up the data to be persisted. Possible values: [\"REDACT_WITH_SERVICE\"]"]
    pub fn set_redaction_strategy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().redaction_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retention_strategy`.\nDefines how long we retain persisted data that contains sensitive info. Only one of 'retention_window_days' and 'retention_strategy' may be set.\n* REMOVE_AFTER_CONVERSATION: Removes data when the conversation ends. If there is no conversation explicitly established, a default conversation ends when the corresponding Dialogflow session ends. Possible values: [\"REMOVE_AFTER_CONVERSATION\"]"]
    pub fn set_retention_strategy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().retention_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `retention_window_days`.\nRetains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL.\nOnly one of 'retention_window_days' and 'retention_strategy' may be set."]
    pub fn set_retention_window_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retention_window_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audio_export_settings`.\n"]
    pub fn set_audio_export_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxSecuritySettingsAudioExportSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().audio_export_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.audio_export_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `insights_export_settings`.\n"]
    pub fn set_insights_export_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DialogflowCxSecuritySettingsInsightsExportSettingsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().insights_export_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.insights_export_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DialogflowCxSecuritySettingsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `deidentify_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. If empty, Dialogflow replaces sensitive info with [redacted] text.\nNote: deidentifyTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/deidentifyTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/deidentifyTemplates/<Template ID>"]
    pub fn deidentify_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deidentify_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the security settings, unique within the location."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inspect_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. If empty, we use the default DLP inspect config.\nNote: inspectTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/inspectTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/inspectTemplates/<Template ID>"]
    pub fn inspect_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location these settings are located in. Settings can only be applied to an agent in the same location.\nSee [Available Regions](https://cloud.google.com/dialogflow/cx/docs/concept/region#avail) for a list of supported locations."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the settings.\nFormat: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purge_data_types` after provisioning.\nList of types of data to remove when retention settings triggers purge. Possible values: [\"DIALOGFLOW_HISTORY\"]"]
    pub fn purge_data_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.purge_data_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redaction_scope` after provisioning.\nDefines what types of data to redact. If not set, defaults to not redacting any kind of data.\n* REDACT_DISK_STORAGE: On data to be written to disk or similar devices that are capable of holding data even if power is disconnected. This includes data that are temporarily saved on disk. Possible values: [\"REDACT_DISK_STORAGE\"]"]
    pub fn redaction_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redaction_scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redaction_strategy` after provisioning.\nDefines how we redact data. If not set, defaults to not redacting.\n* REDACT_WITH_SERVICE: Call redaction service to clean up the data to be persisted. Possible values: [\"REDACT_WITH_SERVICE\"]"]
    pub fn redaction_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redaction_strategy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_strategy` after provisioning.\nDefines how long we retain persisted data that contains sensitive info. Only one of 'retention_window_days' and 'retention_strategy' may be set.\n* REMOVE_AFTER_CONVERSATION: Removes data when the conversation ends. If there is no conversation explicitly established, a default conversation ends when the corresponding Dialogflow session ends. Possible values: [\"REMOVE_AFTER_CONVERSATION\"]"]
    pub fn retention_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_strategy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_window_days` after provisioning.\nRetains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL.\nOnly one of 'retention_window_days' and 'retention_strategy' may be set."]
    pub fn retention_window_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_window_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio_export_settings` after provisioning.\n"]
    pub fn audio_export_settings(&self) -> terrars::ListRef<DialogflowCxSecuritySettingsAudioExportSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_export_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `insights_export_settings` after provisioning.\n"]
    pub fn insights_export_settings(&self) -> terrars::ListRef<DialogflowCxSecuritySettingsInsightsExportSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.insights_export_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxSecuritySettingsTimeoutsElRef {
        DialogflowCxSecuritySettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DialogflowCxSecuritySettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DialogflowCxSecuritySettings { }

impl terrars::ToListMappable for DialogflowCxSecuritySettings {
    type O = terrars::ListRef<DialogflowCxSecuritySettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DialogflowCxSecuritySettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_dialogflow_cx_security_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDialogflowCxSecuritySettings {
    pub tf_id: std::string::String,
    #[doc = "The human-readable name of the security settings, unique within the location."]
    pub display_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The location these settings are located in. Settings can only be applied to an agent in the same location.\nSee [Available Regions](https://cloud.google.com/dialogflow/cx/docs/concept/region#avail) for a list of supported locations."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildDialogflowCxSecuritySettings {
    pub fn build(self, stack: &mut terrars::Stack) -> DialogflowCxSecuritySettings {
        let out = DialogflowCxSecuritySettings(std::rc::Rc::new(DialogflowCxSecuritySettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DialogflowCxSecuritySettingsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                deidentify_template: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                inspect_template: core::default::Default::default(),
                location: self.location,
                project: core::default::Default::default(),
                purge_data_types: core::default::Default::default(),
                redaction_scope: core::default::Default::default(),
                redaction_strategy: core::default::Default::default(),
                retention_strategy: core::default::Default::default(),
                retention_window_days: core::default::Default::default(),
                audio_export_settings: core::default::Default::default(),
                insights_export_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DialogflowCxSecuritySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxSecuritySettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DialogflowCxSecuritySettingsRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `deidentify_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) deidentify template name. Use this template to define de-identification configuration for the content. If empty, Dialogflow replaces sensitive info with [redacted] text.\nNote: deidentifyTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/deidentifyTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/deidentifyTemplates/<Template ID>"]
    pub fn deidentify_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deidentify_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `display_name` after provisioning.\nThe human-readable name of the security settings, unique within the location."]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `inspect_template` after provisioning.\n[DLP](https://cloud.google.com/dlp/docs) inspect template name. Use this template to define inspect base settings. If empty, we use the default DLP inspect config.\nNote: inspectTemplate must be located in the same region as the SecuritySettings.\nFormat: projects/<Project ID>/locations/<Location ID>/inspectTemplates/<Template ID> OR organizations/<Organization ID>/locations/<Location ID>/inspectTemplates/<Template ID>"]
    pub fn inspect_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspect_template", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `location` after provisioning.\nThe location these settings are located in. Settings can only be applied to an agent in the same location.\nSee [Available Regions](https://cloud.google.com/dialogflow/cx/docs/concept/region#avail) for a list of supported locations."]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nThe unique identifier of the settings.\nFormat: projects/<Project ID>/locations/<Location ID>/securitySettings/<Security Settings ID>."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purge_data_types` after provisioning.\nList of types of data to remove when retention settings triggers purge. Possible values: [\"DIALOGFLOW_HISTORY\"]"]
    pub fn purge_data_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.purge_data_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redaction_scope` after provisioning.\nDefines what types of data to redact. If not set, defaults to not redacting any kind of data.\n* REDACT_DISK_STORAGE: On data to be written to disk or similar devices that are capable of holding data even if power is disconnected. This includes data that are temporarily saved on disk. Possible values: [\"REDACT_DISK_STORAGE\"]"]
    pub fn redaction_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redaction_scope", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `redaction_strategy` after provisioning.\nDefines how we redact data. If not set, defaults to not redacting.\n* REDACT_WITH_SERVICE: Call redaction service to clean up the data to be persisted. Possible values: [\"REDACT_WITH_SERVICE\"]"]
    pub fn redaction_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redaction_strategy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_strategy` after provisioning.\nDefines how long we retain persisted data that contains sensitive info. Only one of 'retention_window_days' and 'retention_strategy' may be set.\n* REMOVE_AFTER_CONVERSATION: Removes data when the conversation ends. If there is no conversation explicitly established, a default conversation ends when the corresponding Dialogflow session ends. Possible values: [\"REMOVE_AFTER_CONVERSATION\"]"]
    pub fn retention_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_strategy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `retention_window_days` after provisioning.\nRetains the data for the specified number of days. User must set a value lower than Dialogflow's default 365d TTL (30 days for Agent Assist traffic), higher value will be ignored and use default. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use default TTL.\nOnly one of 'retention_window_days' and 'retention_strategy' may be set."]
    pub fn retention_window_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_window_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audio_export_settings` after provisioning.\n"]
    pub fn audio_export_settings(&self) -> terrars::ListRef<DialogflowCxSecuritySettingsAudioExportSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audio_export_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `insights_export_settings` after provisioning.\n"]
    pub fn insights_export_settings(&self) -> terrars::ListRef<DialogflowCxSecuritySettingsInsightsExportSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.insights_export_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DialogflowCxSecuritySettingsTimeoutsElRef {
        DialogflowCxSecuritySettingsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DialogflowCxSecuritySettingsAudioExportSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_export_pattern: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audio_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_audio_redaction: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gcs_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxSecuritySettingsAudioExportSettingsEl {
    #[doc = "Set the field `audio_export_pattern`.\nFilename pattern for exported audio."]
    pub fn set_audio_export_pattern(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.audio_export_pattern = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `audio_format`.\nFile format for exported audio file. Currently only in telephony recordings.\n* MULAW: G.711 mu-law PCM with 8kHz sample rate.\n* MP3: MP3 file format.\n* OGG: OGG Vorbis. Possible values: [\"MULAW\", \"MP3\", \"OGG\"]"]
    pub fn set_audio_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.audio_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_audio_redaction`.\nEnable audio redaction if it is true."]
    pub fn set_enable_audio_redaction(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_audio_redaction = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `gcs_bucket`.\nCloud Storage bucket to export audio record to. Setting this field would grant the Storage Object Creator role to the Dialogflow Service Agent. API caller that tries to modify this field should have the permission of storage.buckets.setIamPolicy."]
    pub fn set_gcs_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gcs_bucket = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DialogflowCxSecuritySettingsAudioExportSettingsEl {
    type O = terrars::BlockAssignable<DialogflowCxSecuritySettingsAudioExportSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxSecuritySettingsAudioExportSettingsEl {}

impl BuildDialogflowCxSecuritySettingsAudioExportSettingsEl {
    pub fn build(self) -> DialogflowCxSecuritySettingsAudioExportSettingsEl {
        DialogflowCxSecuritySettingsAudioExportSettingsEl {
            audio_export_pattern: core::default::Default::default(),
            audio_format: core::default::Default::default(),
            enable_audio_redaction: core::default::Default::default(),
            gcs_bucket: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxSecuritySettingsAudioExportSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxSecuritySettingsAudioExportSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxSecuritySettingsAudioExportSettingsElRef {
        DialogflowCxSecuritySettingsAudioExportSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxSecuritySettingsAudioExportSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `audio_export_pattern` after provisioning.\nFilename pattern for exported audio."]
    pub fn audio_export_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_export_pattern", self.base))
    }

    #[doc =
        "Get a reference to the value of field `audio_format` after provisioning.\nFile format for exported audio file. Currently only in telephony recordings.\n* MULAW: G.711 mu-law PCM with 8kHz sample rate.\n* MP3: MP3 file format.\n* OGG: OGG Vorbis. Possible values: [\"MULAW\", \"MP3\", \"OGG\"]"]
    pub fn audio_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audio_format", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enable_audio_redaction` after provisioning.\nEnable audio redaction if it is true."]
    pub fn enable_audio_redaction(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_audio_redaction", self.base))
    }

    #[doc =
        "Get a reference to the value of field `gcs_bucket` after provisioning.\nCloud Storage bucket to export audio record to. Setting this field would grant the Storage Object Creator role to the Dialogflow Service Agent. API caller that tries to modify this field should have the permission of storage.buckets.setIamPolicy."]
    pub fn gcs_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gcs_bucket", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxSecuritySettingsInsightsExportSettingsEl {
    enable_insights_export: terrars::PrimField<bool>,
}

impl DialogflowCxSecuritySettingsInsightsExportSettingsEl { }

impl terrars::ToListMappable for DialogflowCxSecuritySettingsInsightsExportSettingsEl {
    type O = terrars::BlockAssignable<DialogflowCxSecuritySettingsInsightsExportSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxSecuritySettingsInsightsExportSettingsEl {
    #[doc = "If enabled, we will automatically exports conversations to Insights and Insights runs its analyzers."]
    pub enable_insights_export: terrars::PrimField<bool>,
}

impl BuildDialogflowCxSecuritySettingsInsightsExportSettingsEl {
    pub fn build(self) -> DialogflowCxSecuritySettingsInsightsExportSettingsEl {
        DialogflowCxSecuritySettingsInsightsExportSettingsEl { enable_insights_export: self.enable_insights_export }
    }
}

pub struct DialogflowCxSecuritySettingsInsightsExportSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxSecuritySettingsInsightsExportSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DialogflowCxSecuritySettingsInsightsExportSettingsElRef {
        DialogflowCxSecuritySettingsInsightsExportSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxSecuritySettingsInsightsExportSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `enable_insights_export` after provisioning.\nIf enabled, we will automatically exports conversations to Insights and Insights runs its analyzers."]
    pub fn enable_insights_export(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_insights_export", self.base))
    }
}

#[derive(Serialize)]
pub struct DialogflowCxSecuritySettingsTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DialogflowCxSecuritySettingsTimeoutsEl {
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

impl terrars::ToListMappable for DialogflowCxSecuritySettingsTimeoutsEl {
    type O = terrars::BlockAssignable<DialogflowCxSecuritySettingsTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDialogflowCxSecuritySettingsTimeoutsEl {}

impl BuildDialogflowCxSecuritySettingsTimeoutsEl {
    pub fn build(self) -> DialogflowCxSecuritySettingsTimeoutsEl {
        DialogflowCxSecuritySettingsTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct DialogflowCxSecuritySettingsTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DialogflowCxSecuritySettingsTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DialogflowCxSecuritySettingsTimeoutsElRef {
        DialogflowCxSecuritySettingsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DialogflowCxSecuritySettingsTimeoutsElRef {
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
struct DialogflowCxSecuritySettingsDynamic {
    audio_export_settings: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxSecuritySettingsAudioExportSettingsEl>>,
    insights_export_settings: std
    ::option
    ::Option<terrars::DynamicBlock<DialogflowCxSecuritySettingsInsightsExportSettingsEl>>,
}
