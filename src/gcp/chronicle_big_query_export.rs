#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderGoogle;

#[derive(Serialize)]
struct ChronicleBigQueryExportData_ {
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
    big_query_export_package: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance: terrars::PrimField<std::string::String>,
    location: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    project: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_graph_settings: std::option::Option<std::vec::Vec<ChronicleBigQueryExport_EntityGraphSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ioc_matches_settings: std::option::Option<std::vec::Vec<ChronicleBigQueryExport_IocMatchesSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_detections_settings: std::option::Option<std::vec::Vec<ChronicleBigQueryExport_RuleDetectionsSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ChronicleBigQueryExport_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udm_events_aggregates_settings: std
    ::option
    ::Option<std::vec::Vec<ChronicleBigQueryExport_UdmEventsAggregatesSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udm_events_settings: std::option::Option<std::vec::Vec<ChronicleBigQueryExport_UdmEventsSettings>>,
    dynamic: ChronicleBigQueryExportDynamic,
}

struct ChronicleBigQueryExport_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChronicleBigQueryExportData_>,
}

#[derive(Clone)]
pub struct ChronicleBigQueryExport(std::rc::Rc<ChronicleBigQueryExport_>);

impl ChronicleBigQueryExport {
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
        "Set the field `big_query_export_package`.\nThe BigQueryExportPackage entitled for the Chronicle instance. Possible values: [\"BIG_QUERY_EXPORT_PACKAGE_BYOBQ\", \"BIG_QUERY_EXPORT_PACKAGE_ADVANCED\"]"]
    pub fn set_big_query_export_package(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().big_query_export_package = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `entity_graph_settings`.\n"]
    pub fn set_entity_graph_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleBigQueryExport_EntityGraphSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().entity_graph_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.entity_graph_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ioc_matches_settings`.\n"]
    pub fn set_ioc_matches_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleBigQueryExport_IocMatchesSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ioc_matches_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ioc_matches_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_detections_settings`.\n"]
    pub fn set_rule_detections_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleBigQueryExport_RuleDetectionsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_detections_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_detections_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<ChronicleBigQueryExport_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `udm_events_aggregates_settings`.\n"]
    pub fn set_udm_events_aggregates_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleBigQueryExport_UdmEventsAggregatesSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().udm_events_aggregates_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.udm_events_aggregates_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `udm_events_settings`.\n"]
    pub fn set_udm_events_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChronicleBigQueryExport_UdmEventsSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().udm_events_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.udm_events_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `big_query_export_package` after provisioning.\nThe BigQueryExportPackage entitled for the Chronicle instance. Possible values: [\"BIG_QUERY_EXPORT_PACKAGE_BYOBQ\", \"BIG_QUERY_EXPORT_PACKAGE_ADVANCED\"]"]
    pub fn big_query_export_package(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.big_query_export_package", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the BigQueryExport.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/bigQueryExport"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioned` after provisioning.\nWhether the BigQueryExport has been provisioned for the Chronicle instance."]
    pub fn provisioned(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entity_graph_settings` after provisioning.\n"]
    pub fn entity_graph_settings(&self) -> terrars::ListRef<ChronicleBigQueryExport_EntityGraphSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_graph_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ioc_matches_settings` after provisioning.\n"]
    pub fn ioc_matches_settings(&self) -> terrars::ListRef<ChronicleBigQueryExport_IocMatchesSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ioc_matches_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_detections_settings` after provisioning.\n"]
    pub fn rule_detections_settings(&self) -> terrars::ListRef<ChronicleBigQueryExport_RuleDetectionsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_detections_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleBigQueryExport_TimeoutsRef {
        ChronicleBigQueryExport_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `udm_events_aggregates_settings` after provisioning.\n"]
    pub fn udm_events_aggregates_settings(
        &self,
    ) -> terrars::ListRef<ChronicleBigQueryExport_UdmEventsAggregatesSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.udm_events_aggregates_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `udm_events_settings` after provisioning.\n"]
    pub fn udm_events_settings(&self) -> terrars::ListRef<ChronicleBigQueryExport_UdmEventsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.udm_events_settings", self.extract_ref()))
    }
}

impl terrars::Referable for ChronicleBigQueryExport {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChronicleBigQueryExport { }

impl terrars::ToListMappable for ChronicleBigQueryExport {
    type O = terrars::ListRef<ChronicleBigQueryExportRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChronicleBigQueryExport_ {
    fn extract_resource_type(&self) -> std::string::String {
        "google_chronicle_big_query_export".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChronicleBigQueryExport {
    pub tf_id: std::string::String,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub instance: terrars::PrimField<std::string::String>,
    #[doc =
        "Resource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122."]
    pub location: terrars::PrimField<std::string::String>,
}

impl BuildChronicleBigQueryExport {
    pub fn build(self, stack: &mut terrars::Stack) -> ChronicleBigQueryExport {
        let out = ChronicleBigQueryExport(std::rc::Rc::new(ChronicleBigQueryExport_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChronicleBigQueryExportData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                big_query_export_package: core::default::Default::default(),
                id: core::default::Default::default(),
                instance: self.instance,
                location: self.location,
                project: core::default::Default::default(),
                entity_graph_settings: core::default::Default::default(),
                ioc_matches_settings: core::default::Default::default(),
                rule_detections_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                udm_events_aggregates_settings: core::default::Default::default(),
                udm_events_settings: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChronicleBigQueryExportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleBigQueryExportRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChronicleBigQueryExportRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `big_query_export_package` after provisioning.\nThe BigQueryExportPackage entitled for the Chronicle instance. Possible values: [\"BIG_QUERY_EXPORT_PACKAGE_BYOBQ\", \"BIG_QUERY_EXPORT_PACKAGE_ADVANCED\"]"]
    pub fn big_query_export_package(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.big_query_export_package", self.extract_ref()))
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
        "Get a reference to the value of field `name` after provisioning.\nIdentifier. The resource name of the BigQueryExport.\nFormat:\nprojects/{project}/locations/{location}/instances/{instance}/bigQueryExport"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `provisioned` after provisioning.\nWhether the BigQueryExport has been provisioned for the Chronicle instance."]
    pub fn provisioned(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entity_graph_settings` after provisioning.\n"]
    pub fn entity_graph_settings(&self) -> terrars::ListRef<ChronicleBigQueryExport_EntityGraphSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.entity_graph_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ioc_matches_settings` after provisioning.\n"]
    pub fn ioc_matches_settings(&self) -> terrars::ListRef<ChronicleBigQueryExport_IocMatchesSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ioc_matches_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_detections_settings` after provisioning.\n"]
    pub fn rule_detections_settings(&self) -> terrars::ListRef<ChronicleBigQueryExport_RuleDetectionsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_detections_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ChronicleBigQueryExport_TimeoutsRef {
        ChronicleBigQueryExport_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `udm_events_aggregates_settings` after provisioning.\n"]
    pub fn udm_events_aggregates_settings(
        &self,
    ) -> terrars::ListRef<ChronicleBigQueryExport_UdmEventsAggregatesSettingsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.udm_events_aggregates_settings", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `udm_events_settings` after provisioning.\n"]
    pub fn udm_events_settings(&self) -> terrars::ListRef<ChronicleBigQueryExport_UdmEventsSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.udm_events_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChronicleBigQueryExportDynamic {
    entity_graph_settings: std::option::Option<terrars::DynamicBlock<ChronicleBigQueryExport_EntityGraphSettings>>,
    ioc_matches_settings: std::option::Option<terrars::DynamicBlock<ChronicleBigQueryExport_IocMatchesSettings>>,
    rule_detections_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleBigQueryExport_RuleDetectionsSettings>>,
    udm_events_aggregates_settings: std
    ::option
    ::Option<terrars::DynamicBlock<ChronicleBigQueryExport_UdmEventsAggregatesSettings>>,
    udm_events_settings: std::option::Option<terrars::DynamicBlock<ChronicleBigQueryExport_UdmEventsSettings>>,
}

#[derive(Serialize)]
pub struct ChronicleBigQueryExport_EntityGraphSettings {
    enabled: terrars::PrimField<bool>,
    retention_days: terrars::PrimField<f64>,
}

impl ChronicleBigQueryExport_EntityGraphSettings { }

impl terrars::ToListMappable for ChronicleBigQueryExport_EntityGraphSettings {
    type O = terrars::BlockAssignable<ChronicleBigQueryExport_EntityGraphSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleBigQueryExport_EntityGraphSettings {
    #[doc = "Whether the data source is enabled for export."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The retention period for the data source in days."]
    pub retention_days: terrars::PrimField<f64>,
}

impl BuildChronicleBigQueryExport_EntityGraphSettings {
    pub fn build(self) -> ChronicleBigQueryExport_EntityGraphSettings {
        ChronicleBigQueryExport_EntityGraphSettings {
            enabled: self.enabled,
            retention_days: self.retention_days,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleBigQueryExport_EntityGraphSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleBigQueryExport_EntityGraphSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleBigQueryExport_EntityGraphSettingsRef {
        ChronicleBigQueryExport_EntityGraphSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleBigQueryExport_EntityGraphSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_freshness_time` after provisioning.\nThe data freshness of the given export which represents the time bucket at\nwhich the latest event was exported."]
    pub fn data_freshness_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_freshness_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_volume` after provisioning.\nThe stored data volume of all the exports."]
    pub fn data_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_volume", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the data source is enabled for export."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `latest_export_job_state` after provisioning.\nThe latest export job state."]
    pub fn latest_export_job_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_export_job_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_days` after provisioning.\nThe retention period for the data source in days."]
    pub fn retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_days", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleBigQueryExport_IocMatchesSettings {
    enabled: terrars::PrimField<bool>,
    retention_days: terrars::PrimField<f64>,
}

impl ChronicleBigQueryExport_IocMatchesSettings { }

impl terrars::ToListMappable for ChronicleBigQueryExport_IocMatchesSettings {
    type O = terrars::BlockAssignable<ChronicleBigQueryExport_IocMatchesSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleBigQueryExport_IocMatchesSettings {
    #[doc = "Whether the data source is enabled for export."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The retention period for the data source in days."]
    pub retention_days: terrars::PrimField<f64>,
}

impl BuildChronicleBigQueryExport_IocMatchesSettings {
    pub fn build(self) -> ChronicleBigQueryExport_IocMatchesSettings {
        ChronicleBigQueryExport_IocMatchesSettings {
            enabled: self.enabled,
            retention_days: self.retention_days,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleBigQueryExport_IocMatchesSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleBigQueryExport_IocMatchesSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleBigQueryExport_IocMatchesSettingsRef {
        ChronicleBigQueryExport_IocMatchesSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleBigQueryExport_IocMatchesSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_freshness_time` after provisioning.\nThe data freshness of the given export which represents the time bucket at\nwhich the latest event was exported."]
    pub fn data_freshness_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_freshness_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_volume` after provisioning.\nThe stored data volume of all the exports."]
    pub fn data_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_volume", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the data source is enabled for export."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `latest_export_job_state` after provisioning.\nThe latest export job state."]
    pub fn latest_export_job_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_export_job_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_days` after provisioning.\nThe retention period for the data source in days."]
    pub fn retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_days", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleBigQueryExport_RuleDetectionsSettings {
    enabled: terrars::PrimField<bool>,
    retention_days: terrars::PrimField<f64>,
}

impl ChronicleBigQueryExport_RuleDetectionsSettings { }

impl terrars::ToListMappable for ChronicleBigQueryExport_RuleDetectionsSettings {
    type O = terrars::BlockAssignable<ChronicleBigQueryExport_RuleDetectionsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleBigQueryExport_RuleDetectionsSettings {
    #[doc = "Whether the data source is enabled for export."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The retention period for the data source in days."]
    pub retention_days: terrars::PrimField<f64>,
}

impl BuildChronicleBigQueryExport_RuleDetectionsSettings {
    pub fn build(self) -> ChronicleBigQueryExport_RuleDetectionsSettings {
        ChronicleBigQueryExport_RuleDetectionsSettings {
            enabled: self.enabled,
            retention_days: self.retention_days,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleBigQueryExport_RuleDetectionsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleBigQueryExport_RuleDetectionsSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleBigQueryExport_RuleDetectionsSettingsRef {
        ChronicleBigQueryExport_RuleDetectionsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleBigQueryExport_RuleDetectionsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_freshness_time` after provisioning.\nThe data freshness of the given export which represents the time bucket at\nwhich the latest event was exported."]
    pub fn data_freshness_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_freshness_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_volume` after provisioning.\nThe stored data volume of all the exports."]
    pub fn data_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_volume", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the data source is enabled for export."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `latest_export_job_state` after provisioning.\nThe latest export job state."]
    pub fn latest_export_job_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_export_job_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_days` after provisioning.\nThe retention period for the data source in days."]
    pub fn retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_days", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleBigQueryExport_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChronicleBigQueryExport_Timeouts {
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

impl terrars::ToListMappable for ChronicleBigQueryExport_Timeouts {
    type O = terrars::BlockAssignable<ChronicleBigQueryExport_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleBigQueryExport_Timeouts {}

impl BuildChronicleBigQueryExport_Timeouts {
    pub fn build(self) -> ChronicleBigQueryExport_Timeouts {
        ChronicleBigQueryExport_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleBigQueryExport_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleBigQueryExport_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleBigQueryExport_TimeoutsRef {
        ChronicleBigQueryExport_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleBigQueryExport_TimeoutsRef {
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
pub struct ChronicleBigQueryExport_UdmEventsAggregatesSettings {
    enabled: terrars::PrimField<bool>,
    retention_days: terrars::PrimField<f64>,
}

impl ChronicleBigQueryExport_UdmEventsAggregatesSettings { }

impl terrars::ToListMappable for ChronicleBigQueryExport_UdmEventsAggregatesSettings {
    type O = terrars::BlockAssignable<ChronicleBigQueryExport_UdmEventsAggregatesSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleBigQueryExport_UdmEventsAggregatesSettings {
    #[doc = "Whether the data source is enabled for export."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The retention period for the data source in days."]
    pub retention_days: terrars::PrimField<f64>,
}

impl BuildChronicleBigQueryExport_UdmEventsAggregatesSettings {
    pub fn build(self) -> ChronicleBigQueryExport_UdmEventsAggregatesSettings {
        ChronicleBigQueryExport_UdmEventsAggregatesSettings {
            enabled: self.enabled,
            retention_days: self.retention_days,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleBigQueryExport_UdmEventsAggregatesSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleBigQueryExport_UdmEventsAggregatesSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ChronicleBigQueryExport_UdmEventsAggregatesSettingsRef {
        ChronicleBigQueryExport_UdmEventsAggregatesSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleBigQueryExport_UdmEventsAggregatesSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_freshness_time` after provisioning.\nThe data freshness of the given export which represents the time bucket at\nwhich the latest event was exported."]
    pub fn data_freshness_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_freshness_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_volume` after provisioning.\nThe stored data volume of all the exports."]
    pub fn data_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_volume", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the data source is enabled for export."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `latest_export_job_state` after provisioning.\nThe latest export job state."]
    pub fn latest_export_job_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_export_job_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_days` after provisioning.\nThe retention period for the data source in days."]
    pub fn retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_days", self.base))
    }
}

#[derive(Serialize)]
pub struct ChronicleBigQueryExport_UdmEventsSettings {
    enabled: terrars::PrimField<bool>,
    retention_days: terrars::PrimField<f64>,
}

impl ChronicleBigQueryExport_UdmEventsSettings { }

impl terrars::ToListMappable for ChronicleBigQueryExport_UdmEventsSettings {
    type O = terrars::BlockAssignable<ChronicleBigQueryExport_UdmEventsSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChronicleBigQueryExport_UdmEventsSettings {
    #[doc = "Whether the data source is enabled for export."]
    pub enabled: terrars::PrimField<bool>,
    #[doc = "The retention period for the data source in days."]
    pub retention_days: terrars::PrimField<f64>,
}

impl BuildChronicleBigQueryExport_UdmEventsSettings {
    pub fn build(self) -> ChronicleBigQueryExport_UdmEventsSettings {
        ChronicleBigQueryExport_UdmEventsSettings {
            enabled: self.enabled,
            retention_days: self.retention_days,
        }
    }
}

#[allow(dead_code)]
pub struct ChronicleBigQueryExport_UdmEventsSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChronicleBigQueryExport_UdmEventsSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChronicleBigQueryExport_UdmEventsSettingsRef {
        ChronicleBigQueryExport_UdmEventsSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChronicleBigQueryExport_UdmEventsSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `data_freshness_time` after provisioning.\nThe data freshness of the given export which represents the time bucket at\nwhich the latest event was exported."]
    pub fn data_freshness_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_freshness_time", self.base))
    }

    #[doc =
        "Get a reference to the value of field `data_volume` after provisioning.\nThe stored data volume of all the exports."]
    pub fn data_volume(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_volume", self.base))
    }

    #[doc =
        "Get a reference to the value of field `enabled` after provisioning.\nWhether the data source is enabled for export."]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `latest_export_job_state` after provisioning.\nThe latest export job state."]
    pub fn latest_export_job_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_export_job_state", self.base))
    }

    #[doc =
        "Get a reference to the value of field `retention_days` after provisioning.\nThe retention period for the data source in days."]
    pub fn retention_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_days", self.base))
    }
}
